# Exact-fidelity OpenAPI plan

> [!NOTE]
> This is a historical planning artifact. As of the `lta-next` migration, the
> project uses one evolving `openapi.yaml` as the generated-client source rather
> than maintaining a second `openapi.exact.yaml`. Bus Arrival is also an explicit
> contract exception: its generated public response follows the current DataMall
> v3 wire shape instead of the cooked handwritten SDK model. The blocker matrix
> below remains useful for planning later endpoint migrations.

## Goal

Create a second OpenAPI artifact, proposed as `openapi.exact.yaml`, after Satay grows the missing generator features needed to preserve the current `lta-rs` public SDK semantics exactly.

`openapi.yaml` remains the generator-valid wire-shape spec. It is intentionally conservative: it models DataMall JSON in forms Satay can generate today. `openapi.exact.yaml` should be stricter: it should encode the same wire inputs while generating Rust outputs that match the current handwritten SDK contracts.

## Non-goals

- Do not replace `openapi.yaml`; keep it as the stable Satay-compatible baseline.
- Do not require `../satay-rs` changes inside `lta-rs`.
- Do not commit generated Rust from either spec in this planning step.
- Do not preserve known accidental behavior unless explicitly chosen. The known example is `BusService.OriginCode` / `DestinationCode`: current `lta_models` parses 5-digit stop codes into `Option<NonZeroU8>`, which loses ordinary DataMall values such as `65009`.

## Proposed files

| File | Purpose | Expected status |
|---|---|---|
| `openapi.yaml` | Current Satay-compatible DataMall wire spec. | Must continue to generate today. |
| `openapi.exact.yaml` | Future exact-fidelity spec using additional Satay extensions. | Add only after upstream Satay supports the required extension set. |
| `openapi-exact-fidelity-plan.md` | This support matrix and migration plan. | Documentation/planning artifact. |

## Exact-fidelity definition

An endpoint is exact-fidelity only when generated code preserves all of these:

1. Same request path and query/header behavior as the current SDK.
2. Same public operation naming intent.
3. Same typed request parameters where the current SDK has typed parameters.
4. Same successful response payload shape as the current SDK, not just the same DataMall wrapper.
5. Same field-level conversions currently performed by `lta_models`, except for explicitly fixed bugs.
6. Same lossy behavior where the current SDK intentionally treats malformed/empty nested values as `None`.
7. Same fixture behavior across `../lta-models/dumped_data/*.json`.

## Upstream Satay feature keys

| Key | Satay feature | Needed for |
|---|---|---|
| F1 | Response projection / wrapper unwrapping, including `value` and nested `value[].Link`. | Matching current SDK return values instead of exposing OData wrappers. |
| F2 | Ignored-but-accepted fields, especially `odata.metadata`. | Accept DataMall metadata without exposing it in exact public models. |
| F3 | Open string enum schemas in parameters. | Typed `TrainLine` and geospatial `ID` request builders. |
| F4 | Parameter defaults. | Bike parking `Dist = 0.5` when omitted. |
| F5 | Parameter-group validation. | Passenger-volume `Date` and `$skip` mutual-exclusion behavior. |
| F6 | Configurable time parsing with `none-if` sentinels and multiple formats. | Bus route first/last bus fields and ERP start/end times. |
| F7 | Configurable date-time parsing with custom format strings and optional sentinels. | Faulty traffic light `StartDate` / `EndDate`. |
| F8 | Configurable boolean string parsing. | `TaxiStand.Bfa` and `BikeParking.ShelterIndicator`. |
| F9 | Delimited string parsing into typed arrays. | ERP `VehicleType` and train alert `AffectedSegment.Stations`. |
| F10 | Open integer enums / repr enum generation. | Train status, highway direction, and technical alarm type. |
| F11 | String-to-coordinate parser or external parse hook. | `CarPark.Location` as optional coordinates. |
| F12 | Alias-aware deserialization. | DataMall spelling drift and serde alias parity. |
| F13 | Contract decision or compatibility extension for known `lta_models` mismatch. | Bus service origin/destination stop-code behavior. |

