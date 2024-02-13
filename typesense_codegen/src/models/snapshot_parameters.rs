/*
 * Typesense API
 *
 * An open source search engine for building delightful search experiences.
 *
 * The version of the OpenAPI document: 0.25.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SnapshotParameters {
    #[serde(rename = "snapshot_path", skip_serializing_if = "Option::is_none")]
    pub snapshot_path: Option<String>,
}

impl SnapshotParameters {
    pub fn new() -> SnapshotParameters {
        SnapshotParameters {
            snapshot_path: None,
        }
    }
}
