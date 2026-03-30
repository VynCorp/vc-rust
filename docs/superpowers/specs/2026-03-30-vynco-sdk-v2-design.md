# VynCo Rust SDK v2.0.0 Design

**Date**: 2026-03-30
**Status**: Approved
**Scope**: Major version update — align SDK with rewritten Rust-based VynCo API

## Context

The VynCo API has been rewritten from .NET to Rust (Axum). The new API has a fundamentally different endpoint surface: 12 of 14 old resource modules are removed or replaced, 6 entirely new modules are added, response shapes changed from camelCase to snake_case, and the base URL prefix changed from `/api/v1` to `/v1`.

This is a clean rewrite of the resource/types layer. The proven SDK infrastructure (Client builder, retry logic, Response wrapper, error handling, blocking wrapper) is preserved with targeted modifications.

## Design Decisions

- **Version**: 2.0.0 (breaking changes to endpoint surface, types, and serde conventions)
- **Auth model**: Always require API key (single `Client` type). All endpoints will require auth soon.
- **Blog endpoints**: Excluded — outside the SDK's corporate intelligence domain.
- **Export download**: Full support including raw byte download for CSV/NDJSON files.
- **ResponseMeta**: Keep all existing fields (credits, rate limits, data source) — API will add missing headers.
- **Approach**: Clean rewrite of resources and types; keep infrastructure.

## Resource Modules (9 modules, 28 endpoints)

### 1. health (1 endpoint)

| Method | Path | SDK Method | Returns |
|--------|------|-----------|---------|
| GET | `/health` | `check()` | `Response<HealthResponse>` |

**HealthResponse**:
```rust
struct HealthResponse {
    status: String,
    database: String,
    redis: String,
    version: String,
}
```

### 2. companies (4 endpoints)

| Method | Path | SDK Method | Returns |
|--------|------|-----------|---------|
| GET | `/v1/companies` | `list(&CompanyListParams)` | `Response<PagedResponse<Company>>` |
| GET | `/v1/companies/{uid}` | `get(&str)` | `Response<Company>` |
| GET | `/v1/companies/count` | `count()` | `Response<CompanyCount>` |
| GET | `/v1/companies/{uid}/events` | `events(&str, Option<u32>)` | `Response<EventListResponse>` |

**CompanyListParams**:
```rust
struct CompanyListParams {
    search: Option<String>,
    canton: Option<String>,
    changed_since: Option<String>,
    page: Option<i64>,
    page_size: Option<i64>,
}
```

**Company** (renamed from CompanyResponse to match SDK conventions):
```rust
struct Company {
    uid: String,
    name: String,
    canton: Option<String>,
    status: Option<String>,
    legal_form: Option<String>,
    share_capital: Option<f64>,
    industry: Option<String>,
    auditor_category: Option<String>,
    updated_at: Option<String>,
}
```

**CompanyCount**:
```rust
struct CompanyCount {
    count: i64,
}
```

**EventListResponse**:
```rust
struct EventListResponse {
    events: Vec<CompanyEvent>,
    count: i64,
}
```

**CompanyEvent** (CloudEvents-compatible):
```rust
struct CompanyEvent {
    id: String,
    ce_type: String,
    ce_source: String,
    ce_time: String,
    company_uid: String,
    company_name: String,
    category: String,
    severity: String,
    summary: String,
    detail_json: serde_json::Value,
    created_at: String,
}
```

### 3. auditors (2 endpoints)

| Method | Path | SDK Method | Returns |
|--------|------|-----------|---------|
| GET | `/v1/companies/{uid}/auditor-history` | `history(&str)` | `Response<AuditorHistoryResponse>` |
| GET | `/v1/auditor-tenures` | `tenures(&AuditorTenureParams)` | `Response<PagedResponse<AuditorTenure>>` |

**AuditorHistoryResponse**:
```rust
struct AuditorHistoryResponse {
    company_uid: String,
    company_name: String,
    current_auditor: Option<AuditorTenure>,
    history: Vec<AuditorTenure>,
}
```