## Endpoint support matrix

Legend:

- `Good wire support`: `openapi.yaml` can generate and model the DataMall wire shape today; exact SDK parity still needs common wrapper/projection features.
- `Needs upstream`: field semantics or request builder behavior require new Satay work.
- `Contract decision`: current handwritten behavior appears inconsistent with DataMall wire values; decide whether exact-fidelity means preserving the bug or fixing the SDK/spec together.

| Domain | Path | operationId | Current support | Exact-fidelity gaps | Needed Satay work |
|---|---|---|---|---|---|
| Bus | `/v3/BusArrival` | `getBusArrival` | Implemented in `openapi/lta.yaml`. The required request and response `BusStopCode` values use a fixed-width, regex-validated `BusStopCode` string newtype, preserving leading zeroes; optional `ServiceNo` remains typed; v3 timings require `Monitored`. | The response intentionally retains `BusArrivalResponse`; legacy v2 timing objects without `Monitored` are unsupported by maintainer decision. | None. |
| Bus | `/BusServices` | `getBusServices` | Needs contract decision. Frequencies can be parsed by current Satay; pagination is supported. | Wrapper exposure; `OriginCode` / `DestinationCode` currently differ from `lta_models` because DataMall has 5-digit stop codes while the model uses `Option<NonZeroU8>`. | F1, F2, F13 |
| Bus | `/BusRoutes` | `getBusRoutes` | Needs upstream. Request pagination and primitive fields are supported. | First/last bus fields are raw strings today; current SDK parses `HHMM` and `-` as optional times. Wrapper exposure. | F1, F2, F6 |
| Bus | `/BusStops` | `getBusStops` | Implemented in `openapi/lta.yaml`. `unwrap-field: value` returns `Vec<BusStop>`; bus stop codes use the fixed-width `BusStopCode` newtype, and coordinates use bounded `Latitude` / `Longitude` newtypes. | The stricter identifier and coordinate types are accepted public-contract changes. | None. |
| Crowd | `/PV/Bus` | `getPassengerVolumeBusStops` | Needs upstream. Link response wire shape is supported. | Current SDK returns `Vec<String>` links, not wrapper objects. `Date` and `$skip` exclusivity is not enforced. | F1, F2, F5 |
| Crowd | `/PV/ODBus` | `getPassengerVolumeOriginDestinationBusStops` | Needs upstream. Link response wire shape is supported. | Current SDK returns `Vec<String>` links, not wrapper objects. `Date` and `$skip` exclusivity is not enforced. | F1, F2, F5 |
| Crowd | `/PV/Train` | `getPassengerVolumeOriginDestinationTrainStations` | Needs upstream. Link response wire shape is supported. | Current SDK returns `Vec<String>` links, not wrapper objects. `Date` and `$skip` exclusivity is not enforced. | F1, F2, F5 |
| Crowd | `/PV/ODTrain` | `getPassengerVolumeTrainStations` | Needs upstream. Link response wire shape is supported. | Current SDK returns `Vec<String>` links, not wrapper objects. `Date` and `$skip` exclusivity is not enforced. | F1, F2, F5 |
| Crowd | `/PCDRealTime` | `getCrowdDensityRealTime` | Needs upstream. Response field conversions are mostly representable today. | `TrainLine` parameter had to be plain string because Satay rejects `anyOf` parameters. Current SDK uses richer station code typing than the current spec. Wrapper exposure. | F1, F2, F3, possibly F12 |
| Crowd | `/PCDForecast` | `getCrowdDensityForecast` | Needs upstream. Forecast wire shape is supported. | `TrainLine` parameter had to be plain string. Current SDK returns the first forecast object from the wrapper value array, not the full wrapper/array shape. | F1, F2, F3 |
| Facility | `/FacilitiesMaintenance` | `getFacilitiesMaintenance` | Implemented in `openapi/lta.yaml`. Required `StationCode` is a closed string enum of all 229 MRT/LRT station codes; `unwrap-field: value` + `map-field: Link` return an unwrapped `Vec<String>` of pre-signed links; `TimeStamp` is not surfaced; no `$skip` pagination; vendored fixture coverage (`tests/fixtures/facilities_maintenance/`, 2 files) passes. | Accepted deviations, tested in `tests/facilities_maintenance.rs`: (1) generated `StationCode` variants use satay's upper-camel naming (`Ns1`, `Ew24`) instead of the reference's `NS1`/`EW24`; wire serialization is identical; (2) no `Unknown` fallback variant — the reference's `#[serde(other)] Unknown` is unreachable for a request-only parameter, so unknown codes are rejected instead of serializing `?StationCode=Unknown`; (3) maintainer-approved: `StationCode` also includes all 56 codes LTA has officially announced for future lines/extensions (CRL `CR1`-`CR23` incl. Changi T5, CRL Punggol extension `CP1`-`CP4`, JRL `JS1`-`JS12`/`JS2A`/`JW1`-`JW5`/`JE1`-`JE7`, TELe `TE32`, DTL2e `DE1`-`DE2`, NSL `NS3A` infill), which the reference crate deliberately excludes until they open. | F1, F2 |
| Geo | `/GeospatialWholeIsland` | `getGeospatialWholeIsland` | Needs upstream. Link wire shape is supported. | `ID` parameter had to be plain string because Satay rejects `anyOf` parameters. Current SDK returns `Vec<String>` links, not wrapper objects. | F1, F2, F3, F12 |
| Taxi | `/Taxi-Availability` | `getTaxiAvailability` | Implemented in `openapi/lta.yaml`. `unwrap-field: value` returns `Vec<Coordinates>` whose fields use bounded `Latitude` / `Longitude` newtypes. | The bounded coordinate types are an accepted public-contract change. | None. |
| Taxi | `/TaxiStands` | `getTaxiStands` | Implemented in `openapi/lta.yaml`. Optional `$skip` is preserved; `unwrap-field: value` returns `Vec<TaxiStand>`; `TaxiCode` uses the fixed-width `TaxiCode` newtype (`^[A-Z][0-9]{2}$`); `Bfa` decodes through the configured boolean-string mapping (`Y`/`Yes`/`1`/`true` → true; `N`/`No`/`0`/`false`/`""` → false; other strings fall back to false via `unknown-as`); coordinates use bounded `Latitude` / `Longitude` newtypes; the 279-entry vendored fixture at `tests/fixtures/taxi_stands/taxi_stands_0.json` passes. | Unknown owner/type values remain losslessly represented as `TaxiStandOwner::Other(String)` / `TaxiStandType::Other(String)` rather than the reference's unit `Unknown` fallback (same accepted deviation as `RoadCategory`); the mapping also treats `1`/`true` as true where the reference only matches `Y`/`Yes`. | Configurable unit fallback would be required to reproduce the reference's lossy `Unknown` value. |
| Traffic | `/ERPRates` | `getErpRates` | Needs upstream. Pagination and basic fields are supported. | `VehicleType` remains raw slash-delimited string; start/end times remain raw strings. Current SDK parses vehicle types and optional times. Wrapper exposure. | F1, F2, F6, F9 |
| Traffic | `/CarParkAvailabilityv2` | `getCarParkAvailability` | Needs upstream. Most scalar fields and enums are representable. | `Location` remains raw nullable string; current SDK parses supported values into optional coordinates. Wrapper exposure. | F1, F2, F11 |
| Traffic | `/EstTravelTimes` | `getEstimatedTravelTimes` | Needs upstream. Request and most fields are supported. | `Direction` is plain integer today; current SDK uses an integer-valued enum. Wrapper exposure. | F1, F2, F10 |
| Traffic | `/FaultyTrafficLights` | `getFaultyTrafficLights` | Needs upstream. Request and scalar fields are supported. | `Type` is sparse integer enum; dates are space-separated optional date-times kept as strings today. Wrapper exposure. | F1, F2, F7, F10 |
| Traffic | `/RoadOpenings` | `getRoadOpenings` | Implemented in `openapi/lta.yaml`. Optional `$skip` is preserved; `unwrap-field: value` returns `Vec<RoadDetails>`; dates decode into `satay_runtime::Date`; `EventID` uses the ASCII-regex-validated `EventId` newtype (`^[A-Z]+-[0-9]{6}-[0-9]{4}$`); the 20-entry golden fixture passes. | The endpoint is separate from `/RoadWorks`; its fixture is synthetic because `lta-models` has no Road Openings dump. | None. |
| Traffic | `/RoadWorks` | `getRoadWorks` | Implemented in `openapi/lta.yaml`. Optional `$skip` is preserved; `unwrap-field: value` returns `Vec<RoadDetails>`; dates decode into `satay_runtime::Date`; `EventID` uses the ASCII-regex-validated `EventId` newtype (`^[A-Z]+-[0-9]{6}-[0-9]{4}$`); the 500-entry vendored fixture passes. | The generated API uses a dedicated endpoint instead of the reference's `RoadDetailsType` selector. | None. |
| Traffic | `/v3/TrafficSpeedBands` | `getTrafficSpeedBands` | Implemented in `openapi/lta.yaml`. `LinkID` is a fixed-width validated `LinkId`; `SpeedBand` is constrained to 1–8; `min_speed` is `u8`; the `999` open-ended maximum sentinel decodes to `max_speed: Option<u8>`; string coordinates remain parsed `f64`; the 500-entry fixture passes. | Unknown road categories remain losslessly represented as `RoadCategory::Other(String)` rather than the reference's unit fallback. | Configurable unit fallback would be required to reproduce the reference's lossy `Unknown` value. |
| Traffic | `/Traffic-Imagesv2` | `getTrafficImages` | Implemented in `openapi/lta.yaml`. Optional `$skip` is preserved; `unwrap-field: value` returns `Vec<TrafficImage>`; camera IDs use the fixed-width `CameraId` newtype; coordinates use bounded `Latitude` / `Longitude` newtypes; the 90-entry fixture at `tests/fixtures/traffic_images/traffic_images_0.json` passes. | The stricter identifier and coordinate types are accepted public-contract changes. | None. |
| Traffic | `/TrafficIncidents` | `getTrafficIncidents` | Implemented in `openapi/lta.yaml`. `unwrap-field: value` returns `Vec<TrafficIncident>`; coordinates use bounded `Latitude` / `Longitude` newtypes; DataMall spellings map to typed `IncidentType` variants; the 39-entry fixture at `tests/fixtures/traffic_incidents/traffic_incidents_0.json` passes. | Unknown spellings remain losslessly represented as `IncidentType::Other(String)`; canonical Rust variant spellings are not accepted as wire aliases. | Alias semantics or a configurable unit fallback would be required for exact reference behavior. |
| Traffic | `/VMS` | `getVariableMessageSigns` | Implemented in `openapi/lta.yaml`. Optional `$skip` is preserved; `unwrap-field: value` returns `Vec<Vms>`; `EquipmentID`/`Latitude`/`Longitude`/`Message` map to `equipment_id`/`lat`/`long`/`msg`; `EquipmentID` uses the `EquipmentId` newtype (`^[A-Z]{3,4}_[A-Z0-9]{4}$`, derived from the complete 12-record fixture); coordinates use bounded `Latitude` / `Longitude` newtypes; the 12-entry vendored fixture at `tests/fixtures/vms_emas/vms_emas_0.json` passes. | The stricter `EquipmentId` and bounded coordinate types are accepted public-contract changes. | None. |
| Traffic | `/BicycleParkingv2` | `getBikeParking` | Needs upstream. Required `Lat` / `Long` and optional `Dist` request generation are verified. | `Dist` default is not generated as current SDK behavior. `ShelterIndicator` is raw string today; current SDK parses `Y` / `Yes` as true. Wrapper exposure. | F1, F2, F4, F8 |
| Traffic | `/TrafficFlow` | `getTrafficFlow` | Implemented in `openapi/lta.yaml`. The request has no pagination parameter; `unwrap-field: value` + `map-field: Link` return the reference-compatible `Vec<String>`; the vendored `tests/fixtures/traffic_flow/traffic_flow.json` fixture passes. | No endpoint-specific deviations. | None. |
| Train | `/TrainServiceAlerts` | `getTrainServiceAlerts` | Needs upstream. Basic wrapper and nested object shape are supported. | `Status` is integer enum; `AffectedSegment.Stations` is dash-delimited string parsed by current SDK; response wrapper exposes `value`. | F1, F2, F9, F10 |

