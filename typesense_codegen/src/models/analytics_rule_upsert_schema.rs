/*
 * Typesense API
 *
 * An open source search engine for building delightful search experiences.
 *
 * The version of the OpenAPI document: 28.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnalyticsRuleUpsertSchema {
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "params")]
    pub params: Box<models::AnalyticsRuleParameters>,
}

impl AnalyticsRuleUpsertSchema {
    pub fn new(r#type: Type, params: models::AnalyticsRuleParameters) -> AnalyticsRuleUpsertSchema {
        AnalyticsRuleUpsertSchema {
            r#type,
            params: Box::new(params),
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "popular_queries")]
    PopularQueries,
    #[serde(rename = "nohits_queries")]
    NohitsQueries,
    #[serde(rename = "counter")]
    Counter,
}

impl Default for Type {
    fn default() -> Type {
        Self::PopularQueries
    }
}

