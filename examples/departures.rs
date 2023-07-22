use ptv::{DeparturesParams, ResponseExpand, SearchParams, Stop};
use std::cmp::Reverse;
use std::iter::zip;
use std::str::FromStr;
use time::OffsetDateTime;

type PTV = ptv::PTV<::reqwest::Client>;

fn similar(left: &str, right: &str) -> usize {
    let matching = zip(left.chars(), right.chars())
        .take_while(|(l, r)| l.eq_ignore_ascii_case(r))
        .count();
    if matching == right.len() && matching == left.len() {
        return usize::MAX;
    } else {
        matching
    }
}

async fn search_for_stop(ptv: &PTV, search: &str) -> Stop {
    let params = SearchParams {
        include_outlets: Some(false),
        match_stop_by_suburb: Some(false),
        match_route_by_suburb: Some(false),
        ..Default::default()
    };

    let mut search_results = ptv
        .search(&search, params)
        .await
        .expect("Failed to search for station");

    search_results
        .stops
        .sort_by_key(|stop| Reverse(similar(&search, &stop.stop_name)));

    if search_results.stops.len() > 1 {
        println!("Multiple stops found for: {search}");
        for stop in &search_results.stops {
            println!("    {} {}", stop.stop(), stop.stop_name);
        }
    }

    search_results
        .stops
        .first()
        .expect("Station search returned nothing")
        .stop()
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Get the developer ID and Key from environment variables
    let devid = std::env::var("PTV_DEVID").expect("PTV_DEVID not set");
    let key = std::env::var("PTV_KEY").expect("PTV_KEY not set");
    let args: Vec<_> = std::env::args().collect();
    let search = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| "Flinders Street Station".to_string());

    // Create the API client instance
    let ptv = PTV::new(devid, key.to_string(), "RustPTVExample/0.1");

    let stop = if let Ok(stop) = Stop::from_str(&search) {
        stop
    } else {
        search_for_stop(&ptv, &search).await
    };

    // Setup parameters for our departures query
    // We ask for five results per route, and extra details for related routes, runs, and vehicles
    let params = DeparturesParams {
        expand: Some(vec![
            ResponseExpand::Route,
            ResponseExpand::Run,
            ResponseExpand::Stop,
            ResponseExpand::VehicleDescriptor,
        ]),
        max_results: Some(5),
        ..Default::default()
    };

    // Get departures from the API
    let result = ptv
        .departures(&stop, params)
        .await
        .expect("Failed to get departures");

    let stop_details = result.stops.get(&stop.id);
    let stop_name = stop_details
        .map(|s| s.stop_name.as_str())
        .unwrap_or("Unknown");

    println!("Departures for: {stop} {stop_name}");

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

            let run = &result.runs.get(&departure.run_ref).unwrap();

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
