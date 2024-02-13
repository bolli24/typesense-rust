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
pub struct SearchResult {
    #[serde(rename = "facet_counts", skip_serializing_if = "Option::is_none")]
    pub facet_counts: Option<Vec<crate::models::FacetCounts>>,
    /// The number of documents found
    #[serde(rename = "found", skip_serializing_if = "Option::is_none")]
    pub found: Option<i32>,
    /// The number of milliseconds the search took
    #[serde(rename = "search_time_ms", skip_serializing_if = "Option::is_none")]
    pub search_time_ms: Option<i32>,
    /// The total number of documents in the collection
    #[serde(rename = "out_of", skip_serializing_if = "Option::is_none")]
    pub out_of: Option<i32>,
    /// Whether the search was cut off
    #[serde(rename = "search_cutoff", skip_serializing_if = "Option::is_none")]
    pub search_cutoff: Option<bool>,
    /// The search result page number
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(rename = "grouped_hits", skip_serializing_if = "Option::is_none")]
    pub grouped_hits: Option<Vec<crate::models::SearchGroupedHit>>,
    /// The documents that matched the search query
    #[serde(rename = "hits", skip_serializing_if = "Option::is_none")]
    pub hits: Option<Vec<crate::models::SearchResultHit>>,
    #[serde(rename = "request_params", skip_serializing_if = "Option::is_none")]
    pub request_params: Option<Box<crate::models::SearchResultRequestParams>>,
}

impl SearchResult {
    pub fn new() -> SearchResult {
        SearchResult {
            facet_counts: None,
            found: None,
            search_time_ms: None,
            out_of: None,
            search_cutoff: None,
            page: None,
            grouped_hits: None,
            hits: None,
            request_params: None,
        }
    }
}
