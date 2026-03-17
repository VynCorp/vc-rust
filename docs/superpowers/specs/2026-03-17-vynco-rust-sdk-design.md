# VynCo Rust SDK Design Spec

## Overview

Rust SDK for the VynCo Swiss Corporate Intelligence API (`https://api.vynco.ch/v1`). Mirrors the architecture of the existing VynFi Rust SDK: single crate, builder-based client, resource borrowing pattern, retry with exponential backoff, async + blocking variants.

Key addition over VynFi-rust: a `Response<T>` wrapper that exposes API response header metadata (credits used/remaining, request ID, rate limit info, data source attribution).

## Authentication

API key Bearer tokens only. Keys use `vc_live_*` (production) and `vc_test_*` (sandbox) prefixes.

```
Authorization: Bearer vc_live_XXXXXXXXXXXXXX
```

## Crate Configuration

- **Name:** `vynco`
- **Edition:** 2021
- **License:** Apache-2.0
- **Default base URL:** `https://api.vynco.ch/v1`
- **Default timeout:** 30 seconds
- **Default max retries:** 2
- **Features:** `default = ["rustls-tls"]`, `blocking`, `rustls-tls`, `native-tls`

### Dependencies

| Crate | Purpose |
|-------|---------|
| `reqwest` (0.12, json + rustls-tls) | HTTP client |
| `serde` (1, derive) | Serialization |
| `serde_json` (1) | JSON handling |
| `chrono` (0.4, serde) | Date/time types |
| `thiserror` (2) | Error derives |
| `tokio` (1, rt-multi-thread + macros) | Async runtime |

### Dev Dependencies

| Crate | Purpose |
|-------|---------|
| `mockito` (1) | HTTP mocking |

## Architecture

### File Structure

```
src/
  lib.rs              # Root module, public re-exports
  client.rs           # Client + ClientBuilder, request methods, retry
  error.rs            # VyncoError enum, ErrorBody
  types.rs            # All domain types/models
  response.rs         # Response<T> wrapper with ResponseMeta
  blocking.rs         # Synchronous client wrapper
  resources/
    mod.rs            # Resource module exports
    companies.rs      # Companies resource (search, get, count, stats, changes, persons)
    persons.rs        # Persons resource (get, search)
    dossiers.rs       # Dossiers resource (get, generate)
    api_keys.rs       # API Keys resource (create, list, revoke)
    credits.rs        # Credits resource (balance, usage, history)
    billing.rs        # Billing resource (checkout, portal sessions)
    webhooks.rs       # Webhooks resource (CRUD, test)
    teams.rs          # Teams resource (create, get)
    users.rs          # Users resource (profile, update)
    settings.rs       # Settings resource (get, update preferences)
tests/
  test_client.rs      # Integration tests with mockito
```

### Client (client.rs)

```rust
pub struct ClientBuilder {
    api_key: String,
    base_url: String,
    timeout: Duration,
    max_retries: u32,
}

#[derive(Clone)]
pub struct Client {
    http: reqwest::Client,
    base_url: String,
    max_retries: u32,
}
```

**Builder:** `Client::builder(api_key).base_url(...).timeout(...).max_retries(...).build()?`

**Request methods:**
- `request<T>(method, path) -> Result<Response<T>>` — no body
- `request_with_body<T, B>(method, path, body) -> Result<Response<T>>` — JSON body
- `request_with_params<T>(method, path, params) -> Result<Response<T>>` — query params
- `request_empty(method, path) -> Result<ResponseMeta>` — no response body (DELETE)

All methods parse response headers into `ResponseMeta`.

**Retry:** Exponential backoff (500ms, 1s, 2s...) on 429 and 5xx. Respects `Retry-After` header.

**Headers:**
- `Authorization: Bearer {api_key}`
- `User-Agent: vynco-rust/{version}`
- `Content-Type: application/json`

**Resource accessors:**
```rust
pub fn companies(&self) -> Companies<'_>
pub fn persons(&self) -> Persons<'_>
pub fn dossiers(&self) -> Dossiers<'_>
pub fn api_keys(&self) -> ApiKeys<'_>
pub fn credits(&self) -> Credits<'_>
pub fn billing(&self) -> Billing<'_>
pub fn webhooks(&self) -> Webhooks<'_>
pub fn teams(&self) -> Teams<'_>
pub fn users(&self) -> Users<'_>
pub fn settings(&self) -> Settings<'_>
```

### Response Wrapper (response.rs)

```rust
#[derive(Debug, Clone)]
pub struct ResponseMeta {
    pub request_id: Option<String>,
    pub credits_used: Option<i64>,
    pub credits_remaining: Option<i64>,
    pub rate_limit_limit: Option<u32>,
    pub data_source: Option<String>,
}

#[derive(Debug)]
pub struct Response<T> {
    pub data: T,
    pub meta: ResponseMeta,
}
```

`ResponseMeta` is parsed from response headers:
- `X-Request-Id` -> `request_id`
- `X-Credits-Used` -> `credits_used`
- `X-Credits-Remaining` -> `credits_remaining`
- `X-Rate-Limit-Limit` -> `rate_limit_limit`
- `X-Data-Source` -> `data_source`

