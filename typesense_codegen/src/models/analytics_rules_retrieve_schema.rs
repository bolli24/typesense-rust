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
pub struct AnalyticsRulesRetrieveSchema {
    #[serde(rename = "rules", skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<models::AnalyticsRuleSchema>>,
}

impl AnalyticsRulesRetrieveSchema {
    pub fn new() -> AnalyticsRulesRetrieveSchema {
        AnalyticsRulesRetrieveSchema {
            rules: None,
        }
    }
}

