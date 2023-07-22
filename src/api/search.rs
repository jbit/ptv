use crate::*;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

/// Response from [PTV::search]
///
/// Swagger type: `V3.SearchResult`
#[derive(Clone, Debug, Deserialize)]
pub struct SearchResponse {
    /// Train stations, tram stops, bus stops, regional coach stops or Night Bus stops
    pub stops: Vec<StopDetails>,
    /// Train lines, tram routes, bus routes, regional coach routes, Night Bus routes
    pub routes: Vec<RouteDetails>,
    /// myki ticket outlets
    pub outlets: Vec<OutletDetails>,
    /// API status / Metadata
    pub status: Status,
}

/// Parameters for [PTV::search]
#[derive(Clone, Debug, Default, Serialize)]
pub struct SearchParams {
    /// Filter by route_type; values returned via RouteTypes API (note: stops and routes are ordered by route_types specified)
    pub route_types: Option<Vec<RouteType>>,
    /// Filter by geographic coordinate of latitude
    pub latitude: Option<f32>,
    /// Filter by geographic coordinate of longitude
    pub longitude: Option<f32>,
    /// Filter by maximum distance (in metres) from location specified via latitude and longitude parameters
    pub max_distance: Option<f32>,
    /// Placeholder for future development; currently unavailable
    pub include_addresses: Option<bool>,
    /// Indicates if outlets will be returned in response (default = true)
    pub include_outlets: Option<bool>,
    /// Indicates whether to find stops by suburbs in the search term (default = true)
    pub match_stop_by_suburb: Option<bool>,
    /// Indicates whether to find routes by suburbs in the search term (default = true)
    pub match_route_by_suburb: Option<bool>,
    /// Indicates whether to search for stops according to a metlink stop ID (default = false)
    pub match_stop_by_gtfs_stop_id: Option<bool>,
}

impl<HTTPClient: PTVHttpClient> PTV<HTTPClient> {
    /// View stops, routes and myki ticket outlets that match the search term
    ///
    /// Swagger operation: `Search_Search`
    pub async fn search(&self, search_term: &str, params: SearchParams) -> Result<SearchResponse> {
        // For some reason special characters need to be encoded twice
        // otherwise the following message is returned:
        // Forbidden (403): Supplied signature is invalid for request.
        let search_term = utf8_percent_encode(search_term, NON_ALPHANUMERIC).to_string();
        let search_term = utf8_percent_encode(&search_term, NON_ALPHANUMERIC);
        let path = format!("/v3/search/{search_term}");
        let url = self.build_url(&path, &params);
        self.http_client.api_get(url).await
    }
}