## Exact spec authoring strategy

1. Keep `openapi.yaml` generator-valid against current Satay on every change.
2. Add upstream Satay features behind small, explicit extensions instead of LTA-specific magic.
3. Create `openapi.exact.yaml` only when the required extension set is available in Satay.
4. Prefer generic Satay features over external Rust type overrides:
   - configurable parser sentinels,
   - delimited string arrays,
   - response projection,
   - open integer enums,
   - parameter validation.
5. Use external Rust type mapping only as an explicit escape hatch after generic features are exhausted.
6. Add golden fixture tests for every endpoint before switching consumers to generated code.

## Suggested exact-spec extension shape

### Wrapper projection

```yaml
x-satay:
  output:
    unwrap-field: value
```

For link endpoints:

```yaml
x-satay:
  output:
    unwrap-field: value
    map-field: Link
```

### Optional time with sentinels

```yaml
x-satay:
  parse-as: time
  formats: ["%H%M", "%H:%M"]
  none-if: ["", "-"]
```

### Boolean string parsing

```yaml
x-satay:
  parse-as: bool
  true-values: ["Y", "Yes", "1", "true"]
  false-values: ["N", "No", "0", "false", ""]
  unknown-as: false
```

### Delimited string arrays

```yaml
x-satay:
  parse-as: delimited-array
  delimiter: "/"
  items:
    $ref: "#/components/schemas/VehicleType"
```

