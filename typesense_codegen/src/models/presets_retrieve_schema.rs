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
pub struct PresetsRetrieveSchema {
    #[serde(rename = "presets")]
    pub presets: Vec<models::PresetSchema>,
}

impl PresetsRetrieveSchema {
    pub fn new(presets: Vec<models::PresetSchema>) -> PresetsRetrieveSchema {
        PresetsRetrieveSchema {
            presets,
        }
    }
}

