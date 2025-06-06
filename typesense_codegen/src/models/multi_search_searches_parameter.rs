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
pub struct MultiSearchSearchesParameter {
    /// When true, merges the search results from each search query into a single ordered set of hits.
    #[serde(rename = "union", skip_serializing_if = "Option::is_none")]
    pub union: Option<bool>,
    #[serde(rename = "searches")]
    pub searches: Vec<models::MultiSearchCollectionParameters>,
}

impl MultiSearchSearchesParameter {
    pub fn new(searches: Vec<models::MultiSearchCollectionParameters>) -> MultiSearchSearchesParameter {
        MultiSearchSearchesParameter {
            union: None,
            searches,
        }
    }
}