**AuditorTenure**:
```rust
struct AuditorTenure {
    id: String,
    company_uid: String,
    company_name: String,
    auditor_name: String,
    appointed_at: Option<String>,
    resigned_at: Option<String>,
    tenure_years: Option<f64>,
    is_current: bool,
    source: String,
}
```

**AuditorTenureParams**:
```rust
struct AuditorTenureParams {
    min_years: Option<f64>,
    canton: Option<String>,
    page: Option<i64>,
    page_size: Option<i64>,
}
```

### 4. dashboard (1 endpoint)

| Method | Path | SDK Method | Returns |
|--------|------|-----------|---------|
| GET | `/v1/dashboard` | `get()` | `Response<DashboardResponse>` |

**DashboardResponse**:
```rust
struct DashboardResponse {
    generated_at: String,
    data: DataCompleteness,
    pipelines: Vec<PipelineStatus>,
    auditor_tenures: AuditorTenureStats,
}

struct DataCompleteness {
    total_companies: i64,
    with_canton: i64,
    with_status: i64,
    with_legal_form: i64,
    with_capital: i64,
    with_industry: i64,
    with_auditor: i64,
    completeness_pct: f64,
}

struct PipelineStatus {
    name: String,
    last_run: Option<String>,
    status: String,
    records_processed: Option<i64>,
    duration_seconds: Option<f64>,
}

struct AuditorTenureStats {
    total_tenures: i64,
    long_tenures_7plus: i64,
    avg_tenure_years: f64,
    max_tenure_years: f64,
}
```

### 5. screening (1 endpoint)

| Method | Path | SDK Method | Returns |
|--------|------|-----------|---------|
| POST | `/v1/screening` | `screen(&ScreeningRequest)` | `Response<ScreeningResponse>` |

**ScreeningRequest**:
```rust
struct ScreeningRequest {
    name: String,
    uid: Option<String>,
    sources: Option<Vec<String>>,
}
```

**ScreeningResponse**:
```rust
struct ScreeningResponse {
    query_name: String,
    query_uid: Option<String>,
    screened_at: String,
    hit_count: i32,
    risk_level: String,
    hits: Vec<ScreeningHit>,
    sources_checked: Vec<String>,
}

struct ScreeningHit {
    source: String,
    matched_name: String,
    entity_type: String,
    score: f64,
    datasets: Vec<String>,
    details: serde_json::Value,
}
```

### 6. watchlists (7 endpoints)

| Method | Path | SDK Method | Returns |
|--------|------|-----------|---------|
| GET | `/v1/watchlists` | `list()` | `Response<Vec<WatchlistSummary>>` |
| POST | `/v1/watchlists` | `create(&CreateWatchlistRequest)` | `Response<Watchlist>` |
| DELETE | `/v1/watchlists/{id}` | `delete(&str)` | `Result<ResponseMeta>` |
| GET | `/v1/watchlists/{id}/companies` | `companies(&str)` | `Response<WatchlistCompaniesResponse>` |
| POST | `/v1/watchlists/{id}/companies` | `add_companies(&str, &AddCompaniesRequest)` | `Response<AddCompaniesResponse>` |
| DELETE | `/v1/watchlists/{id}/companies/{uid}` | `remove_company(&str, &str)` | `Result<ResponseMeta>` |
| GET | `/v1/watchlists/{id}/events` | `events(&str, Option<u32>)` | `Response<EventListResponse>` |

**Types**:
```rust
struct Watchlist {
    id: String,
    name: String,
    description: String,
    created_at: String,
    updated_at: String,
}

struct WatchlistSummary {
    id: String,
    name: String,
    description: String,
    company_count: i64,
    created_at: String,
}

struct CreateWatchlistRequest {
    name: String,
    description: Option<String>,
}

struct WatchlistCompaniesResponse {
    uids: Vec<String>,
}

struct AddCompaniesRequest {
    uids: Vec<String>,
}

struct AddCompaniesResponse {
    added: i64,
}
```

### 7. webhooks (6 endpoints)

