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
pub struct AnalyticsRuleParametersSourceEventsInner {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "weight")]
    pub weight: f32,
    #[serde(rename = "name")]
    pub name: String,
}

impl AnalyticsRuleParametersSourceEventsInner {
    pub fn new(r#type: String, weight: f32, name: String) -> AnalyticsRuleParametersSourceEventsInner {
        AnalyticsRuleParametersSourceEventsInner {
            r#type,
            weight,
            name,
        }
    }
}

