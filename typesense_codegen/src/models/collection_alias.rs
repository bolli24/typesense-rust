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
pub struct CollectionAlias {
    /// Name of the collection alias
    #[serde(rename = "name")]
    pub name: String,
    /// Name of the collection the alias mapped to
    #[serde(rename = "collection_name")]
    pub collection_name: String,
}

impl CollectionAlias {
    pub fn new(name: String, collection_name: String) -> CollectionAlias {
        CollectionAlias {
            name,
            collection_name,
        }
    }
}

