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
pub struct FieldEmbed {
    #[serde(rename = "from")]
    pub from: Vec<String>,
    #[serde(rename = "model_config")]
    pub model_config: Box<models::FieldEmbedModelConfig>,
}

impl FieldEmbed {
    pub fn new(from: Vec<String>, model_config: models::FieldEmbedModelConfig) -> FieldEmbed {
        FieldEmbed {
            from,
            model_config: Box::new(model_config),
        }
    }
}