### Open integer enums

```yaml
x-satay:
  integer-enum:
    1: Normal
    2: Disrupted
  open-enum: true
```

### Parameter groups

```yaml
x-satay:
  parameter-groups:
    - at-most-one-of: [Date, $skip]
```

## Verification plan for `openapi.exact.yaml`

For every endpoint:

1. Generate into `/tmp/lta-openapi-exact-check/src/generated`.
2. Compile a throwaway crate against `satay-runtime`.
3. Decode the corresponding `../lta-models/dumped_data/*.json` fixture through generated code.
4. Decode the same fixture through current `lta_models` types.
5. Compare public outputs, not raw wrapper structs.
6. Add request-builder tests for required parameters, optional parameters, defaults, and validation errors.

Minimum required request tests:

- `getBusArrival`: `AccountKey`, `/BusArrivalv2`, `BusStopCode`, optional `ServiceNo`.
- Passenger volume: reject `Date` plus `$skip` together if exact SDK parity keeps that invariant.
- `getCrowdDensityRealTime` / `getCrowdDensityForecast`: typed `TrainLine` parameter.
- `getGeospatialWholeIsland`: typed `ID` parameter.
- `getBikeParking`: required `Lat` / `Long`, default `Dist = 0.5`, explicit `.dist(...)` override.

## Cutover criteria

Do not migrate `lta-rs` callers from handwritten modules to generated code until all are true:

1. `openapi.yaml` still generates with current stable Satay.
2. `openapi.exact.yaml` generates with the new Satay feature set.
3. Every endpoint in the matrix is either exact-fidelity or explicitly accepted as a contract change.
4. Golden fixture tests pass for all current dumped DataMall fixtures.
5. Request-builder tests cover every required query parameter and every optional/default/query-validation rule.
6. The `BusService.OriginCode` / `DestinationCode` mismatch is resolved by an explicit maintainer decision.
