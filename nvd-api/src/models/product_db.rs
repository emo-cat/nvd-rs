use crate::error::{NVDApiError, Result};
use crate::models::{Product, Vendor};
use crate::schema::{products, vendors};
use crate::DB;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use serde::{Deserialize, Serialize};

#[derive(Insertable)]
#[diesel(table_name = products)]
pub struct CreateProduct {
  pub id: Vec<u8>,
  pub vendor_id: Vec<u8>,
  pub official: u8,
  pub part: String,
  pub name: String,
  pub description: Option<String>,
  pub homepage: Option<String>,
}

pub struct QueryProductById {
  pub vendor_id: Vec<u8>,
  pub name: String,
}
pub struct QueryProductByVendorName {
  pub vendor_name: String,
  pub name: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductCount {
  pub result: Vec<Product>,
  pub total: i64,
}
// 产品查询参数
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryProduct {
  pub vendor_name: Option<String>,
  pub name: Option<String>,
  pub official: Option<u8>,
  pub limit: Option<i64>,
  pub offset: Option<i64>,
}

impl QueryProduct {
  fn query<'a>(
    &'a self,
    _conn: &mut MysqlConnection,
    mut query: products::BoxedQuery<'a, DB>,
  ) -> Result<products::BoxedQuery<'a, DB>> {
    if let Some(name) = &self.name {
      let name = format!("{name}%");
      query = query.filter(products::name.like(name));
    }
    if let Some(official) = &self.official {
      query = query.filter(products::official.eq(official));
    }
    Ok(query)
  }
  fn total(&self, conn: &mut MysqlConnection) -> Result<i64> {
    let query = self.query(conn, products::table.into_boxed())?;
    // 统计查询全部，分页用
    Ok(
      query
        .select(diesel::dsl::count(products::id))
        .first::<i64>(conn)?,
    )
  }
}
impl Product {
  // 创建产品
  pub fn create(conn: &mut MysqlConnection, args: &CreateProduct) -> Result<Self> {
    if let Err(err) = diesel::insert_into(products::table)
      .values(args)
      .execute(conn)
    {
      // 重复了，说明已经存在该产品
      match err {
        DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {}
        _ => {
          return Err(NVDApiError::DieselError { source: err });
        }
      }
    }
    Self::query_by_id(
      conn,
      &QueryProductById {
        vendor_id: args.vendor_id.clone(),
        name: args.name.clone(),
      },
    )
  }
  // 查询产品从提供商的id
  pub fn query_by_id(conn: &mut MysqlConnection, args: &QueryProductById) -> Result<Self> {
    Ok(
      products::dsl::products
        .filter(products::vendor_id.eq(&args.vendor_id))
        .filter(products::name.eq(&args.name))
        .first::<Product>(conn)?,
    )
  }
  // 查询产品从提供商的名称
  pub fn query_by_vendor_name(
    conn: &mut MysqlConnection,
    args: &QueryProductByVendorName,
  ) -> Result<Self> {
    let v: Vendor = vendors::table
      .filter(vendors::name.eq(&args.vendor_name))
      .first(conn)?;
    let p: Product = Product::belonging_to(&v)
      .filter(products::name.eq(&args.name))
      .first(conn)?;
    Ok(p)
  }

  pub fn query(conn: &mut MysqlConnection, args: &QueryProduct) -> Result<ProductCount> {
    let total = args.total(conn)?;
    let result = {
      let query = args.query(conn, products::table.into_boxed())?;
      query
        .offset(args.offset.unwrap_or(0))
        .limit(args.limit.map_or(20, |l| if l > 20 { 20 } else { l }))
        .order(products::name.asc())
        .load::<Product>(conn)?
    };
    Ok(ProductCount { result, total })
  }
}
