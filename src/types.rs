use super::*;

// Generate new-types for various Ids, allows more type safety
macro_rules! id_type {
    ($(#[$meta:meta])* $idname:ident) => {
        $(#[$meta])*
        ///
        /// Swagger type: `int32`
        #[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Deserialize, Serialize)]
        pub struct $idname(i32);
        impl $idname {
            /// Create from a raw `i32`
            pub const fn new(value: i32) -> Self {
                Self(value)
            }
            /// Convert to a raw `i32`
            pub const fn value(self) -> i32 {
                self.0
            }
        }
    };
    ($(#[$meta:meta])* $idname:ident, $(#[$helpermeta:meta])* $helpername:ident) => {
        id_type!($(#[$meta])* $idname);
        $(#[$helpermeta])*
        #[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
        pub struct $helpername {
            pub route_type: RouteType,
            pub id: $idname,
        }
    };
}
id_type!(
    /** Stop Identifier */
    StopId,
    /** Stop Identifier with associated transport mode */
    Stop
);
id_type!(
    /** Route Identifier */
    RouteId,
    /** Route Identifier with associated transport mode */
    Route
);
id_type!(
    /** Run Identifier */
    RunId,
    /** Run Identifier with associated transport mode */
    Run
);
id_type!(
    /** Direction Identifier */
    DirectionId,
    /** Direction Identifier with associated transport mode */
    Direction
);
id_type!(
    /** Disruption Identifier */
    DisruptionId
);

/// All route types (i.e. identifiers of transport modes) and their names.
///
/// Swagger type: `int32`
// These *should* be queried from the `/v3/route_types` API, but this is more ergonomic.
// It requires a rethink if the identifiers change, but that seems unlikely
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct RouteType(i32);
impl RouteType {
    pub const TRAIN: RouteType = RouteType(0);
    pub const TRAM: RouteType = RouteType(1);
    pub const BUS: RouteType = RouteType(2);
    pub const VLINE: RouteType = RouteType(3);
    pub const NIGHT_BUS: RouteType = RouteType(4);
    pub fn new(value: i32) -> Self {
        Self(value)
    }
    pub fn value(self) -> i32 {
        self.0
    }
}
impl std::fmt::Display for RouteType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Self::TRAIN => write!(f, "Train"),
            Self::TRAM => write!(f, "Tram"),
            Self::BUS => write!(f, "Bus"),
            Self::VLINE => write!(f, "Vline"),
            Self::NIGHT_BUS => write!(f, "Night Bus"),
            Self(unknown) => write!(f, "Unknown({unknown})"),
        }
    }
}

/// API system health status
///
/// Swagger type: `int32`
#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct HealthStatus(i32);
impl HealthStatus {
    pub const OFFLINE: HealthStatus = HealthStatus(0);
    pub const ONLINE: HealthStatus = HealthStatus(1);
}
impl std::fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Self::OFFLINE => write!(f, "Offline"),
            Self::ONLINE => write!(f, "Online"),
            Self(unknown) => write!(f, "Unknown({unknown})"),
        }
    }
}

/// API Status / Metadata
///
/// Swagger type: `V3.Status`
#[derive(Clone, Debug, Deserialize)]
pub struct Status {
    /// API Version number
    pub version: String,
    /// API system health status
    pub health: HealthStatus,
}

/// An error response
///
/// Swagger type: `V3.ErrorResponse`
#[derive(Clone, Debug, Deserialize)]
pub struct ErrorResponse {
    /// Error message
    pub message: String,
    /// API Status / Metadata
    pub status: Status,
}

