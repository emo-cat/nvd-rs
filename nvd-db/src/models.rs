use diesel::prelude::*;
use crate::schema::*;
use chrono::NaiveDateTime;
use serde_json::Value;

#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(Cve))]
#[diesel(belongs_to(Product))]
#[diesel(table_name = cve_product)]
#[diesel(primary_key(cve_id, product_id))]
pub struct CveProduct {
  pub cve_id: String,
  pub product_id: Vec<u8>,
}

#[derive(Queryable, Debug, Clone)]
pub struct Cve {
  pub id: String,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
  pub references: Value,
  pub description: Value,
  pub cwe: Value,
  pub cvss3_id: Option<Vec<u8>>,
  pub cvss2_id: Option<Vec<u8>>,
  pub raw: Value,
  pub assigner: String,
  pub configurations: Value,
  pub official: u8,
}

#[derive(Queryable, Debug, Clone)]
pub struct Cvss2 {
  pub id: Vec<u8>,
  pub version: String,
  pub vector_string: String,
  pub access_vector: String,
  pub access_complexity: String,
  pub authentication: String,
  pub confidentiality_impact: String,
  pub integrity_impact: String,
  pub availability_impact: String,
  pub base_score: f32,
  pub exploitability_score: f32,
  pub impact_score: f32,
  pub severity: String,
  pub ac_insuf_info: Option<String>,
  pub obtain_all_privilege: i8,
  pub obtain_user_privilege: i8,
  pub obtain_other_privilege: i8,
  pub user_interaction_required: Option<i8>,
}

#[derive(Queryable, Debug, Clone)]
pub struct Cvss3 {
  pub id: Vec<u8>,
  pub version: String,
  pub vector_string: String,
  pub attack_vector: String,
  pub attack_complexity: String,
  pub privileges_required: String,
  pub user_interaction: String,
  pub scope: String,
  pub confidentiality_impact: String,
  pub integrity_impact: String,
  pub availability_impact: String,
  pub base_score: f32,
  pub base_severity: String,
  pub exploitability_score: f32,
  pub impact_score: f32,
}

#[derive(Queryable, Debug, Clone)]
#[diesel(belongs_to(Vendor))]
pub struct Product {
  pub id: Vec<u8>,
  pub vendor_id: Vec<u8>,
  pub name: String,
  pub description: Option<String>,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
  pub homepage: Option<String>,
  pub official: u8,
  pub part: String,
}

#[derive(Queryable, Debug, Clone)]
pub struct Vendor {
  pub id: Vec<u8>,
  pub name: String,
  pub description: Option<String>,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
  pub homepage: Option<String>,
  pub official: u8,
}

#[derive(Queryable, Debug)]
pub struct Cwe {
  pub id: i32,
  pub name: String,
  pub description: String,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}