### Error Handling (error.rs)

```rust
#[derive(Debug, Clone, Deserialize)]
pub struct ErrorBody {
    pub detail: String,
    pub message: String,
    pub status: u16,
}

#[derive(Debug, thiserror::Error)]
pub enum VyncoError {
    Authentication(ErrorBody),      // 401
    InsufficientCredits(ErrorBody), // 402
    Forbidden(ErrorBody),           // 403
    NotFound(ErrorBody),            // 404
    Validation(ErrorBody),          // 400/422
    RateLimit(ErrorBody),           // 429
    Server(ErrorBody),              // 5xx
    Http(reqwest::Error),           // Transport errors
    Deserialize(serde_json::Error), // Parse errors
    Config(String),                 // Configuration errors
}
```

### Types (types.rs)

#### Companies
```rust
pub struct Company {
    pub uid: String,
    pub name: String,
    pub legal_seat: String,
    pub canton: String,
    pub legal_form: String,
    pub status: String,
    pub purpose: String,
    pub capital_nominal: Option<f64>,
    pub capital_currency: Option<String>,
    pub auditor_name: Option<String>,
    pub registration_date: Option<String>,
    pub deletion_date: Option<String>,
    pub data_source: String,
    pub last_modified: String,
}

pub struct CompanySearchParams {
    pub search: Option<String>,
    pub canton: Option<String>,
    pub legal_form: Option<String>,
    pub status: Option<String>,
    pub sort_by: Option<String>,
    pub sort_desc: Option<bool>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: u64,
    pub page: u32,
    pub page_size: u32,
}

pub struct CompanyChange {
    pub id: String,
    pub company_uid: String,
    pub change_type: String,
    pub field_name: String,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub detected_at: String,
    pub source_date: Option<String>,
}

pub struct PersonRole {
    pub person_id: String,
    pub first_name: String,
    pub last_name: String,
    pub role: String,
    pub since: Option<String>,
    pub until: Option<String>,
}

pub struct CompanyCount {
    pub count: u64,
}

pub struct CompanyStatistics {
    // Aggregate statistics (total, enriched, by canton, by auditor category)
    // serde_json::Value for flexibility since the exact shape may evolve
}
```

#### Persons
```rust
pub struct Person {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub roles: Vec<PersonRole>,
}

pub struct PersonSearchParams {
    pub name: String,
}
```

#### Dossiers
```rust
pub struct Dossier {
    pub id: String,
    pub company_uid: String,
    pub status: String,
    pub executive_summary: Option<String>,
    pub key_insights: Option<Vec<String>>,
    pub risk_factors: Option<Vec<String>>,
    pub generated_at: Option<String>,
}

pub struct GenerateDossierRequest {
    pub level: String, // "summary" | "standard" | "comprehensive"
}
```

#### Credits & Billing
```rust
pub struct CreditBalance {
    pub balance: i64,
    pub monthly_credits: i64,
    pub used_this_month: i64,
    pub tier: String,
    pub overage_rate: f64,
}

pub struct UsageBreakdown {
    pub operations: Vec<UsageOperation>,
    pub total_debited: i64,
    pub period: UsagePeriod,
}

pub struct UsageOperation {
    pub operation: String,
    pub count: u64,
    pub credits: i64,
}

pub struct UsagePeriod {
    pub start: String,
    pub end: String,
}

pub struct CheckoutSessionResponse {
    pub url: String,
}

pub struct PortalSessionResponse {
    pub url: String,
}
```

#### API Keys
```rust
pub struct ApiKeyInfo {
    pub id: String,
    pub name: String,
    pub key_prefix: String,
    pub key_hint: String,
    pub permissions: Vec<String>,
    pub is_active: bool,
    pub last_used_at: Option<String>,
    pub created_at: String,
    pub expires_at: Option<String>,
}

pub struct ApiKeyCreated {
    pub id: String,
    pub name: String,
    pub raw_key: String,
    pub key_prefix: String,
    pub permissions: Vec<String>,
    pub created_at: String,
    pub expires_at: Option<String>,
}

pub struct CreateApiKeyRequest {
    pub name: String,
    pub is_test: bool,
    pub permissions: Vec<String>,
}
```

#### Teams
```rust
pub struct Team {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub tier: String,
    pub credit_balance: i64,
    pub monthly_credits: i64,
    pub overage_rate: f64,
    pub created_at: String,
}

pub struct CreateTeamRequest {
    pub name: String,
    pub slug: String,
}
```

#### Webhooks
```rust
pub struct Webhook {
    pub id: String,
    pub url: String,
    pub events: Vec<String>,
    pub status: String,
    pub secret: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

pub struct WebhookCreated {
    pub id: String,
    pub url: String,
    pub events: Vec<String>,
    pub secret: String,
    pub created_at: Option<String>,
}

pub struct CreateWebhookRequest {
    pub url: String,
    pub events: Vec<String>,
}

pub struct UpdateWebhookRequest {
    pub url: Option<String>,
    pub events: Option<Vec<String>>,
    pub status: Option<String>,
}
```

