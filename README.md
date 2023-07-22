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
- [X] Search_Search
- [ ] Stops_StopDetails
- [ ] Stops_StopsForRoute
- [ ] Stops_StopsByGeolocation

Example
-------
There's an example program which displays upcoming departures for a specific station, i.e. [Flinders Street railway station](https://en.wikipedia.org/wiki/Flinders_Street_railway_station).

Example of how to run:  
(be sure to replace the PTV_DEVID and PTV_KEY environment variables with your own!)
```text
PTV_DEVID=0 PTV_KEY=00000000-0000-0000-0000-000000000000 cargo run --example departures 'Flinders Street Station'
```
Example output:
```text
Multiple stops found for: Flinders Street Station
    Stop:Train/1071 Flinders Street Station
    Stop:Vline/1071 Flinders Street Railway Station 
    Stop:Tram/2722 Flinders Street Railway Station/Elizabeth St #1 
Departures for: Stop:Train/1071 Flinders Street Station
Route: Alamein
Route: Belgrave
    Platform 2  departing in 17Min to Belgrave [6 Car Xtrapolis]
    Platform 2  departing in 37Min to Belgrave [6 Car Xtrapolis]
    Platform 2  departing in 57Min to Belgrave [6 Car Xtrapolis]
    Platform 2  departing in 77Min to Belgrave [3 Car Siemens]
Route: Craigieburn
    Platform 5  departing in  2Min to Craigieburn [6 Car Siemens]
    Platform 5  departing in 22Min to Craigieburn [6 Car Comeng]
    Platform 5  departing in 42Min to Craigieburn
    Platform 5  departing in 62Min to Craigieburn
    Platform 5  departing in 82Min to Craigieburn
Route: Cranbourne
    Platform ?? departing in  2Min to Caulfield
    Platform ?? departing in 12Min to Caulfield
    Platform ?? departing in 22Min to Caulfield
    Platform ?? departing in 32Min to Caulfield
    Platform ?? departing in 42Min to Caulfield
Route: Mernda
    Platform 1  departing in 14Min to Mernda [6 Car Xtrapolis]
    Platform 1  departing in 34Min to Mernda
    Platform 1  departing in 54Min to Mernda
    Platform 1  departing in 74Min to Mernda
Route: Frankston
    Platform ?? departing in  2Min to Moorabbin
    Platform ?? departing in 12Min to Moorabbin
    Platform ?? departing in 22Min to Moorabbin
    Platform ?? departing in 32Min to Moorabbin
    Platform ?? departing in 42Min to Moorabbin
Route: Glen Waverley
    Platform 4  departing in  4Min to Glen Waverley [6 Car Xtrapolis]
    Platform 3  departing in 14Min to Glen Waverley
    Platform 4  departing in 24Min to Glen Waverley [6 Car Xtrapolis]
    Platform 3  departing in 34Min to Glen Waverley [6 Car Xtrapolis]
    Platform 4  departing in 44Min to Glen Waverley [6 Car Xtrapolis]
Route: Hurstbridge
    Platform 1  departing in  4Min to Heidelberg [6 Car Xtrapolis]
    Platform 1  departing in 24Min to Heidelberg [6 Car Xtrapolis]
    Platform 1  departing in 44Min to Heidelberg [6 Car Xtrapolis]
    Platform 1  departing in 64Min to Heidelberg [6 Car Xtrapolis]
    Platform 1  departing in 84Min to Heidelberg
Route: Lilydale
    Platform 3  departing in  0Min to Ringwood [6 Car Xtrapolis]
    Platform 2  departing in  7Min to Lilydale [6 Car Xtrapolis]
    Platform 3  departing in  8Min to Ringwood [6 Car Xtrapolis]
    Platform 2  departing in 27Min to Lilydale [6 Car Xtrapolis]
    Platform 2  departing in 47Min to Lilydale [6 Car Xtrapolis]
Route: Pakenham
    Platform ?? departing in  2Min to Caulfield
    Platform ?? departing in 12Min to Caulfield
    Platform ?? departing in 22Min to Caulfield
    Platform ?? departing in 32Min to Caulfield
    Platform ?? departing in 42Min to Caulfield
Route: Sandringham
    Platform 13 departing in  5Min to Sandringham [6 Car Siemens]
    Platform 12 departing in 15Min to Sandringham [6 Car Siemens]
    Platform 13 departing in 25Min to Sandringham [6 Car Comeng]
    Platform 12 departing in 35Min to Sandringham [6 Car Comeng]
    Platform 13 departing in 45Min to Sandringham [6 Car Comeng]
Route: Sunbury
    Platform 5  departing in  7Min to Watergardens [6 Car Comeng]
    Platform 10 departing in 13Min to Sunbury [6 Car Comeng]
    Platform 5  departing in 27Min to Sunbury
    Platform 5  departing in 47Min to Watergardens [6 Car Siemens]
    Platform 5  departing in 67Min to Sunbury
Route: Upfield
    Platform 5  departing in 17Min to Upfield [6 Car Comeng]
    Platform 5  departing in 37Min to Upfield [6 Car Siemens]
    Platform 5  departing in 57Min to Upfield
    Platform 5  departing in 77Min to Upfield
Route: Werribee
    Platform 8  departing in 11Min to Werribee [6 Car Comeng]
    Platform 8  departing in 31Min to Werribee [6 Car Comeng]
    Platform 8  departing in 51Min to Werribee [6 Car Siemens]
    Platform 8  departing in 71Min to Werribee [6 Car Comeng]
Route: Williamstown
    Platform 9  departing in  1Min to Williamstown [6 Car Comeng]
    Platform 9  departing in 21Min to Williamstown [6 Car Comeng]
    Platform 9  departing in 41Min to Williamstown [6 Car Comeng]
    Platform 9  departing in 61Min to Williamstown
    Platform 9  departing in 81Min to Williamstown
```
