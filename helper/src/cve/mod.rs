use chrono::{NaiveDate, Utc};
use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::ops::DerefMut;
use std::path::PathBuf;
use std::str::FromStr;

use cnvd::cnnvd::{CNNVDData, VulListParametersBuilder};
use diesel::mysql::MysqlConnection;
use nvd_api::pagination::Object;
use nvd_api::v2::vulnerabilities::CveParameters;
use nvd_api::ApiVersion;

use nvd_cves::v4::{CVEContainer, CVEItem};
use nvd_model::cve::{CreateCve, Cve};
use nvd_model::error::DBResult;
use nvd_model::types::AnyValue;

use crate::cpe::{del_expire_product, import_vendor_product_to_db};
use crate::kb::associate_cve_and_exploit;
use crate::{create_cve_product, init_db_pool};

pub(crate) async fn async_cve(param: CveParameters) {
  let connection_pool = init_db_pool();
  let api = nvd_api::NVDApi::new(None, ApiVersion::default()).unwrap();
  let resp = api.cve(param).await.unwrap();
  if let Object::Vulnerabilities(vs) = resp.results {
    for v in vs {
      println!("正在同步：{:?} {:?}", v.cve.vuln_status, v.cve.id);
      import_from_api(connection_pool.get().unwrap().deref_mut(), v.cve).unwrap();
    }
  }
}

fn import_from_archive(connection: &mut MysqlConnection, cve_item: CVEItem) -> DBResult<String> {
  let id = cve_item.cve.meta.id;
  let y = id.split('-').nth(1).unwrap_or_default();
  let new_post = CreateCve {
    id: id.clone(),
    created_at: cve_item.published_date,
    updated_at: cve_item.last_modified_date,
    references: AnyValue::new(cve_item.cve.references.reference_data),
    description: AnyValue::new(cve_item.cve.description.description_data),
    severity: cve_item.impact.severity().to_lowercase(),
    metrics: AnyValue::new(cve_item.impact),
    assigner: cve_item.cve.meta.assigner,
    configurations: AnyValue::new(cve_item.configurations.nodes.clone()),
    year: i32::from_str(y).unwrap_or_default(),
    weaknesses: AnyValue::new(cve_item.cve.problem_type.problem_type_data),
  };
  // 插入到数据库
  match Cve::create(connection, &new_post) {
    Ok(cve_id) => {
      // 插入cpe_match关系表
      for vendor_product in cve_item
        .configurations
        .nodes
        .iter()
        .flat_map(|n| n.vendor_product())
        .collect::<HashSet<_>>()
      {
        import_vendor_product_to_db(connection, vendor_product.clone());
        create_cve_product(
          connection,
          cve_id.id.clone(),
          vendor_product.vendor,
          vendor_product.product,
        );
      }
    }
    Err(err) => {
      println!("Cve::create: {err:?}");
    }
  }
  Ok(new_post.id)
}

fn import_from_api(
  connection: &mut MysqlConnection,
  cve_item: nvd_cves::api::CVE,
) -> DBResult<String> {
  let id = cve_item.id;
  let y = id.split('-').nth(1).unwrap_or_default();
  let configurations = cve_item
    .configurations
    .iter()
    .flat_map(|n| n.nodes.clone())
    .collect::<Vec<nvd_cves::v4::configurations::Node>>();
  // println!("{:?}", configurations);
  let new_post = CreateCve {
    id: id.clone(),
    created_at: cve_item.published,
    updated_at: cve_item.last_modified,
    references: AnyValue::new(cve_item.references),
    description: AnyValue::new(cve_item.descriptions),
    severity: cve_item.metrics.severity().to_lowercase(),
    metrics: AnyValue::new(cve_item.metrics),
    assigner: cve_item.source_identifier,
    configurations: AnyValue::new(configurations.clone()),
    year: i32::from_str(y).unwrap_or_default(),
    weaknesses: AnyValue::new(cve_item.weaknesses),
  };
  // 插入或者更新到数据库
  match Cve::create_or_update(connection, &new_post) {
    Ok(cve_id) => {
      // 插入cpe_match关系表
      let vendor_products = configurations
        .iter()
        .flat_map(|n| n.vendor_product())
        .collect::<HashSet<_>>();
      let mut product_set = HashSet::new();
      for vendor_product in vendor_products {
        // 创建供应商和产品
        let product_id = import_vendor_product_to_db(connection, vendor_product.clone());
        product_set.insert(product_id);
        // CVE编号关联产品
        create_cve_product(
          connection,
          cve_id.id.clone(),
          vendor_product.vendor,
          vendor_product.product,
        );
      }
      del_expire_product(connection, &cve_id.id, product_set);
      associate_cve_and_exploit(connection, &cve_id.id);
    }
    Err(err) => {
      println!("Cve::create_or_update: {err:?}");
    }
  }
  Ok(new_post.id)
}

pub(crate) fn with_archive_cve(path: PathBuf) {
  let connection_pool = init_db_pool();
  let gz_open_file = File::open(path).unwrap();
  let gz_decoder = flate2::read::GzDecoder::new(gz_open_file);
  let file = BufReader::new(gz_decoder);
  let c: CVEContainer = serde_json::from_reader(file).unwrap();
  for w in c.CVE_Items {
    import_from_archive(connection_pool.get().unwrap().deref_mut(), w).unwrap_or_default();
  }
}

pub async fn cnnvd_api() {
  let api = cnvd::cnnvd::CNNVDApi::new().unwrap();
  let list = api
    .vul_list(
      VulListParametersBuilder::default()
        .end_time(Some(NaiveDate::from(Utc::now().naive_local())))
        .build()
        .unwrap(),
    )
    .await
    .unwrap();
  if let CNNVDData::VulList(list) = list.data {
    for v in list.records {
      if let CNNVDData::Detail(detail) = api.detail(v.into()).await.unwrap().data {
        println!("{:?}", detail);
      }
    }
  }
}
