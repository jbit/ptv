use crate::*;

/// Response from [PTV::departures]
///
/// Swagger type: `V3.DeparturesResponse`
#[derive(Clone, Debug, Deserialize)]
pub struct DeparturesResponse {
    /// Timetabled and real-time service departures
    pub departures: Vec<DepartureDetails>,
    /// A train station, tram stop, bus stop, regional coach stop or Night Bus stop
    pub stops: BTreeMap<StopId, StopDetails>,
    /// Train lines, tram routes, bus routes, regional coach routes, Night Bus routes
    pub routes: BTreeMap<RouteId, RouteDetails>,
    /// Individual trips/services of a route
    pub runs: BTreeMap<RunRef, RunDetails>,
    /// Directions of travel of route
    pub directions: BTreeMap<DirectionId, DirectionDetails>,
    /// Disruption information applicable to relevant routes or stops
    pub disruptions: BTreeMap<DisruptionId, DisruptionDetails>,
    /// API status / Metadata
    pub status: Status,
}

/// Values to expand the output of [PTV::departures]
#[derive(Clone, Copy, Debug, Serialize)]
pub enum ResponseExpand {
    All,
    Stop,
    Route,
    Run,
    Direction,
    Disruption,
    VehicleDescriptor,
    VehiclePosition,
    None,
}

/// Parameters for [PTV::departures]
#[derive(Clone, Debug, Default, Serialize)]
pub struct DeparturesParams {
    /// Filter by platform number at stop
    pub platform_numbers: Option<Vec<i32>>,
    /// Filter by identifier of direction of travel; values returned by Directions API - /v3/directions/route/{route_id}
    pub direction_id: Option<DirectionId>,
    /// Indicates that stop_id parameter will accept \"GTFS stop_id\" data
    pub gtfs: Option<bool>,
    /// Filter by the date and time of the request (ISO 8601 UTC format) (default = current date and time)
    pub date_utc: Option<String>,
    /// Maximum number of results returned
    pub max_results: Option<i32>,
    /// Indicates if cancelled services (if they exist) are returned (default = false) - metropolitan train only
    pub include_cancelled: Option<bool>,
    /// Indicates if filtering runs (and their departures) to those that arrive at destination before date_utc (default = false). Requires max_results > 0.
    pub look_backwards: Option<bool>,
    /// List of objects to be returned in full (i.e. expanded)
    ///
    /// options include: All, Stop, Route, Run, Direction, Disruption, VehiclePosition, VehicleDescriptor or None.
    /// Run must be expanded to receive VehiclePosition and VehicleDescriptor information.
    pub expand: Option<Vec<ResponseExpand>>,
    /// Indicates if the route geopath should be returned
    pub include_geopath: Option<bool>,
}

impl<HTTPClient: PTVHttpClient> PTV<HTTPClient> {
    /// View departures for all routes from a stop
    ///
    /// Swagger operation: `Departures_GetForStop`
    pub async fn departures(
        &self,
        stop: &Stop,
        params: DeparturesParams,
    ) -> Result<DeparturesResponse> {
        let path = format!(
            "/v3/departures/route_type/{route_type}/stop/{stop_id}",
            route_type = stop.route_type.value(),
            stop_id = stop.id.value(),
        );
        let url = self.build_url(&path, &params);
        self.http_client.api_get(url).await
    }
}
