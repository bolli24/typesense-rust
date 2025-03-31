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
pub struct AnalyticsRuleDeleteResponse {
    #[serde(rename = "name")]
    pub name: String,
}

impl AnalyticsRuleDeleteResponse {
    pub fn new(name: String) -> AnalyticsRuleDeleteResponse {
        AnalyticsRuleDeleteResponse {
            name,
        }
    }
}

