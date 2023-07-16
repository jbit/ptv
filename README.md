Rust PTV Timetable API
======================

Unofficial Rust library for accessing the [Public Transport Victoria Timetable API](https://www.ptv.vic.gov.au/footer/data-and-reporting/datasets/ptv-timetable-api/).

You will require a unique key to access the API. Please check the [PTV site](https://www.ptv.vic.gov.au/footer/data-and-reporting/datasets/ptv-timetable-api/) ([PTV-Timetable-API-key-and-signature-document.rtf](https://www.ptv.vic.gov.au/assets/default-site/footer/data-and-reporting/Datasets/PTV-Timetable-API/60096c0692/PTV-Timetable-API-key-and-signature-document.rtf)) for instructions on how to register for an API key.

This library is designed to be HTTP client library agnostic, but includes [reqwest](https://crates.io/crates/reqwest) bindings.

Supported API Operations
------------------------
- [X] Departures_GetForStop
- [ ] Departures_GetForStopAndRoute
- [ ] Directions_ForRoute
- [ ] Directions_ForDirection
- [ ] Directions_ForDirectionAndType
- [ ] Disruptions_GetAllDisruptions
- [ ] Disruptions_GetDisruptionsByRoute
- [ ] Disruptions_GetDisruptionsByRouteAndStop
- [ ] Disruptions_GetDisruptionsByStop
- [ ] Disruptions_GetDisruptionById
- [ ] Disruptions_GetDisruptionModes
- [ ] FareEstimate_GetFareEstimateByZone
- [ ] Outlets_GetAllOutlets
- [ ] Outlets_GetOutletsByGeolocation
- [ ] Patterns_GetPatternByRun
- [ ] Routes_OneOrMoreRoutes
- [ ] Routes_RouteFromId
- [ ] RouteTypes_GetRouteTypes
- [ ] Runs_ForRoute
- [ ] Runs_ForRouteAndRouteType
- [ ] Runs_ForRun
- [ ] Runs_ForRunAndRouteType
- [ ] Search_Search
- [ ] Stops_StopDetails
- [ ] Stops_StopsForRoute
- [ ] Stops_StopsByGeolocation

Example
-------
There's an example program which displays upcoming departures for [Flinders Street railway station](https://en.wikipedia.org/wiki/Flinders_Street_railway_station).

Example of how to run:  
(be sure to replace the PTV_DEVID and PTV_KEY environment variables with your own!)
```text
PTV_DEVID=0 PTV_KEY=00000000-0000-0000-0000-000000000000 cargo run --example flinders-departures
```
Example output:
```text
Route: Alamein
Route: Belgrave
    Platform 2  departing in 29Min to Belgrave [6 Car Xtrapolis]
    Platform 2  departing in 89Min to Belgrave
Route: Craigieburn
    Platform 4  departing in  9Min to Craigieburn [6 Car Comeng]
    Platform 4  departing in 39Min to Craigieburn [6 Car Comeng]
    Platform 4  departing in 69Min to Craigieburn
Route: Cranbourne
    Platform ?? departing in 19Min to Caulfield
    Platform 6  departing in 32Min to Cranbourne [7-car HCMT]
    Platform 6  departing in 12Min to Cranbourne (Delayed 21Min) [7-car HCMT]
    Platform ?? departing in 39Min to Caulfield
    Platform 6  departing in 52Min to Cranbourne [7-car HCMT]
Route: Mernda
Route: Frankston
    Platform ?? departing in 19Min to Moorabbin
    Platform ?? departing in 39Min to Moorabbin
    Platform ?? departing in 59Min to Moorabbin
    Platform ?? departing in 79Min to Moorabbin
Route: Glen Waverley
    Platform 3  departing in 23Min to Glen Waverley
    Platform 2  departing in 53Min to Glen Waverley [6 Car Xtrapolis]
    Platform 3  departing in 83Min to Glen Waverley
Route: Hurstbridge
    Platform 1  departing in  6Min to Hurstbridge [6 Car Xtrapolis]
    Platform 1  departing in 24Min to Clifton Hill (Delayed 1Min) [6 Car Xtrapolis]
    Platform 1  departing in 28Min to Eltham
    Platform 1  departing in 43Min to Eltham
    Platform 1  departing in 54Min to Clifton Hill (Delayed 1Min) [6 Car Xtrapolis]
Route: Lilydale
    Platform 3  departing in 14Min to Ringwood [6 Car Xtrapolis]
    Platform 3  departing in 44Min to Ringwood [6 Car Xtrapolis]
    Platform 3  departing in 59Min to Lilydale
    Platform 2  departing in 74Min to Ringwood
Route: Pakenham
    Platform 6  departing in  2Min to Pakenham (Delayed 6Min) [7-car HCMT]
    Platform ?? departing in 19Min to Caulfield
    Platform 7  departing in 22Min to Pakenham (Delayed 9Min) [7-car HCMT]
    Platform ?? departing in 39Min to Caulfield
    Platform 7  departing in 42Min to Pakenham [7-car HCMT]
Route: Sandringham
    Platform 13 departing in  1Min to Sandringham [6 Car Comeng]
    Platform 13 departing in 21Min to Sandringham [6 Car Siemens]
    Platform 13 departing in 41Min to Sandringham [6 Car Siemens]
    Platform 13 departing in 61Min to Sandringham
    Platform 13 departing in 81Min to Sandringham
Route: Sunbury
Route: Upfield
    Platform 5  departing in  5Min to Upfield [6 Car Comeng]
    Platform 5  departing in 35Min to Upfield [6 Car Comeng]
    Platform 5  departing in 65Min to Upfield [6 Car Comeng]
Route: Werribee
    Platform 9  departing in 12Min to Werribee [6 Car Comeng]
    Platform 8  departing in 32Min to Werribee
    Platform 8  departing in 52Min to Werribee [6 Car Comeng]
    Platform 8  departing in 72Min to Werribee
Route: Williamstown
Route: Showgrounds - Flemington Racecourse
```
