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
pub struct SearchResultRequestParams {
    #[serde(rename = "collection_name")]
    pub collection_name: String,
    #[serde(rename = "q")]
    pub q: String,
    #[serde(rename = "per_page")]
    pub per_page: i32,
    #[serde(rename = "voice_query", skip_serializing_if = "Option::is_none")]
    pub voice_query: Option<Box<models::SearchResultRequestParamsVoiceQuery>>,
}

impl SearchResultRequestParams {
    pub fn new(collection_name: String, q: String, per_page: i32) -> SearchResultRequestParams {
        SearchResultRequestParams {
            collection_name,
            q,
            per_page,
            voice_query: None,
        }
    }
}