| Method | Path | SDK Method | Returns |
|--------|------|-----------|---------|
| GET | `/v1/webhooks` | `list()` | `Response<Vec<WebhookSubscription>>` |
| POST | `/v1/webhooks` | `create(&CreateWebhookRequest)` | `Response<CreateWebhookResponse>` |
| PUT | `/v1/webhooks/{id}` | `update(&str, &UpdateWebhookRequest)` | `Response<WebhookSubscription>` |
| DELETE | `/v1/webhooks/{id}` | `delete(&str)` | `Result<ResponseMeta>` |
| POST | `/v1/webhooks/{id}/test` | `test(&str)` | `Response<TestDeliveryResponse>` |
| GET | `/v1/webhooks/{id}/deliveries` | `deliveries(&str, Option<u32>)` | `Response<Vec<WebhookDelivery>>` |

**Types**:
```rust
struct WebhookSubscription {
    id: String,
    url: String,
    description: String,
    event_filters: Vec<String>,
    company_filters: Vec<String>,
    status: String,
    created_at: String,
    updated_at: String,
}

struct CreateWebhookRequest {
    url: String,
    description: Option<String>,
    event_filters: Option<Vec<String>>,
    company_filters: Option<Vec<String>>,
}

struct CreateWebhookResponse {
    webhook: WebhookSubscription,
    signing_secret: String,
}

struct UpdateWebhookRequest {
    url: Option<String>,
    description: Option<String>,
    event_filters: Option<Vec<String>>,
    company_filters: Option<Vec<String>>,
    status: Option<String>,
}

struct TestDeliveryResponse {
    success: bool,
    http_status: Option<i32>,
    error: Option<String>,
}

struct WebhookDelivery {
    id: String,
    event_id: String,
    status: String,
    attempt: i32,
    http_status: Option<i32>,
    error_message: Option<String>,
    delivered_at: Option<String>,
    created_at: String,
}
```

### 8. exports (3 endpoints)

| Method | Path | SDK Method | Returns |
|--------|------|-----------|---------|
| POST | `/v1/exports` | `create(&CreateExportRequest)` | `Response<ExportJob>` |
| GET | `/v1/exports/{id}` | `get(&str)` | `Response<ExportDownload>` |
| GET | `/v1/exports/{id}/download` | `download(&str)` | `Result<ExportFile>` |

**Types**:
```rust
struct CreateExportRequest {
    format: Option<String>,
    canton: Option<String>,
    status: Option<String>,
    changed_since: Option<String>,
    industry: Option<String>,
    max_rows: Option<i64>,
}

struct ExportJob {
    id: String,
    status: String,
    format: String,
    total_rows: Option<i32>,
    file_size_bytes: Option<i64>,
    error_message: Option<String>,
    created_at: String,
    completed_at: Option<String>,
    expires_at: Option<String>,
}

struct ExportDownload {
    job: ExportJob,
    data: Option<String>,
}

struct ExportFile {
    meta: ResponseMeta,
    bytes: Vec<u8>,
    content_type: String,
    filename: String,
}
```

The `download` method uses a new `request_bytes()` internal method on Client that returns raw bytes instead of deserializing JSON. It parses `Content-Type` and `Content-Disposition` headers to populate `ExportFile`.

### 9. ai (3 endpoints)

| Method | Path | SDK Method | Returns |
|--------|------|-----------|---------|
| POST | `/v1/ai/dossier` | `dossier(&DossierRequest)` | `Response<DossierResponse>` |
| POST | `/v1/ai/search` | `search(&AiSearchRequest)` | `Response<AiSearchResponse>` |
| POST | `/v1/ai/risk-score` | `risk_score(&RiskScoreRequest)` | `Response<RiskScoreResponse>` |

