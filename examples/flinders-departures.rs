use ptv::{DeparturesParams, ResponseExpand, RouteType, Stop, StopId, PTV};
use time::OffsetDateTime;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Get the developer ID and Key from environment variables
    let devid = std::env::var("PTV_DEVID").expect("PTV_DEVID not set");
    let key = std::env::var("PTV_KEY").expect("PTV_KEY not set");

    // Create the API client instance
    let ptv = PTV::new(devid, key.to_string(), "RustPTVExample/0.1");

    // Hardcoded stop identifier for Flinders Street railway station
    const FLINDERS: Stop = Stop {
        route_type: RouteType::TRAIN,
        id: StopId::new(1071),
    };

    // Setup parameters for our departures query
    // We ask for five results per route, and extra details for related routes, runs, and vehicles
    let params = DeparturesParams {
        expand: Some(vec![
            ResponseExpand::Route,
            ResponseExpand::Run,
            ResponseExpand::VehicleDescriptor,
        ]),
        max_results: Some(5),
        ..Default::default()
    };

    // Get departures from the API
    let result = ptv
        .departures(FLINDERS, params)
        .await
        .expect("Failed to get departures");

    // Display the output:
    let now = OffsetDateTime::now_utc();

    for route in result.routes.values() {
        println!("Route: {}", route.route_name);
        for departure in &result.departures {
            if departure.route_id != route.route_id {
                continue;
            }
            let scheduled = departure.scheduled_departure_utc;
            let estimated = departure.estimated_departure_utc.unwrap_or(scheduled);
            let departing_min = (scheduled - now).whole_minutes();
            if departing_min > 90 {
                continue;
            }

            let platform = departure
                .platform_number
                .clone()
                .unwrap_or_else(|| "??".to_string());

            let delayed = (estimated - scheduled).whole_minutes();
            let delayed = if delayed > 0 {
                format!(" (Delayed {delayed}Min)")
            } else {
                String::new()
            };

            let run = &result.runs.get(&departure.run_id).unwrap();

            let details = if let Some(description) = run
                .vehicle_descriptor
                .as_ref()
                .and_then(|x| x.description.as_ref())
            {
                format!(" [{description}]")
            } else {
                String::new()
            };

            let destination = &run.destination_name;

            println!("    Platform {platform:-2} departing in {departing_min:-2}Min to {destination}{delayed}{details}");
        }
    }
}
