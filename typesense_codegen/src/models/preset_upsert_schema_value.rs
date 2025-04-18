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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PresetUpsertSchemaValue {
    SearchParameters(Box<models::SearchParameters>),
    MultiSearchSearchesParameter(Box<models::MultiSearchSearchesParameter>),
}

impl Default for PresetUpsertSchemaValue {
    fn default() -> Self {
        Self::SearchParameters(Default::default())
    }
}