**Types**:
```rust
struct DossierRequest {
    uid: String,
    depth: Option<String>,
}

struct DossierResponse {
    uid: String,
    company_name: String,
    dossier: String,
    sources: Vec<String>,
    generated_at: String,
}

struct AiSearchRequest {
    query: String,
}

struct AiSearchResponse {
    query: String,
    explanation: String,
    filters_applied: serde_json::Value,
    results: Vec<Company>,
    total: i64,
}

struct RiskScoreRequest {
    uid: String,
}

struct RiskScoreResponse {
    uid: String,
    company_name: String,
    overall_score: i32,
    risk_level: String,
    breakdown: Vec<RiskFactor>,
    assessed_at: String,
}

struct RiskFactor {
    factor: String,
    score: i32,
    weight: f64,
    description: String,
}
```

## Infrastructure Changes

### Base URL
Default changes from `https://api.vynco.ch/api/v1` to `https://api.vynco.ch/v1`.

### Serde Conventions
All types switch from `#[serde(rename_all = "camelCase")]` to `#[serde(rename_all = "snake_case")]`.

### ErrorBody
Update to match new RFC 7807 shape:
```rust
struct ErrorBody {
    #[serde(rename = "type")]
    error_type: String,
    title: String,
    status: u16,
    detail: Option<String>,
    instance: Option<String>,
}
```

### Client Changes
- Add `request_bytes()` internal method for export download (returns raw `Vec<u8>` with headers)
- Remove `extract_list<T>()` — no longer needed (new API uses consistent response shapes)
- Resource accessors: replace 14 old accessors with 9 new ones

### Blocking Client
Regenerate for the 9 new resource modules. Same `block_on()` pattern.

### ResponseMeta
No changes to the struct. Keep all fields including credits and data_source.

## Pagination

The new API uses a consistent pagination shape. Replace old `PaginatedResponse<T>` with:
```rust
struct PagedResponse<T> {
    items: Vec<T>,
    total: i64,
    page: i64,
    page_size: i64,
}
```

## Example CLI

Update `examples/vynco_cli.rs` to demonstrate:
- `health` — API health check
- `companies` — List with filters (canton, search)
- `company <uid>` — Get single company
- `count` — Company count
- `events <uid>` — Company events
- `screen <name>` — Sanctions screening
- `dashboard` — Dashboard stats
- `auditors` — Long-tenure auditors
- `risk <uid>` — AI risk score

## Files to Modify

| File | Action |
|------|--------|
| `Cargo.toml` | Version → 2.0.0, update description |
| `src/lib.rs` | Update module declarations and re-exports |
| `src/client.rs` | New base URL, new resource accessors, add `request_bytes()`, remove `extract_list()` |
| `src/error.rs` | Update `ErrorBody` fields |
| `src/types.rs` | Complete rewrite — all new types |
| `src/response.rs` | No changes |
| `src/resources/mod.rs` | Replace module declarations |
| `src/resources/health.rs` | Minor update (snake_case serde) |
| `src/resources/companies.rs` | Rewrite — 4 endpoints, new params |
| `src/resources/auditors.rs` | New file |
| `src/resources/dashboard.rs` | New file |
| `src/resources/screening.rs` | New file |
| `src/resources/watchlists.rs` | New file |
| `src/resources/webhooks.rs` | New file |
| `src/resources/exports.rs` | New file |
| `src/resources/ai.rs` | New file |
| `src/blocking.rs` | Regenerate for 9 resources |
| `examples/vynco_cli.rs` | Rewrite for new endpoints |
| `README.md` | Update for v2 endpoints, examples, base URL |
| `CLAUDE.md` | Update architecture, resource table, serde conventions |
| `CHANGELOG.md` | Add v2.0.0 entry |
| Old resource files | Delete: `analytics.rs`, `api_keys.rs`, `billing.rs`, `changes.rs`, `credits.rs`, `dossiers.rs`, `news.rs`, `persons.rs`, `relationships.rs`, `reports.rs`, `teams.rs`, `watches.rs` |

## Testing

All tests use `mockito` with the same pattern as v1. Each resource module gets tests for:
- Happy path (200/201/202/204 responses)
- Error cases (404, 422)
- Parameter serialization
- `max_retries(0)` for error tests to avoid retry delays
