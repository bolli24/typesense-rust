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
pub struct CollectionUpdateSchema {
    /// A list of fields for querying, filtering and faceting
    #[serde(rename = "fields")]
    pub fields: Vec<models::Field>,
}

impl CollectionUpdateSchema {
    pub fn new(fields: Vec<models::Field>) -> CollectionUpdateSchema {
        CollectionUpdateSchema {
            fields,
        }
    }
}