/// Service departure details
///
/// Swagger type: `V3.Departure`
#[derive(Clone, Debug, Deserialize)]
pub struct DepartureDetails {
    /// Stop identifier
    pub stop_id: StopId,
    /// Route identifier
    pub route_id: RouteId,
    /// Numeric trip/service run identifier.
    /// Defaults to -1 when run identifier is Alphanumeric
    pub run_id: RunId,
    /// Alphanumeric trip/service run identifier
    pub run_ref: String,
    /// Direction of travel identifier
    pub direction_id: DirectionId,
    /// Disruption information identifier(s)
    pub disruption_ids: Vec<DisruptionId>,
    /// Scheduled (i.e. timetabled) departure time and date in ISO 8601 UTC format
    #[serde(with = "time::serde::iso8601")]
    pub scheduled_departure_utc: OffsetDateTime,
    /// Real-time estimate of departure time and date in ISO 8601 UTC format
    #[serde(with = "time::serde::iso8601::option")]
    pub estimated_departure_utc: Option<OffsetDateTime>,
    /// Indicates if the metropolitan train service is at the platform at the time of query; returns false for other modes
    pub at_platform: bool,
    /// Platform number at stop (metropolitan train only; returns null for other modes)
    pub platform_number: Option<String>,
    /// Flag indicating special condition for run (e.g. RR Reservations Required, GC Guaranteed Connection, DOO Drop Off Only, PUO Pick Up Only, MO Mondays only, TU Tuesdays only, WE Wednesdays only, TH Thursdays only, FR Fridays only, SS School days only; ignore E flag)
    pub flags: String,
    /// Chronological sequence for the departures in a run. Order ascendingly by this field to get chronological order (earliest first) of departures with the same run_ref.
    /// NOTE, this field is not always N+1 or N-1 of the previous or following departure. e.g 100, 200, 250, 300 instead of 1, 2, 3, 4
    pub departure_sequence: i32,
}

/// A train station, tram stop, bus stop, regional coach stop or Night Bus stop
///
/// Swagger type: `V3.StopModel`
#[derive(Clone, Debug, Deserialize)]
pub struct StopDetails {
    /// Distance of stop from input location (in metres); returns 0 if no location is input
    pub stop_distance: f32,
    /// suburb of stop
    pub stop_suburb: String,
    /// Name of stop
    pub stop_name: String,
    /// Stop identifier
    pub stop_id: StopId,
    /// Transport mode identifier
    pub route_type: RouteType,
    /// Geographic coordinate of latitude at stop
    pub stop_latitude: f32,
    /// Geographic coordinate of longitude at stop
    pub stop_longitude: f32,
    /// Landmark in proximity of stop
    pub stop_landmark: String,
    /// Sequence of the stop on the route/run; return 0 when route_id or run_id not specified. Order ascendingly by this field (when non zero) to get physical order (earliest first) of stops on the route_id/run_id.
    pub stop_sequence: i32,
}

/// Descriptor of the trip/service run
///
/// Swagger type: `V3.VehicleDescriptor`
#[derive(Clone, Debug, Deserialize)]
pub struct VehicleDescriptor {
    /// Operator name of the vehicle such as:
    ///   "Metro Trains Melbourne"
    ///   "Yarra Trams"
    ///   "Ventura Bus Line"
    ///   "CDC"
    ///   "Sita Bus Lines"
    /// May be null/empty.
    /// Only available for train, tram, v/line and some bus runs.
    pub operator: Option<String>,
    /// Operator identifier of the vehicle such as "26094".
    /// May be null/empty.
    /// Only available for some tram and bus runs.
    pub id: Option<String>,
    /// Indicator if vehicle has a low floor.
    /// May be null.
    /// Only available for some tram runs.
    pub low_floor: Option<bool>,
    /// Indicator if vehicle is air conditioned.
    /// May be null.
    /// Only available for some tram runs.
    pub air_conditioned: Option<bool>,
    /// Vehicle description such as:
    ///   "6 Car Comeng"
    ///   "6 Car Xtrapolis"
    ///   "3 Car Comeng"
    ///   "6 Car Siemens"
    ///   "3 Car Siemens"
    /// May be null/empty.
    /// Only available for some metropolitan train runs.
    pub description: Option<String>,
    /// Supplier of vehicle descriptor data.
    pub supplier: Option<String>,
    /// The length of the vehicle.
    /// Applies to CIS - Metro Trains
    pub length: Option<String>,
}

/// Position of the trip/service run
///
/// Swagger type: `V3.VehiclePosition`
#[derive(Clone, Debug, Deserialize)]
pub struct VehiclePosition {
    /// Geographic coordinate of latitude of the vehicle when known.
    /// Only available for some bus runs.
    pub latitude: Option<f64>,
    /// Geographic coordinate of longitude of the vehicle when known.
    /// Only available for some bus runs.
    pub longitude: Option<f64>,
    /// CIS - Metro Train Vehicle Location Easting coordinate
    pub easting: Option<f64>,
    /// CIS - Metro Train Vehicle Location Northing coordinate
    pub northing: Option<f64>,
    /// CIS - Metro Train Vehicle Location Direction
    pub direction: Option<String>,
    /// Compass bearing of the vehicle when known, clockwise from True North
    /// i.e., 0 is North and 90 is East.
    /// May be null.
    /// Only available for some bus runs.
    pub bearing: Option<f64>,
    /// Supplier of vehicle position data.
    pub supplier: Option<String>,
    /// Date and time that the vehicle position data was supplied.
    pub datetime_utc: String,
    /// CIS - Metro Train Vehicle Location data expiry time
    pub expiry_time: Option<String>,
}

