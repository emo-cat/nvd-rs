use std::fs::File;
use std::io::BufReader;
use std::ops::DerefMut;
use std::path::PathBuf;

use chrono::{Duration, Utc};
use diesel::r2d2::ConnectionManager;
use diesel::{r2d2, MysqlConnection};
use nvd_api::pagination::Object;
use nvd_api::v2::vulnerabilities::CveParameters;
use nvd_api::v2::LastModDate;
use nvd_api::ApiVersion;

pub use cli::{CPECommand, CVECommand, NVDHelper, TopLevel};
pub use import_cpe::{create_cve_product, create_product, create_vendor};
pub use import_cve::{import_from_api, import_from_archive};
pub use import_cwe::import_cwe;
use nvd_cpe::dictionary::CPEList;
use nvd_cves::v4::CVEContainer;

mod cli;
mod import_cpe;
mod import_cve;
mod import_cwe;

pub type Connection = MysqlConnection;

pub type Pool = r2d2::Pool<ConnectionManager<Connection>>;

pub fn init_db_pool() -> Pool {
  let database_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let manager = ConnectionManager::<Connection>::new(database_url);
  Pool::builder()
    .build(manager)
    .expect("Failed to create pool.")
}

async fn async_cve(param: CveParameters) {
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

fn with_archive_cve(path: PathBuf) {
  let connection_pool = init_db_pool();
  let gz_open_file = File::open(path).unwrap();
  let gz_decoder = flate2::read::GzDecoder::new(gz_open_file);
  let file = BufReader::new(gz_decoder);
  let c: CVEContainer = serde_json::from_reader(file).unwrap();
  for w in c.CVE_Items {
    import_from_archive(connection_pool.get().unwrap().deref_mut(), w).unwrap_or_default();
  }
}

pub async fn cve_mode(config: CVECommand) {
  if let Some(p) = config.path {
    with_archive_cve(p)
  }
  if config.api || config.id.is_some() {
    let mut param = CveParameters {
      cve_id: config.id,
      ..CveParameters::default()
    };
    if let Some(hours) = config.hours {
      let now = Utc::now();
      // 每两个小时拉取三小时内的更新数据入库
      let three_hours = now - Duration::hours(hours);
      param = CveParameters {
        last_mod: Some(LastModDate {
          last_mod_start_date: three_hours.to_rfc3339(),
          last_mod_end_date: now.to_rfc3339(),
        }),
        ..param
      };
    }
    async_cve(param).await
  }
}

pub async fn cpe_mode(config: CPECommand) {
  if let Some(path) = config.path {
    with_archive_cpe(path)
  }
}

fn with_archive_cpe(path: PathBuf) {
  let gz_open_file = File::open(path).unwrap();
  let gz_decoder = flate2::read::GzDecoder::new(gz_open_file);
  let file = BufReader::new(gz_decoder);
  let c: CPEList = quick_xml::de::from_reader(file).unwrap();
  let mut current = None;
  let mut all_references = vec![];
  let mut all_titles = vec![];
  for cpe_item in c.cpe_item.into_iter() {
    let product = nvd_cpe::Product::from(&cpe_item.cpe23_item.name);
    if cpe_item.deprecated {
      continue;
    }
    if current == Some(product.clone()) {
      if let Some(references) = cpe_item.references {
        all_references.extend(references.reference);
      }
      all_titles.extend(cpe_item.title)
    } else if current.is_none() {
      current = Some(product.clone());
      if let Some(references) = cpe_item.references {
        all_references.extend(references.reference);
      }
      all_titles.extend(cpe_item.title)
    } else {
      current = Some(product.clone());
      let count = all_references.len();
      println!("{:?}", all_references);
      println!("{:?}", all_titles);
      if count > 2 {
        break;
      }
      all_references = vec![];
      all_titles = vec![];
    }
  }
}

fn get_title(titles: Vec<nvd_cpe::dictionary::Title>) {
  // TextDiff::from_lines()

  // similar::
}
#[cfg(test)]
mod tests {
  use super::*;
  use std::str::FromStr;

  #[test]
  fn it_works() {
    let v1 = "@thi.ng/egf Project @thi.ng/egf 0.2.2 for Node.js";
    let v2 = "@thi.ng/egf Project @thi.ng/egf 0.3.4 for Node.js";
    let v1s: Vec<&str> = v1.split_ascii_whitespace().collect();
    let v2s: Vec<&str> = v2.split_ascii_whitespace().collect();
    // let diffs = similar::capture_diff_slices(similar::Algorithm::Myers, &v1s, &v2s);
    let diffs = similar::TextDiff::from_slices(&v1s, &v2s);
    println!("{}", diffs.ratio());
    for diff in diffs.iter_all_changes() {
      // println!("{:?}", diff.tag());
      if diff.tag() != similar::ChangeTag::Equal {
        println!("{}", diff);
      }
    }
    // assert_eq!(result, 4);
  }
}
