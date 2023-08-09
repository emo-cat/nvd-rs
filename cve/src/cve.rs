use crate::node;
use cvss::error::{CVSSError, Result};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

// 单个CVE信息
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::upper_case_acronyms)]
pub struct CVEItem {
  // CVE 信息
  pub cve: CVE,
  // 影响
  pub impact: Impact,
  // 配置
  pub configurations: Configurations,
  // 公开时间
  pub published_date: String,
  // 最后修改时间
  pub last_modified_date: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Impact {
  // TODO: Implement V1?
  // cvssV2 过期
  pub base_metric_v2: Option<ImpactMetricV2>,
  // cvssV3
  pub base_metric_v3: Option<ImpactMetricV3>,
  // TODO: Implement V4?
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ImpactMetricV2 {
  pub cvss_v2: cvss::v2::CVSS,
  // 漏洞的可利用 评分
  pub exploitability_score: f32,
  // 评分
  pub impact_score: f32,
  // 评级
  pub severity: String,
  pub ac_insuf_info: Option<bool>,
  pub obtain_all_privilege: bool,
  pub obtain_user_privilege: bool,
  pub obtain_other_privilege: bool,
  // 用户交互
  pub user_interaction_required: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ImpactMetricV3 {
  pub cvss_v3: cvss::v3::CVSS,
  // 漏洞的可利用 评分
  pub exploitability_score: f32,
  // cvss 评分
  pub impact_score: f32,
}

impl FromStr for ImpactMetricV3 {
  type Err = CVSSError;

  fn from_str(s: &str) -> Result<Self> {
    match cvss::v3::CVSS::from_str(s) {
      Ok(c) => {
        let exploitability_score = c.exploit_ability_score();
        let impact_score = c.impact_score();
        Ok(Self {
          cvss_v3: c,
          exploitability_score,
          impact_score,
        })
      }
      Err(err) => Err(err),
    }
  }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Configurations {
  // 版本
  #[serde(rename = "CVE_data_version")]
  pub data_version: String,
  // 漏洞节点
  pub nodes: Vec<node::Node>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CVE {
  // 元数据
  #[serde(rename = "CVE_data_meta")]
  pub meta: Meta,
  // 参考
  pub references: References,
  // 描述
  pub description: Description,
  // 问题类型 关联：CWE
  #[serde(rename = "problemtype")]
  pub problem_type: ProblemType,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct References {
  pub reference_data: Vec<Reference>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Reference {
  pub url: String,
  pub name: String,
  pub tags: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Info {
  #[serde(rename = "CVE_data_meta")]
  pub meta: Meta,
  pub references: References,
  pub description: Description,
  #[serde(rename = "problemtype")]
  pub problem_type: ProblemType,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProblemType {
  #[serde(rename = "problemtype_data")]
  problem_type_data: Vec<ProblemTypeDataItem>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProblemTypeDataItem {
  pub description: Vec<ProblemTypeDescription>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProblemTypeDescription {
  pub lang: String,
  pub value: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Description {
  pub description_data: Vec<DescriptionData>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DescriptionData {
  pub lang: String,
  pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Meta {
  // CVE编号
  #[serde(rename = "ID")]
  id: String,
  // 委托人
  #[serde(rename = "ASSIGNER")]
  assigner: Option<String>,
}