#### Users & Settings
```rust
pub struct UserProfile {
    pub id: String,
    pub name: String,
    pub email: String,
    pub avatar: String,
    pub plan: String,
    pub credit_balance: i64,
}

pub struct UpdateProfileRequest {
    pub name: Option<String>,
    pub email: Option<String>,
}

pub struct UserPreferences {
    // Flexible key-value preferences
    // Using serde_json::Value for forward compatibility
}
```

### Resources Pattern

Each resource borrows `&Client` and delegates to its request methods:

```rust
pub struct Companies<'a> {
    client: &'a Client,
}

impl<'a> Companies<'a> {
    pub(crate) fn new(client: &'a Client) -> Self { Self { client } }

    pub async fn search(&self, params: &CompanySearchParams) -> Result<Response<PaginatedResponse<Company>>>
    pub async fn get(&self, uid: &str) -> Result<Response<Company>>
    pub async fn count(&self, params: &CompanySearchParams) -> Result<Response<CompanyCount>>
    pub async fn statistics(&self) -> Result<Response<serde_json::Value>>
    pub async fn changes(&self, uid: &str) -> Result<Response<Vec<CompanyChange>>>
    pub async fn persons(&self, uid: &str) -> Result<Response<Vec<PersonRole>>>
}
```

All resources follow this identical pattern. See file structure above for the full list.

### Blocking Client (blocking.rs)

Wraps async `Client` with a single-threaded Tokio runtime. Each resource gets a synchronous wrapper. Enabled via `blocking` feature flag. No streaming methods.

### Testing

mockito-based integration tests covering:
- Empty API key validation
- Auth header format
- Error status code mapping (401, 402, 403, 404, 429, 5xx)
- Company search with pagination
- Company get by UID
- Credit balance retrieval
- API key creation
- Response header metadata parsing

## API Endpoints Covered

| Resource | Method | Path | SDK Method |
|----------|--------|------|------------|
| Companies | GET | /v1/companies | `companies().search(params)` |
| Companies | GET | /v1/companies/{uid} | `companies().get(uid)` |
| Companies | GET | /v1/companies/count | `companies().count(params)` |
| Companies | GET | /v1/companies/statistics | `companies().statistics()` |
| Companies | GET | /v1/companies/{uid}/changes | `companies().changes(uid)` |
| Companies | GET | /v1/companies/{uid}/persons | `companies().persons(uid)` |
| Companies | GET | /v1/companies/{uid}/dossier | `companies().dossier(uid)` |
| Persons | GET | /v1/persons/{uid} | `persons().get(uid)` |
| Persons | POST | /v1/persons/search | `persons().search(params)` |
| Dossiers | POST | /v1/dossiers | `dossiers().generate(uid, req)` |
| API Keys | GET | /v1/api-keys | `api_keys().list()` |
| API Keys | POST | /v1/api-keys | `api_keys().create(req)` |
| API Keys | DELETE | /v1/api-keys/{id} | `api_keys().revoke(id)` |
| Credits | GET | /v1/credits/balance | `credits().balance()` |
| Credits | GET | /v1/credits/usage | `credits().usage(since)` |
| Credits | GET | /v1/credits/history | `credits().history(limit, offset)` |
| Billing | POST | /v1/billing/checkout | `billing().create_checkout(tier)` |
| Billing | POST | /v1/billing/portal | `billing().create_portal()` |
| Webhooks | GET | /v1/webhooks | `webhooks().list()` |
| Webhooks | POST | /v1/webhooks | `webhooks().create(req)` |
| Webhooks | GET | /v1/webhooks/{id} | `webhooks().get(id)` |
| Webhooks | DELETE | /v1/webhooks/{id} | `webhooks().delete(id)` |
| Webhooks | POST | /v1/webhooks/{id}/test | `webhooks().test(id)` |
| Teams | POST | /v1/teams | `teams().create(req)` |
| Teams | GET | /v1/teams/me | `teams().me()` |
| Users | GET | /v1/auth/me | `users().me()` |
| Users | PUT | /v1/auth/profile | `users().update_profile(req)` |
| Settings | GET | /v1/settings/preferences | `settings().get()` |
| Settings | PUT | /v1/settings/preferences | `settings().update(prefs)` |

## Usage Example

```rust
use vynco::{Client, CompanySearchParams};

#[tokio::main]
async fn main() -> Result<(), vynco::VyncoError> {
    let client = Client::builder("vc_live_my_api_key")
        .build()?;

    // Search companies
    let params = CompanySearchParams {
        search: Some("Novartis".into()),
        canton: Some("BS".into()),
        ..Default::default()
    };
    let resp = client.companies().search(&params).await?;
    println!("Found {} companies", resp.data.total);
    println!("Credits used: {:?}", resp.meta.credits_used);

    // Get company details
    let company = client.companies().get("CHE-100.023.968").await?;
    println!("{}: {}", company.data.name, company.data.purpose);

    // Check credit balance
    let balance = client.credits().balance().await?;
    println!("Balance: {} credits", balance.data.balance);

    Ok(())
}
```