/// Individual trips/services of a route
///
/// Swagger type: `V3.Run`
#[derive(Clone, Debug, Deserialize)]
pub struct RunDetails {
    /// Numeric trip/service run identifier. Defaults to -1 when run identifier is Alphanumeric
    pub run_id: RunId,
    /// Alphanumeric trip/service run identifier
    pub run_ref: String,
    /// Route identifier
    pub route_id: RouteId,
    /// Transport mode identifier
    pub route_type: RouteType,
    /// stop_id of final stop of run
    pub final_stop_id: StopId,
    /// Name of destination of run
    pub destination_name: String,
    /// Status of metropolitan train run; returns \"scheduled\" for other modes
    pub status: String,
    /// Direction of travel identifier
    pub direction_id: DirectionId,
    /// Chronological sequence of the trip/service run on the route in direction. Order ascendingly by this field to get chronological order (earliest first) of runs with the same route_id and direction_id.
    pub run_sequence: i32,
    /// The number of remaining skipped/express stations for the run/service from a stop
    pub express_stop_count: i32,
    /// Position of the trip/service run.
    /// Available for some Bus, Nightrider and Train runs.
    /// May be null.
    pub vehicle_position: Option<VehiclePosition>,
    /// Descriptor of the trip/service run.
    /// Only available for some runs.
    /// May be null.
    pub vehicle_descriptor: Option<VehicleDescriptor>,
}

/// Directions of travel of route
///
/// Swagger type: `V3.Direction`
#[derive(Clone, Debug, Deserialize)]
pub struct DirectionDetails {
    /// Direction of travel identifier
    pub direction_id: DirectionId,
    /// Name of direction of travel
    pub direction_name: String,
    /// Route identifier
    pub route_id: RouteId,
    /// Transport mode identifier
    pub route_type: RouteType,
}

/// Train lines, tram routes, bus routes, regional coach routes, Night Bus routes
#[derive(Clone, Debug, Deserialize)]
pub struct RouteDetails {
    /// Transport mode identifier
    pub route_type: RouteType,
    /// Route identifier
    pub route_id: RouteId,
    /// Name of route
    pub route_name: String,
    /// Route number presented to public (i.e. not route_id)
    pub route_number: String,
    /// GTFS Identifer of the route
    pub route_gtfs_id: Option<String>,
}

/// Disruption information applicable to relevant routes or stops
///
/// Swagger type: `V3.Disruption`
#[derive(Clone, Debug, Deserialize)]
pub struct DisruptionDetails {
    /// Disruption information identifier
    pub disruption_id: DisruptionId,
    /// Headline title summarising disruption information
    pub title: String,
    /// URL of relevant article on PTV website
    pub url: String,
    /// Description of the disruption
    pub description: String,
    /// Status of the disruption (e.g. "Planned", "Current")
    pub disruption_status: String,
    /// Type of disruption
    pub disruption_type: String,
    /// Date and time disruption information is published on PTV website, in ISO 8601 UTC format
    #[serde(rename = "published_on", skip_serializing_if = "Option::is_none")]
    pub published_on: Option<String>,
    /// Date and time disruption information was last updated by PTV, in ISO 8601 UTC format
    #[serde(rename = "last_updated", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    /// Date and time at which disruption begins, in ISO 8601 UTC format
    #[serde(rename = "from_date", skip_serializing_if = "Option::is_none")]
    pub from_date: Option<String>,
    /// Date and time at which disruption ends, in ISO 8601 UTC format (returns null if unknown)
    #[serde(rename = "to_date", skip_serializing_if = "Option::is_none")]
    pub to_date: Option<String>,
    /// Route relevant to a disruption (if applicable)
    //    pub routes: Option<Vec<crate::models::V3PeriodDisruptionRoute>>,
    /// Stop relevant to a disruption (if applicable)
    //    pub stops: Option<Vec<crate::models::V3PeriodDisruptionStop>>,
    pub colour: Option<String>,
    pub display_on_board: Option<bool>,
    pub display_status: Option<bool>,
}
