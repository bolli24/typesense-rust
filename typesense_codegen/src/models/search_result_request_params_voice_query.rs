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
pub struct SearchResultRequestParamsVoiceQuery {
    #[serde(rename = "transcribed_query", skip_serializing_if = "Option::is_none")]
    pub transcribed_query: Option<String>,
}

impl SearchResultRequestParamsVoiceQuery {
    pub fn new() -> SearchResultRequestParamsVoiceQuery {
        SearchResultRequestParamsVoiceQuery {
            transcribed_query: None,
        }
    }
}

