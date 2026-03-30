# VynCo Rust SDK v2.0.0 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Rewrite the vynco SDK resource/types layer to align with the new Rust-based VynCo API, producing a publishable v2.0.0 crate.

**Architecture:** Clean rewrite of all resource modules and types. Keep proven infrastructure (Client builder, retry, Response wrapper, error handling, blocking pattern). Base URL changes from `https://api.vynco.ch/api/v1` to `https://api.vynco.ch`. All paths now include `/v1/` prefix explicitly (except health at `/health`). Serde drops `camelCase` rename since the new API uses snake_case natively.

**Tech Stack:** Rust, reqwest, serde, tokio, mockito (tests), clap (example CLI)

---

## File Structure

| File | Action | Responsibility |
|------|--------|---------------|
| `Cargo.toml` | Modify | Version 2.0.0 |
| `src/error.rs` | Modify | Update ErrorBody fields |
| `src/types.rs` | Rewrite | All new API types |
| `src/client.rs` | Modify | New base URL, 9 resource accessors, `request_bytes()`, remove `extract_list()` |
| `src/response.rs` | Keep | No changes |
| `src/resources/mod.rs` | Rewrite | 9 new module declarations |
| `src/resources/health.rs` | Modify | Updated HealthResponse |
| `src/resources/companies.rs` | Rewrite | 4 endpoints + tests |
| `src/resources/auditors.rs` | Create | 2 endpoints + tests |
| `src/resources/dashboard.rs` | Create | 1 endpoint + tests |
| `src/resources/screening.rs` | Create | 1 endpoint + tests |
| `src/resources/watchlists.rs` | Create | 7 endpoints + tests |
| `src/resources/webhooks.rs` | Create | 6 endpoints + tests |
| `src/resources/exports.rs` | Create | 3 endpoints + tests |
| `src/resources/ai.rs` | Create | 3 endpoints + tests |
| `src/lib.rs` | Modify | New re-exports |
| `src/blocking.rs` | Rewrite | 9 resource wrappers |
| `examples/vynco_cli.rs` | Rewrite | New commands |
| `README.md` | Rewrite | v2 content |
| `CLAUDE.md` | Modify | Updated architecture section |
| `CHANGELOG.md` | Modify | v2.0.0 entry |
| Old resource files | Delete | 12 files removed |

---

### Task 1: Infrastructure — Cargo.toml and ErrorBody

**Files:**
- Modify: `Cargo.toml` (line 3: version)
- Modify: `src/error.rs` (lines 7-18: ErrorBody struct)

- [ ] **Step 1: Update Cargo.toml version**

Change line 3:
```toml
version = "2.0.0"
```

- [ ] **Step 2: Update ErrorBody in src/error.rs**

Replace the `ErrorBody` struct and its `Display` impl with:

```rust
/// RFC 7807 Problem Details error body returned by the VynCo API.
#[derive(Debug, Clone, Deserialize)]
pub struct ErrorBody {
    #[serde(default, rename = "type")]
    pub error_type: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub status: u16,
    #[serde(default)]
    pub detail: Option<String>,
    #[serde(default)]
    pub instance: Option<String>,
}

impl fmt::Display for ErrorBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref detail) = self.detail {
            if !detail.is_empty() {
                return write!(f, "{}", detail);
            }
        }
        if !self.title.is_empty() {
            write!(f, "{}", self.title)
        } else {
            write!(f, "HTTP {}", self.status)
        }
    }
}
```

- [ ] **Step 3: Update `map_error` fallback in src/client.rs**

The `map_error` method's fallback `ErrorBody` must match the new fields. Replace the `unwrap_or_else` block:

```rust
    async fn map_error(&self, status: StatusCode, resp: reqwest::Response) -> VyncoError {
        let body = resp
            .json::<ErrorBody>()
            .await
            .unwrap_or_else(|_| ErrorBody {
                error_type: String::new(),
                title: String::new(),
                status: status.as_u16(),
                detail: Some(format!("HTTP {}", status.as_u16())),
                instance: None,
            });
```

- [ ] **Step 4: Verify compilation**

Run: `cargo check 2>&1 | head -5`
Expected: Compilation succeeds (no errors)

- [ ] **Step 5: Commit**

```bash
git add Cargo.toml src/error.rs src/client.rs
git commit -m "chore: bump to v2.0.0, update ErrorBody to new RFC 7807 shape"
```

---

### Task 2: Rewrite types.rs

**Files:**
- Rewrite: `src/types.rs`

- [ ] **Step 1: Replace src/types.rs with all new v2 types**

```rust
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Pagination
// ---------------------------------------------------------------------------

/// Paginated response wrapper used by list endpoints.
#[derive(Debug, Clone, Deserialize)]
pub struct PagedResponse<T> {
    pub items: Vec<T>,
    #[serde(default)]
    pub total: i64,
    #[serde(default)]
    pub page: i64,
    #[serde(default)]
    pub page_size: i64,
}

// ---------------------------------------------------------------------------
// Health
// ---------------------------------------------------------------------------

/// API health status response.
#[derive(Debug, Clone, Deserialize)]
pub struct HealthResponse {
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub database: String,
    #[serde(default)]
    pub redis: String,
    #[serde(default)]
    pub version: String,
}

// ---------------------------------------------------------------------------
// Companies
// ---------------------------------------------------------------------------

/// A Swiss company record.
#[derive(Debug, Clone, Deserialize)]
pub struct Company {
    pub uid: String,
    pub name: String,
    #[serde(default)]
    pub canton: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub legal_form: Option<String>,
    #[serde(default)]
    pub share_capital: Option<f64>,
    #[serde(default)]
    pub industry: Option<String>,
    #[serde(default)]
    pub auditor_category: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

/// Query parameters for listing companies.
#[derive(Debug, Clone, Default, Serialize)]
pub struct CompanyListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canton: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changed_since: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

/// Company count response.
#[derive(Debug, Clone, Deserialize)]
pub struct CompanyCount {
    pub count: i64,
}

// ---------------------------------------------------------------------------
// Events
// ---------------------------------------------------------------------------

/// Event list response wrapper.
#[derive(Debug, Clone, Deserialize)]
pub struct EventListResponse {
    #[serde(default)]
    pub events: Vec<CompanyEvent>,
    #[serde(default)]
    pub count: i64,
}

/// A company event (CloudEvents-compatible).
#[derive(Debug, Clone, Deserialize)]
pub struct CompanyEvent {
    pub id: String,
    #[serde(default)]
    pub ce_type: String,
    #[serde(default)]
    pub ce_source: String,
    #[serde(default)]
    pub ce_time: String,
    #[serde(default)]
    pub company_uid: String,
    #[serde(default)]
    pub company_name: String,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub severity: String,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub detail_json: serde_json::Value,
    #[serde(default)]
    pub created_at: String,
}

// ---------------------------------------------------------------------------
// Auditors
// ---------------------------------------------------------------------------

/// Auditor history for a company.
#[derive(Debug, Clone, Deserialize)]
pub struct AuditorHistoryResponse {
    pub company_uid: String,
    #[serde(default)]
    pub company_name: String,
    pub current_auditor: Option<AuditorTenure>,
    #[serde(default)]
    pub history: Vec<AuditorTenure>,
}

/// An auditor tenure record.
#[derive(Debug, Clone, Deserialize)]
pub struct AuditorTenure {
    pub id: String,
    #[serde(default)]
    pub company_uid: String,
    #[serde(default)]
    pub company_name: String,
    #[serde(default)]
    pub auditor_name: String,
    pub appointed_at: Option<String>,
    pub resigned_at: Option<String>,
    pub tenure_years: Option<f64>,
    #[serde(default)]
    pub is_current: bool,
    #[serde(default)]
    pub source: String,
}

/// Query parameters for listing auditor tenures.
#[derive(Debug, Clone, Default, Serialize)]
pub struct AuditorTenureParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_years: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canton: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

// ---------------------------------------------------------------------------
// Dashboard
// ---------------------------------------------------------------------------

/// Dashboard overview response.
#[derive(Debug, Clone, Deserialize)]
pub struct DashboardResponse {
    #[serde(default)]
    pub generated_at: String,
    pub data: DataCompleteness,
    #[serde(default)]
    pub pipelines: Vec<PipelineStatus>,
    pub auditor_tenures: AuditorTenureStats,
}

/// Data completeness metrics.
#[derive(Debug, Clone, Deserialize)]
pub struct DataCompleteness {
    #[serde(default)]
    pub total_companies: i64,
    #[serde(default)]
    pub with_canton: i64,
    #[serde(default)]
    pub with_status: i64,
    #[serde(default)]
    pub with_legal_form: i64,
    #[serde(default)]
    pub with_capital: i64,
    #[serde(default)]
    pub with_industry: i64,
    #[serde(default)]
    pub with_auditor: i64,
    #[serde(default)]
    pub completeness_pct: f64,
}

/// Pipeline execution status.
#[derive(Debug, Clone, Deserialize)]
pub struct PipelineStatus {
    #[serde(default)]
    pub name: String,
    pub last_run: Option<String>,
    #[serde(default)]
    pub status: String,
    pub records_processed: Option<i64>,
    pub duration_seconds: Option<f64>,
}

/// Auditor tenure aggregate statistics.
#[derive(Debug, Clone, Deserialize)]
pub struct AuditorTenureStats {
    #[serde(default)]
    pub total_tenures: i64,
    #[serde(default)]
    pub long_tenures_7plus: i64,
    #[serde(default)]
    pub avg_tenure_years: f64,
    #[serde(default)]
    pub max_tenure_years: f64,
}

// ---------------------------------------------------------------------------
// Screening
// ---------------------------------------------------------------------------

/// Request body for sanctions/compliance screening.
#[derive(Debug, Clone, Serialize)]
pub struct ScreeningRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<String>>,
}

/// Screening result.
#[derive(Debug, Clone, Deserialize)]
pub struct ScreeningResponse {
    #[serde(default)]
    pub query_name: String,
    pub query_uid: Option<String>,
    #[serde(default)]
    pub screened_at: String,
    #[serde(default)]
    pub hit_count: i32,
    #[serde(default)]
    pub risk_level: String,
    #[serde(default)]
    pub hits: Vec<ScreeningHit>,
    #[serde(default)]
    pub sources_checked: Vec<String>,
}

/// A single screening hit.
#[derive(Debug, Clone, Deserialize)]
pub struct ScreeningHit {
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub matched_name: String,
    #[serde(default)]
    pub entity_type: String,
    #[serde(default)]
    pub score: f64,
    #[serde(default)]
    pub datasets: Vec<String>,
    #[serde(default)]
    pub details: serde_json::Value,
}

// ---------------------------------------------------------------------------
// Watchlists
// ---------------------------------------------------------------------------

/// A watchlist.
#[derive(Debug, Clone, Deserialize)]
pub struct Watchlist {
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
}

/// Summary of a watchlist (includes company count).
#[derive(Debug, Clone, Deserialize)]
pub struct WatchlistSummary {
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub company_count: i64,
    #[serde(default)]
    pub created_at: String,
}

/// Request to create a watchlist.
#[derive(Debug, Clone, Serialize)]
pub struct CreateWatchlistRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Companies in a watchlist.
#[derive(Debug, Clone, Deserialize)]
pub struct WatchlistCompaniesResponse {
    #[serde(default)]
    pub uids: Vec<String>,
}

/// Request to add companies to a watchlist.
#[derive(Debug, Clone, Serialize)]
pub struct AddCompaniesRequest {
    pub uids: Vec<String>,
}

/// Response after adding companies.
#[derive(Debug, Clone, Deserialize)]
pub struct AddCompaniesResponse {
    #[serde(default)]
    pub added: i64,
}

// ---------------------------------------------------------------------------
// Webhooks
// ---------------------------------------------------------------------------

/// A webhook subscription.
#[derive(Debug, Clone, Deserialize)]
pub struct WebhookSubscription {
    pub id: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub event_filters: Vec<String>,
    #[serde(default)]
    pub company_filters: Vec<String>,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
}

/// Request to create a webhook.
#[derive(Debug, Clone, Serialize)]
pub struct CreateWebhookRequest {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_filters: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_filters: Option<Vec<String>>,
}

/// Response from creating a webhook (includes signing secret).
#[derive(Debug, Clone, Deserialize)]
pub struct CreateWebhookResponse {
    pub webhook: WebhookSubscription,
    pub signing_secret: String,
}

/// Request to update a webhook.
#[derive(Debug, Clone, Serialize)]
pub struct UpdateWebhookRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_filters: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_filters: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Response from testing a webhook delivery.
#[derive(Debug, Clone, Deserialize)]
pub struct TestDeliveryResponse {
    #[serde(default)]
    pub success: bool,
    pub http_status: Option<i32>,
    pub error: Option<String>,
}

/// A webhook delivery record.
#[derive(Debug, Clone, Deserialize)]
pub struct WebhookDelivery {
    pub id: String,
    #[serde(default)]
    pub event_id: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub attempt: i32,
    pub http_status: Option<i32>,
    pub error_message: Option<String>,
    pub delivered_at: Option<String>,
    #[serde(default)]
    pub created_at: String,
}

// ---------------------------------------------------------------------------
// Exports
// ---------------------------------------------------------------------------

/// Request to create an export job.
#[derive(Debug, Clone, Serialize)]
pub struct CreateExportRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canton: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changed_since: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub industry: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_rows: Option<i64>,
}

/// An export job.
#[derive(Debug, Clone, Deserialize)]
pub struct ExportJob {
    pub id: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub format: String,
    pub total_rows: Option<i32>,
    pub file_size_bytes: Option<i64>,
    pub error_message: Option<String>,
    #[serde(default)]
    pub created_at: String,
    pub completed_at: Option<String>,
    pub expires_at: Option<String>,
}

/// Export status with optional inline data.
#[derive(Debug, Clone, Deserialize)]
pub struct ExportDownload {
    pub job: ExportJob,
    pub data: Option<String>,
}

// ---------------------------------------------------------------------------
// AI
// ---------------------------------------------------------------------------

/// Request to generate a due-diligence dossier.
#[derive(Debug, Clone, Serialize)]
pub struct DossierRequest {
    pub uid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<String>,
}

/// Generated dossier.
#[derive(Debug, Clone, Deserialize)]
pub struct DossierResponse {
    pub uid: String,
    #[serde(default)]
    pub company_name: String,
    #[serde(default)]
    pub dossier: String,
    #[serde(default)]
    pub sources: Vec<String>,
    #[serde(default)]
    pub generated_at: String,
}

/// Request for AI-powered natural language search.
#[derive(Debug, Clone, Serialize)]
pub struct AiSearchRequest {
    pub query: String,
}

/// AI search result.
#[derive(Debug, Clone, Deserialize)]
pub struct AiSearchResponse {
    #[serde(default)]
    pub query: String,
    #[serde(default)]
    pub explanation: String,
    #[serde(default)]
    pub filters_applied: serde_json::Value,
    #[serde(default)]
    pub results: Vec<Company>,
    #[serde(default)]
    pub total: i64,
}

/// Request for multi-signal risk scoring.
#[derive(Debug, Clone, Serialize)]
pub struct RiskScoreRequest {
    pub uid: String,
}

/// Risk score result.
#[derive(Debug, Clone, Deserialize)]
pub struct RiskScoreResponse {
    pub uid: String,
    #[serde(default)]
    pub company_name: String,
    #[serde(default)]
    pub overall_score: i32,
    #[serde(default)]
    pub risk_level: String,
    #[serde(default)]
    pub breakdown: Vec<RiskFactor>,
    #[serde(default)]
    pub assessed_at: String,
}

/// A single risk factor in a risk score breakdown.
#[derive(Debug, Clone, Deserialize)]
pub struct RiskFactor {
    #[serde(default)]
    pub factor: String,
    #[serde(default)]
    pub score: i32,
    #[serde(default)]
    pub weight: f64,
    #[serde(default)]
    pub description: String,
}
```

- [ ] **Step 2: Verify compilation**

Run: `cargo check 2>&1 | head -5`
Expected: Will fail due to client.rs/lib.rs still referencing old types — that's expected. Just verify types.rs itself has no syntax errors.

- [ ] **Step 3: Commit**

```bash
git add src/types.rs
git commit -m "feat: rewrite types.rs for v2 API surface"
```

---

### Task 3: Update client.rs

**Files:**
- Modify: `src/client.rs`

- [ ] **Step 1: Update base URL constant**

```rust
const DEFAULT_BASE_URL: &str = "https://api.vynco.ch";
```

Update the doc comment on `base_url()`:
```rust
    /// Set the API base URL (default: `https://api.vynco.ch`).
```

- [ ] **Step 2: Replace resource accessors**

Replace the entire resource accessors block (lines 98-153) with:

```rust
    // -- Resource accessors --------------------------------------------------

    pub fn health(&self) -> Health<'_> {
        Health::new(self)
    }

    pub fn companies(&self) -> Companies<'_> {
        Companies::new(self)
    }

    pub fn auditors(&self) -> Auditors<'_> {
        Auditors::new(self)
    }

    pub fn dashboard(&self) -> Dashboard<'_> {
        Dashboard::new(self)
    }

    pub fn screening(&self) -> Screening<'_> {
        Screening::new(self)
    }

    pub fn watchlists(&self) -> Watchlists<'_> {
        Watchlists::new(self)
    }

    pub fn webhooks(&self) -> Webhooks<'_> {
        Webhooks::new(self)
    }

    pub fn exports(&self) -> Exports<'_> {
        Exports::new(self)
    }

    pub fn ai(&self) -> Ai<'_> {
        Ai::new(self)
    }
```

- [ ] **Step 3: Add `request_bytes()` method**

Add after `request_empty`:

```rust
    /// Send a request and return raw bytes (for file downloads).
    pub(crate) async fn request_bytes(
        &self,
        method: Method,
        path: &str,
    ) -> Result<(Vec<u8>, ResponseMeta, String, String)> {
        let resp = self
            .execute_raw(self.http.request(method.clone(), self.url(path)))
            .await?;
        let meta = ResponseMeta::from_headers(resp.headers());
        let status = resp.status();

        if !status.is_success() {
            return Err(self.map_error(status, resp).await);
        }

        let content_type = resp
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("application/octet-stream")
            .to_string();

        let filename = resp
            .headers()
            .get("content-disposition")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.split("filename=").nth(1).map(|f| f.trim_matches('"').to_string()))
            .unwrap_or_else(|| "export".to_string());

        let bytes = resp.bytes().await.map_err(VyncoError::Http)?;
        Ok((bytes.to_vec(), meta, content_type, filename))
    }
```

- [ ] **Step 4: Remove `extract_list()` method**

Delete the entire `extract_list` method (lines 316-338).

- [ ] **Step 5: Commit**

```bash
git add src/client.rs
git commit -m "feat: update client for v2 — new base URL, resource accessors, request_bytes"
```

---

### Task 4: Delete old resources, scaffold new modules

**Files:**
- Delete: `src/resources/analytics.rs`, `api_keys.rs`, `billing.rs`, `changes.rs`, `credits.rs`, `dossiers.rs`, `news.rs`, `persons.rs`, `relationships.rs`, `reports.rs`, `teams.rs`, `watches.rs`
- Rewrite: `src/resources/mod.rs`
- Modify: `src/lib.rs`

- [ ] **Step 1: Delete old resource files**

```bash
rm src/resources/analytics.rs src/resources/api_keys.rs src/resources/billing.rs \
   src/resources/changes.rs src/resources/credits.rs src/resources/dossiers.rs \
   src/resources/news.rs src/resources/persons.rs src/resources/relationships.rs \
   src/resources/reports.rs src/resources/teams.rs src/resources/watches.rs
```

- [ ] **Step 2: Rewrite src/resources/mod.rs**

```rust
mod ai;
mod auditors;
mod companies;
mod dashboard;
mod exports;
mod health;
mod screening;
mod watchlists;
mod webhooks;

pub use ai::Ai;
pub use auditors::Auditors;
pub use companies::Companies;
pub use dashboard::Dashboard;
pub use exports::{ExportFile, Exports};
pub use health::Health;
pub use screening::Screening;
pub use watchlists::Watchlists;
pub use webhooks::Webhooks;
```

- [ ] **Step 3: Update src/lib.rs**

```rust
pub mod client;
pub mod error;
pub mod resources;
pub mod response;
pub mod types;

#[cfg(feature = "blocking")]
pub mod blocking;

// Re-export core types at crate root for ergonomic imports.
pub use client::{Client, ClientBuilder};
pub use error::{ErrorBody, VyncoError};
pub use resources::{
    Ai, Auditors, Companies, Dashboard, ExportFile, Exports, Health, Screening, Watchlists,
    Webhooks,
};
pub use response::{Response, ResponseMeta};
pub use types::*;
```

- [ ] **Step 4: Create empty stub files for new resources (so cargo check can run)**

Create each file with a minimal struct so compilation works. These will be filled in Tasks 5-13.

`src/resources/ai.rs`:
```rust
use crate::client::Client;

pub struct Ai<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> Ai<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }
}
```

Create identical stubs for: `auditors.rs` (`Auditors`), `dashboard.rs` (`Dashboard`), `screening.rs` (`Screening`), `watchlists.rs` (`Watchlists`), `webhooks.rs` (`Webhooks`).

`src/resources/exports.rs` (includes ExportFile):
```rust
use crate::client::Client;
use crate::response::ResponseMeta;

pub struct Exports<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> Exports<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }
}

/// Downloaded export file with raw bytes and metadata.
#[derive(Debug)]
pub struct ExportFile {
    pub meta: ResponseMeta,
    pub bytes: Vec<u8>,
    pub content_type: String,
    pub filename: String,
}
```

- [ ] **Step 5: Verify compilation**

Run: `cargo check 2>&1 | head -10`
Expected: Compiles successfully. Blocking module and example will have errors — that's OK for now.

Run: `cargo check 2>&1 | grep "^error" | head -5`
Expected: Only errors from `blocking.rs` and the example, not from core lib.

- [ ] **Step 6: Commit**

```bash
git add -A src/resources/ src/lib.rs
git commit -m "refactor: scaffold v2 resource modules, remove 12 old modules"
```

---

### Task 5: Health resource

**Files:**
- Modify: `src/resources/health.rs`

- [ ] **Step 1: Update health.rs**

```rust
use reqwest::Method;

use crate::client::Client;
use crate::error::Result;
use crate::response::Response;
use crate::types::HealthResponse;

pub struct Health<'a> {
    client: &'a Client,
}

impl<'a> Health<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Check API health status.
    pub async fn check(&self) -> Result<Response<HealthResponse>> {
        self.client.request(Method::GET, "/health").await
    }
}

#[cfg(test)]
mod tests {
    use crate::Client;

    #[tokio::test]
    async fn test_health_check() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/health")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_header("X-Request-Id", "req_abc123")
            .with_body(r#"{"status":"ok","database":"connected","redis":"connected","version":"1.5.0"}"#)
            .create_async()
            .await;

        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();

        let resp = client.health().check().await.unwrap();
        assert_eq!(resp.data.status, "ok");
        assert_eq!(resp.data.database, "connected");
        assert_eq!(resp.data.redis, "connected");
        assert_eq!(resp.data.version, "1.5.0");
        assert_eq!(resp.meta.request_id.as_deref(), Some("req_abc123"));

        mock.assert_async().await;
    }
}
```

- [ ] **Step 2: Run test**

Run: `cargo test test_health_check -- --nocapture 2>&1 | tail -5`
Expected: `test ... ok`

- [ ] **Step 3: Commit**

```bash
git add src/resources/health.rs
git commit -m "feat: update health resource for v2 API"
```

---

### Task 6: Companies resource + tests

**Files:**
- Rewrite: `src/resources/companies.rs`

- [ ] **Step 1: Write companies.rs**

```rust
use reqwest::Method;

use crate::client::Client;
use crate::error::Result;
use crate::response::Response;
use crate::types::*;

pub struct Companies<'a> {
    client: &'a Client,
}

impl<'a> Companies<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List companies with optional filtering and pagination.
    pub async fn list(
        &self,
        params: &CompanyListParams,
    ) -> Result<Response<PagedResponse<Company>>> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(ref s) = params.search {
            query.push(("search", s.clone()));
        }
        if let Some(ref c) = params.canton {
            query.push(("canton", c.clone()));
        }
        if let Some(ref cs) = params.changed_since {
            query.push(("changed_since", cs.clone()));
        }
        if let Some(p) = params.page {
            query.push(("page", p.to_string()));
        }
        if let Some(ps) = params.page_size {
            query.push(("pageSize", ps.to_string()));
        }

        if query.is_empty() {
            self.client.request(Method::GET, "/v1/companies").await
        } else {
            self.client
                .request_with_params(Method::GET, "/v1/companies", &query)
                .await
        }
    }

    /// Get a company by its Swiss UID (e.g. "CHE-105.805.080").
    pub async fn get(&self, uid: &str) -> Result<Response<Company>> {
        self.client
            .request(Method::GET, &format!("/v1/companies/{uid}"))
            .await
    }

    /// Get the total count of companies.
    pub async fn count(&self) -> Result<Response<CompanyCount>> {
        self.client
            .request(Method::GET, "/v1/companies/count")
            .await
    }

    /// Get events for a company.
    pub async fn events(
        &self,
        uid: &str,
        limit: Option<u32>,
    ) -> Result<Response<EventListResponse>> {
        let path = format!("/v1/companies/{uid}/events");
        if let Some(l) = limit {
            self.client
                .request_with_params(Method::GET, &path, &[("limit", l.to_string())])
                .await
        } else {
            self.client.request(Method::GET, &path).await
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Client;

    #[tokio::test]
    async fn test_companies_list() {
        let mut server = mockito::Server::new_async().await;
        let body = r#"{"items":[{"uid":"CHE-100.023.968","name":"Test AG","canton":"ZH","status":"active"}],"total":1,"page":1,"page_size":20}"#;
        let mock = server
            .mock("GET", "/v1/companies?canton=ZH&page=1&pageSize=20")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create_async()
            .await;

        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();

        let params = crate::CompanyListParams {
            canton: Some("ZH".into()),
            page: Some(1),
            page_size: Some(20),
            ..Default::default()
        };
        let resp = client.companies().list(&params).await.unwrap();
        assert_eq!(resp.data.total, 1);
        assert_eq!(resp.data.items.len(), 1);
        assert_eq!(resp.data.items[0].uid, "CHE-100.023.968");

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_companies_get() {
        let mut server = mockito::Server::new_async().await;
        let body = r#"{"uid":"CHE-100.023.968","name":"Test AG","canton":"ZH","status":"active","legal_form":"AG","share_capital":100000.0}"#;
        let mock = server
            .mock("GET", "/v1/companies/CHE-100.023.968")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create_async()
            .await;

        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();

        let resp = client.companies().get("CHE-100.023.968").await.unwrap();
        assert_eq!(resp.data.name, "Test AG");
        assert_eq!(resp.data.share_capital, Some(100000.0));

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_companies_get_not_found() {
        let mut server = mockito::Server::new_async().await;
        let body = r#"{"type":"https://api.vynco.ch/errors/not-found","title":"Not Found","status":404,"detail":"Company not found"}"#;
        let mock = server
            .mock("GET", "/v1/companies/CHE-000.000.000")
            .with_status(404)
            .with_header("content-type", "application/problem+json")
            .with_body(body)
            .create_async()
            .await;

        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .max_retries(0)
            .build()
            .unwrap();

        let err = client.companies().get("CHE-000.000.000").await.unwrap_err();
        assert!(matches!(err, crate::VyncoError::NotFound(_)));

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_companies_count() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/v1/companies/count")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"count":507234}"#)
            .create_async()
            .await;

        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();

        let resp = client.companies().count().await.unwrap();
        assert_eq!(resp.data.count, 507234);

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_companies_events() {
        let mut server = mockito::Server::new_async().await;
        let body = r#"{"events":[{"id":"evt-1","ce_type":"company.auditor.changed","ce_source":"https://api.vynco.ch","ce_time":"2026-03-01T00:00:00Z","company_uid":"CHE-100.023.968","company_name":"Test AG","category":"auditor_change","severity":"medium","summary":"Auditor changed","detail_json":{},"created_at":"2026-03-01T00:00:00Z"}],"count":1}"#;
        let mock = server
            .mock("GET", "/v1/companies/CHE-100.023.968/events?limit=10")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create_async()
            .await;

        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();

        let resp = client
            .companies()
            .events("CHE-100.023.968", Some(10))
            .await
            .unwrap();
        assert_eq!(resp.data.count, 1);
        assert_eq!(resp.data.events[0].category, "auditor_change");

        mock.assert_async().await;
    }
}
```

- [ ] **Step 2: Run tests**

Run: `cargo test companies::tests -- --nocapture 2>&1 | tail -10`
Expected: All 5 tests pass

- [ ] **Step 3: Commit**

```bash
git add src/resources/companies.rs
git commit -m "feat: rewrite companies resource for v2 — list, get, count, events"
```

---

### Task 7: Auditors resource + tests

**Files:**
- Create: `src/resources/auditors.rs`

- [ ] **Step 1: Write auditors.rs**

```rust
use reqwest::Method;

use crate::client::Client;
use crate::error::Result;
use crate::response::Response;
use crate::types::*;

pub struct Auditors<'a> {
    client: &'a Client,
}

impl<'a> Auditors<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Get auditor history for a company.
    pub async fn history(&self, uid: &str) -> Result<Response<AuditorHistoryResponse>> {
        self.client
            .request(Method::GET, &format!("/v1/companies/{uid}/auditor-history"))
            .await
    }

    /// List long-tenure auditors with optional filters.
    pub async fn tenures(
        &self,
        params: &AuditorTenureParams,
    ) -> Result<Response<PagedResponse<AuditorTenure>>> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(y) = params.min_years {
            query.push(("min_years", y.to_string()));
        }
        if let Some(ref c) = params.canton {
            query.push(("canton", c.clone()));
        }
        if let Some(p) = params.page {
            query.push(("page", p.to_string()));
        }
        if let Some(ps) = params.page_size {
            query.push(("pageSize", ps.to_string()));
        }

        if query.is_empty() {
            self.client
                .request(Method::GET, "/v1/auditor-tenures")
                .await
        } else {
            self.client
                .request_with_params(Method::GET, "/v1/auditor-tenures", &query)
                .await
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Client;

    #[tokio::test]
    async fn test_auditor_history() {
        let mut server = mockito::Server::new_async().await;
        let body = r#"{"company_uid":"CHE-100.023.968","company_name":"Test AG","current_auditor":{"id":"t1","company_uid":"CHE-100.023.968","company_name":"Test AG","auditor_name":"KPMG AG","appointed_at":"2020-01-01","resigned_at":null,"tenure_years":6.2,"is_current":true,"source":"ZefixRest"},"history":[]}"#;
        let mock = server
            .mock("GET", "/v1/companies/CHE-100.023.968/auditor-history")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create_async()
            .await;

        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();

        let resp = client.auditors().history("CHE-100.023.968").await.unwrap();
        assert_eq!(resp.data.company_uid, "CHE-100.023.968");
        let current = resp.data.current_auditor.unwrap();
        assert_eq!(current.auditor_name, "KPMG AG");
        assert!(current.is_current);

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_auditor_tenures() {
        let mut server = mockito::Server::new_async().await;
        let body = r#"{"items":[{"id":"t1","company_uid":"CHE-100.023.968","company_name":"Test AG","auditor_name":"KPMG AG","appointed_at":"2015-01-01","resigned_at":null,"tenure_years":11.2,"is_current":true,"source":"ZefixRest"}],"total":1,"page":1,"page_size":50}"#;
        let mock = server
            .mock("GET", "/v1/auditor-tenures?min_years=10&page=1&pageSize=50")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create_async()
            .await;

        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();

        let params = crate::AuditorTenureParams {
            min_years: Some(10.0),
            page: Some(1),
            page_size: Some(50),
            ..Default::default()
        };
        let resp = client.auditors().tenures(&params).await.unwrap();
        assert_eq!(resp.data.total, 1);
        assert!(resp.data.items[0].tenure_years.unwrap() > 10.0);

        mock.assert_async().await;
    }
}
```

- [ ] **Step 2: Run tests**

Run: `cargo test auditors::tests -- --nocapture 2>&1 | tail -5`
Expected: 2 tests pass

- [ ] **Step 3: Commit**

```bash
git add src/resources/auditors.rs
git commit -m "feat: add auditors resource — history, tenures"
```

---

### Task 8: Dashboard resource + tests

**Files:**
- Create: `src/resources/dashboard.rs`

- [ ] **Step 1: Write dashboard.rs**

```rust
use reqwest::Method;

use crate::client::Client;
use crate::error::Result;
use crate::response::Response;
use crate::types::DashboardResponse;

pub struct Dashboard<'a> {
    client: &'a Client,
}

impl<'a> Dashboard<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Get dashboard overview with data completeness and pipeline status.
    pub async fn get(&self) -> Result<Response<DashboardResponse>> {
        self.client.request(Method::GET, "/v1/dashboard").await
    }
}

#[cfg(test)]
mod tests {
    use crate::Client;

    #[tokio::test]
    async fn test_dashboard_get() {
        let mut server = mockito::Server::new_async().await;
        let body = r#"{"generated_at":"2026-03-30T12:00:00Z","data":{"total_companies":507000,"with_canton":500000,"with_status":495000,"with_legal_form":490000,"with_capital":300000,"with_industry":200000,"with_auditor":150000,"completeness_pct":72.5},"pipelines":[{"name":"zefix_sync","last_run":"2026-03-30T06:00:00Z","status":"success","records_processed":1200,"duration_seconds":45.3}],"auditor_tenures":{"total_tenures":45000,"long_tenures_7plus":3200,"avg_tenure_years":5.8,"max_tenure_years":42.0}}"#;
        let mock = server
            .mock("GET", "/v1/dashboard")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create_async()
            .await;

        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();

        let resp = client.dashboard().get().await.unwrap();
        assert_eq!(resp.data.data.total_companies, 507000);
        assert_eq!(resp.data.pipelines.len(), 1);
        assert_eq!(resp.data.auditor_tenures.long_tenures_7plus, 3200);

        mock.assert_async().await;
    }
}
```

- [ ] **Step 2: Run test**

Run: `cargo test dashboard::tests -- --nocapture 2>&1 | tail -5`
Expected: 1 test passes

- [ ] **Step 3: Commit**

```bash
git add src/resources/dashboard.rs
git commit -m "feat: add dashboard resource"
```

---

### Task 9: Screening resource + tests

**Files:**
- Create: `src/resources/screening.rs`

- [ ] **Step 1: Write screening.rs**

```rust
use reqwest::Method;

use crate::client::Client;
use crate::error::Result;
use crate::response::Response;
use crate::types::{ScreeningRequest, ScreeningResponse};

pub struct Screening<'a> {
    client: &'a Client,
}

impl<'a> Screening<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Screen an entity against sanctions and compliance databases.
    pub async fn screen(&self, req: &ScreeningRequest) -> Result<Response<ScreeningResponse>> {
        self.client
            .request_with_body(Method::POST, "/v1/screening", req)
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::{Client, ScreeningRequest};

    #[tokio::test]
    async fn test_screening() {
        let mut server = mockito::Server::new_async().await;
        let body = r#"{"query_name":"Test Corp","query_uid":null,"screened_at":"2026-03-30T12:00:00Z","hit_count":0,"risk_level":"clear","hits":[],"sources_checked":["seco","opensanctions","finma"]}"#;
        let mock = server
            .mock("POST", "/v1/screening")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create_async()
            .await;

        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();

        let req = ScreeningRequest {
            name: "Test Corp".into(),
            uid: None,
            sources: None,
        };
        let resp = client.screening().screen(&req).await.unwrap();
        assert_eq!(resp.data.risk_level, "clear");
        assert_eq!(resp.data.hit_count, 0);
        assert_eq!(resp.data.sources_checked.len(), 3);

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_screening_with_hits() {
        let mut server = mockito::Server::new_async().await;
        let body = r#"{"query_name":"Sanctioned Entity","query_uid":null,"screened_at":"2026-03-30T12:00:00Z","hit_count":1,"risk_level":"high","hits":[{"source":"seco","matched_name":"Sanctioned Entity Ltd","entity_type":"entity","score":0.95,"datasets":["seco-sanctions"],"details":{}}],"sources_checked":["seco"]}"#;
        let mock = server
            .mock("POST", "/v1/screening")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create_async()
            .await;

        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();

        let req = ScreeningRequest {
            name: "Sanctioned Entity".into(),
            uid: None,
            sources: Some(vec!["seco".into()]),
        };
        let resp = client.screening().screen(&req).await.unwrap();
        assert_eq!(resp.data.risk_level, "high");
        assert_eq!(resp.data.hits.len(), 1);
        assert!(resp.data.hits[0].score > 0.9);

        mock.assert_async().await;
    }
}
```

- [ ] **Step 2: Run tests**

Run: `cargo test screening::tests -- --nocapture 2>&1 | tail -5`
Expected: 2 tests pass

- [ ] **Step 3: Commit**

```bash
git add src/resources/screening.rs
git commit -m "feat: add screening resource — sanctions/compliance screening"
```

---

### Task 10: Watchlists resource + tests

**Files:**
- Create: `src/resources/watchlists.rs`

- [ ] **Step 1: Write watchlists.rs**

```rust
use reqwest::Method;

use crate::client::Client;
use crate::error::Result;
use crate::response::{Response, ResponseMeta};
use crate::types::*;

pub struct Watchlists<'a> {
    client: &'a Client,
}

impl<'a> Watchlists<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List all watchlists for the current user.
    pub async fn list(&self) -> Result<Response<Vec<WatchlistSummary>>> {
        self.client.request(Method::GET, "/v1/watchlists").await
    }

    /// Create a new watchlist.
    pub async fn create(&self, req: &CreateWatchlistRequest) -> Result<Response<Watchlist>> {
        self.client
            .request_with_body(Method::POST, "/v1/watchlists", req)
            .await
    }

    /// Delete a watchlist.
    pub async fn delete(&self, id: &str) -> Result<ResponseMeta> {
        self.client
            .request_empty(Method::DELETE, &format!("/v1/watchlists/{id}"))
            .await
    }

    /// List company UIDs in a watchlist.
    pub async fn companies(&self, id: &str) -> Result<Response<WatchlistCompaniesResponse>> {
        self.client
            .request(Method::GET, &format!("/v1/watchlists/{id}/companies"))
            .await
    }

    /// Add companies to a watchlist.
    pub async fn add_companies(
        &self,
        id: &str,
        req: &AddCompaniesRequest,
    ) -> Result<Response<AddCompaniesResponse>> {
        self.client
            .request_with_body(Method::POST, &format!("/v1/watchlists/{id}/companies"), req)
            .await
    }

    /// Remove a company from a watchlist.
    pub async fn remove_company(&self, id: &str, uid: &str) -> Result<ResponseMeta> {
        self.client
            .request_empty(
                Method::DELETE,
                &format!("/v1/watchlists/{id}/companies/{uid}"),
            )
            .await
    }

    /// Get events for companies in a watchlist.
    pub async fn events(
        &self,
        id: &str,
        limit: Option<u32>,
    ) -> Result<Response<EventListResponse>> {
        let path = format!("/v1/watchlists/{id}/events");
        if let Some(l) = limit {
            self.client
                .request_with_params(Method::GET, &path, &[("limit", l.to_string())])
                .await
        } else {
            self.client.request(Method::GET, &path).await
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Client, CreateWatchlistRequest, AddCompaniesRequest};

    #[tokio::test]
    async fn test_watchlists_create() {
        let mut server = mockito::Server::new_async().await;
        let body = r#"{"id":"wl-123","name":"My Watchlist","description":"Test","created_at":"2026-03-30T12:00:00Z","updated_at":"2026-03-30T12:00:00Z"}"#;
        let mock = server
            .mock("POST", "/v1/watchlists")
            .with_status(201)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create_async()
            .await;

        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();

        let req = CreateWatchlistRequest {
            name: "My Watchlist".into(),
            description: Some("Test".into()),
        };
        let resp = client.watchlists().create(&req).await.unwrap();
        assert_eq!(resp.data.id, "wl-123");
        assert_eq!(resp.data.name, "My Watchlist");

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_watchlists_list() {
        let mut server = mockito::Server::new_async().await;
        let body = r#"[{"id":"wl-123","name":"My Watchlist","description":"","company_count":5,"created_at":"2026-03-30T12:00:00Z"}]"#;
        let mock = server
            .mock("GET", "/v1/watchlists")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create_async()
            .await;

        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();

        let resp = client.watchlists().list().await.unwrap();
        assert_eq!(resp.data.len(), 1);
        assert_eq!(resp.data[0].company_count, 5);

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_watchlists_add_companies() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/v1/watchlists/wl-123/companies")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"added":2}"#)
            .create_async()
            .await;

        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();

        let req = AddCompaniesRequest {
            uids: vec!["CHE-100.023.968".into(), "CHE-105.805.080".into()],
        };
        let resp = client
            .watchlists()
            .add_companies("wl-123", &req)
            .await
            .unwrap();
        assert_eq!(resp.data.added, 2);

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_watchlists_delete() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("DELETE", "/v1/watchlists/wl-123")
            .with_status(204)
            .create_async()
            .await;

        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();

        let meta = client.watchlists().delete("wl-123").await.unwrap();
        assert!(meta.request_id.is_none()); // no headers in mock

        mock.assert_async().await;
    }
}
```

- [ ] **Step 2: Run tests**

Run: `cargo test watchlists::tests -- --nocapture 2>&1 | tail -10`
Expected: 4 tests pass

- [ ] **Step 3: Commit**

```bash
git add src/resources/watchlists.rs
git commit -m "feat: add watchlists resource — CRUD, companies, events"
```

---

### Task 11: Webhooks resource + tests

**Files:**
- Create: `src/resources/webhooks.rs`

- [ ] **Step 1: Write webhooks.rs**

```rust
use reqwest::Method;

use crate::client::Client;
use crate::error::Result;
use crate::response::{Response, ResponseMeta};
use crate::types::*;

pub struct Webhooks<'a> {
    client: &'a Client,
}

impl<'a> Webhooks<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List all webhook subscriptions.
    pub async fn list(&self) -> Result<Response<Vec<WebhookSubscription>>> {
        self.client.request(Method::GET, "/v1/webhooks").await
    }

    /// Create a new webhook subscription.
    pub async fn create(
        &self,
        req: &CreateWebhookRequest,
    ) -> Result<Response<CreateWebhookResponse>> {
        self.client
            .request_with_body(Method::POST, "/v1/webhooks", req)
            .await
    }

    /// Update a webhook subscription.
    pub async fn update(
        &self,
        id: &str,
        req: &UpdateWebhookRequest,
    ) -> Result<Response<WebhookSubscription>> {
        self.client
            .request_with_body(Method::PUT, &format!("/v1/webhooks/{id}"), req)
            .await
    }

    /// Delete a webhook subscription.
    pub async fn delete(&self, id: &str) -> Result<ResponseMeta> {
        self.client
            .request_empty(Method::DELETE, &format!("/v1/webhooks/{id}"))
            .await
    }

    /// Send a test delivery to a webhook.
    pub async fn test(&self, id: &str) -> Result<Response<TestDeliveryResponse>> {
        self.client
            .request_with_body(
                Method::POST,
                &format!("/v1/webhooks/{id}/test"),
                &serde_json::Value::Object(serde_json::Map::new()),
            )
            .await
    }

    /// List recent deliveries for a webhook.
    pub async fn deliveries(
        &self,
        id: &str,
        limit: Option<u32>,
    ) -> Result<Response<Vec<WebhookDelivery>>> {
        let path = format!("/v1/webhooks/{id}/deliveries");
        if let Some(l) = limit {
            self.client
                .request_with_params(Method::GET, &path, &[("limit", l.to_string())])
                .await
        } else {
            self.client.request(Method::GET, &path).await
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Client, CreateWebhookRequest, UpdateWebhookRequest};

    #[tokio::test]
    async fn test_webhooks_create() {
        let mut server = mockito::Server::new_async().await;
        let body = r#"{"webhook":{"id":"wh-123","url":"https://example.com/hook","description":"Test","event_filters":["auditor_change"],"company_filters":[],"status":"active","created_at":"2026-03-30T12:00:00Z","updated_at":"2026-03-30T12:00:00Z"},"signing_secret":"whsec_abc123"}"#;
        let mock = server
            .mock("POST", "/v1/webhooks")
            .with_status(201)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create_async()
            .await;

        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();

        let req = CreateWebhookRequest {
            url: "https://example.com/hook".into(),
            description: Some("Test".into()),
            event_filters: Some(vec!["auditor_change".into()]),
            company_filters: None,
        };
        let resp = client.webhooks().create(&req).await.unwrap();
        assert_eq!(resp.data.webhook.id, "wh-123");
        assert_eq!(resp.data.signing_secret, "whsec_abc123");

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_webhooks_update() {
        let mut server = mockito::Server::new_async().await;
        let body = r#"{"id":"wh-123","url":"https://example.com/hook","description":"Updated","event_filters":[],"company_filters":[],"status":"paused","created_at":"2026-03-30T12:00:00Z","updated_at":"2026-03-30T13:00:00Z"}"#;
        let mock = server
            .mock("PUT", "/v1/webhooks/wh-123")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create_async()
            .await;

        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();

        let req = UpdateWebhookRequest {
            status: Some("paused".into()),
            description: Some("Updated".into()),
            ..Default::default()
        };
        let resp = client.webhooks().update("wh-123", &req).await.unwrap();
        assert_eq!(resp.data.status, "paused");

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_webhooks_test_delivery() {
        let mut server = mockito::Server::new_async().await;
        let body = r#"{"success":true,"http_status":200,"error":null}"#;
        let mock = server
            .mock("POST", "/v1/webhooks/wh-123/test")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create_async()
            .await;

        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();

        let resp = client.webhooks().test("wh-123").await.unwrap();
        assert!(resp.data.success);
        assert_eq!(resp.data.http_status, Some(200));

        mock.assert_async().await;
    }
}
```

- [ ] **Step 2: Add `Default` derive to `UpdateWebhookRequest` in types.rs**

The test above uses `..Default::default()` on `UpdateWebhookRequest`. Add `Default` to the derive list for this type in `src/types.rs`:

```rust
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateWebhookRequest {
```

- [ ] **Step 3: Run tests**

Run: `cargo test webhooks::tests -- --nocapture 2>&1 | tail -5`
Expected: 3 tests pass

- [ ] **Step 4: Commit**

```bash
git add src/resources/webhooks.rs src/types.rs
git commit -m "feat: add webhooks resource — CRUD, test, deliveries"
```

---

### Task 12: Exports resource + tests

**Files:**
- Create: `src/resources/exports.rs`

- [ ] **Step 1: Write exports.rs**

```rust
use reqwest::Method;

use crate::client::Client;
use crate::error::Result;
use crate::response::{Response, ResponseMeta};
use crate::types::*;

pub struct Exports<'a> {
    client: &'a Client,
}

impl<'a> Exports<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Create a new export job.
    pub async fn create(&self, req: &CreateExportRequest) -> Result<Response<ExportJob>> {
        self.client
            .request_with_body(Method::POST, "/v1/exports", req)
            .await
    }

    /// Get export job status (with optional inline data if <10MB).
    pub async fn get(&self, id: &str) -> Result<Response<ExportDownload>> {
        self.client
            .request(Method::GET, &format!("/v1/exports/{id}"))
            .await
    }

    /// Download the export file as raw bytes.
    pub async fn download(&self, id: &str) -> Result<ExportFile> {
        let (bytes, meta, content_type, filename) = self
            .client
            .request_bytes(Method::GET, &format!("/v1/exports/{id}/download"))
            .await?;
        Ok(ExportFile {
            meta,
            bytes,
            content_type,
            filename,
        })
    }
}

/// Downloaded export file with raw bytes and metadata.
#[derive(Debug)]
pub struct ExportFile {
    pub meta: ResponseMeta,
    pub bytes: Vec<u8>,
    pub content_type: String,
    pub filename: String,
}

#[cfg(test)]
mod tests {
    use crate::{Client, CreateExportRequest};

    #[tokio::test]
    async fn test_exports_create() {
        let mut server = mockito::Server::new_async().await;
        let body = r#"{"id":"exp-123","status":"pending","format":"ndjson","total_rows":null,"file_size_bytes":null,"error_message":null,"created_at":"2026-03-30T12:00:00Z","completed_at":null,"expires_at":null}"#;
        let mock = server
            .mock("POST", "/v1/exports")
            .with_status(202)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create_async()
            .await;

        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();

        let req = CreateExportRequest {
            format: Some("ndjson".into()),
            canton: Some("ZH".into()),
            ..Default::default()
        };
        let resp = client.exports().create(&req).await.unwrap();
        assert_eq!(resp.data.id, "exp-123");
        assert_eq!(resp.data.status, "pending");

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_exports_get() {
        let mut server = mockito::Server::new_async().await;
        let body = r#"{"job":{"id":"exp-123","status":"completed","format":"ndjson","total_rows":100,"file_size_bytes":5000,"error_message":null,"created_at":"2026-03-30T12:00:00Z","completed_at":"2026-03-30T12:01:00Z","expires_at":"2026-04-06T12:01:00Z"},"data":null}"#;
        let mock = server
            .mock("GET", "/v1/exports/exp-123")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create_async()
            .await;

        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();

        let resp = client.exports().get("exp-123").await.unwrap();
        assert_eq!(resp.data.job.status, "completed");
        assert_eq!(resp.data.job.total_rows, Some(100));

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_exports_download() {
        let mut server = mockito::Server::new_async().await;
        let ndjson = "{\"uid\":\"CHE-100.023.968\",\"name\":\"Test AG\"}\n";
        let mock = server
            .mock("GET", "/v1/exports/exp-123/download")
            .with_status(200)
            .with_header("content-type", "application/x-ndjson; charset=utf-8")
            .with_header(
                "content-disposition",
                "attachment; filename=\"export-exp-123.ndjson\"",
            )
            .with_body(ndjson)
            .create_async()
            .await;

        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();

        let file = client.exports().download("exp-123").await.unwrap();
        assert!(file.content_type.contains("ndjson"));
        assert_eq!(file.filename, "export-exp-123.ndjson");
        assert!(!file.bytes.is_empty());
        let content = String::from_utf8(file.bytes).unwrap();
        assert!(content.contains("CHE-100.023.968"));

        mock.assert_async().await;
    }
}
```

- [ ] **Step 2: Add `Default` derive to `CreateExportRequest` in types.rs**

```rust
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateExportRequest {
```

- [ ] **Step 3: Run tests**

Run: `cargo test exports::tests -- --nocapture 2>&1 | tail -5`
Expected: 3 tests pass

- [ ] **Step 4: Commit**

```bash
git add src/resources/exports.rs src/types.rs
git commit -m "feat: add exports resource — create, poll, download"
```

---

### Task 13: AI resource + tests

**Files:**
- Create: `src/resources/ai.rs`

- [ ] **Step 1: Write ai.rs**

```rust
use reqwest::Method;

use crate::client::Client;
use crate::error::Result;
use crate::response::Response;
use crate::types::*;

pub struct Ai<'a> {
    client: &'a Client,
}

impl<'a> Ai<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Generate a due-diligence dossier for a company.
    pub async fn dossier(&self, req: &DossierRequest) -> Result<Response<DossierResponse>> {
        self.client
            .request_with_body(Method::POST, "/v1/ai/dossier", req)
            .await
    }

    /// AI-powered natural language search.
    pub async fn search(&self, req: &AiSearchRequest) -> Result<Response<AiSearchResponse>> {
        self.client
            .request_with_body(Method::POST, "/v1/ai/search", req)
            .await
    }

    /// Get a multi-signal risk score for a company.
    pub async fn risk_score(
        &self,
        req: &RiskScoreRequest,
    ) -> Result<Response<RiskScoreResponse>> {
        self.client
            .request_with_body(Method::POST, "/v1/ai/risk-score", req)
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::{Client, DossierRequest, AiSearchRequest, RiskScoreRequest};

    #[tokio::test]
    async fn test_ai_dossier() {
        let mut server = mockito::Server::new_async().await;
        let body = r#"{"uid":"CHE-100.023.968","company_name":"Test AG","dossier":"Test AG is a Swiss company...","sources":["zefix","seco"],"generated_at":"2026-03-30T12:00:00Z"}"#;
        let mock = server
            .mock("POST", "/v1/ai/dossier")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create_async()
            .await;

        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();

        let req = DossierRequest {
            uid: "CHE-100.023.968".into(),
            depth: Some("summary".into()),
        };
        let resp = client.ai().dossier(&req).await.unwrap();
        assert_eq!(resp.data.company_name, "Test AG");
        assert!(!resp.data.dossier.is_empty());
        assert_eq!(resp.data.sources.len(), 2);

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_ai_search() {
        let mut server = mockito::Server::new_async().await;
        let body = r#"{"query":"pharma companies in Zurich","explanation":"Searching for pharmaceutical companies in canton ZH","filters_applied":{"canton":"ZH","industry":"pharma"},"results":[{"uid":"CHE-100.023.968","name":"Test Pharma AG","canton":"ZH","status":"active"}],"total":1}"#;
        let mock = server
            .mock("POST", "/v1/ai/search")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create_async()
            .await;

        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();

        let req = AiSearchRequest {
            query: "pharma companies in Zurich".into(),
        };
        let resp = client.ai().search(&req).await.unwrap();
        assert_eq!(resp.data.total, 1);
        assert!(!resp.data.explanation.is_empty());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_ai_risk_score() {
        let mut server = mockito::Server::new_async().await;
        let body = r#"{"uid":"CHE-100.023.968","company_name":"Test AG","overall_score":25,"risk_level":"low","breakdown":[{"factor":"Sanctions Screening","score":0,"weight":0.35,"description":"No sanctions hits"}],"assessed_at":"2026-03-30T12:00:00Z"}"#;
        let mock = server
            .mock("POST", "/v1/ai/risk-score")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create_async()
            .await;

        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();

        let req = RiskScoreRequest {
            uid: "CHE-100.023.968".into(),
        };
        let resp = client.ai().risk_score(&req).await.unwrap();
        assert_eq!(resp.data.overall_score, 25);
        assert_eq!(resp.data.risk_level, "low");
        assert!(!resp.data.breakdown.is_empty());

        mock.assert_async().await;
    }
}
```

- [ ] **Step 2: Run tests**

Run: `cargo test ai::tests -- --nocapture 2>&1 | tail -5`
Expected: 3 tests pass

- [ ] **Step 3: Commit**

```bash
git add src/resources/ai.rs
git commit -m "feat: add AI resource — dossier, search, risk_score"
```

---

### Task 14: Rewrite blocking.rs

**Files:**
- Rewrite: `src/blocking.rs`

- [ ] **Step 1: Write new blocking.rs with 9 resource wrappers**

```rust
//! Synchronous (blocking) client for the VynCo API.
//!
//! Enabled via the `blocking` feature flag. Wraps the async [`crate::Client`]
//! with an internal single-threaded Tokio runtime.
//!
//! **Important:** Do not use from within an async context — the internal
//! `block_on()` will panic if a Tokio runtime is already running.

use std::time::Duration;

use crate::error::{Result, VyncoError};
use crate::resources::ExportFile;
use crate::response::{Response, ResponseMeta};
use crate::types::*;

/// Builder for configuring and constructing a blocking [`Client`].
pub struct ClientBuilder {
    api_key: String,
    base_url: Option<String>,
    timeout: Option<Duration>,
    max_retries: Option<u32>,
}

impl ClientBuilder {
    fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: None,
            timeout: None,
            max_retries: None,
        }
    }

    /// Set the API base URL.
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Set the request timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Set the maximum number of retries on 429/5xx.
    pub fn max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = Some(max_retries);
        self
    }

    /// Build the blocking client.
    pub fn build(self) -> Result<Client> {
        let mut builder = crate::Client::builder(self.api_key);
        if let Some(url) = self.base_url {
            builder = builder.base_url(url);
        }
        if let Some(timeout) = self.timeout {
            builder = builder.timeout(timeout);
        }
        if let Some(max_retries) = self.max_retries {
            builder = builder.max_retries(max_retries);
        }
        let inner = builder.build()?;

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| VyncoError::Config(format!("failed to create runtime: {e}")))?;

        Ok(Client { inner, rt })
    }
}

/// Synchronous client for the VynCo API.
///
/// Wraps the async client with a single-threaded Tokio runtime.
pub struct Client {
    inner: crate::Client,
    rt: tokio::runtime::Runtime,
}

impl Client {
    /// Create a new [`ClientBuilder`] with the given API key.
    pub fn builder(api_key: impl Into<String>) -> ClientBuilder {
        ClientBuilder::new(api_key)
    }

    fn block_on<F: std::future::Future>(&self, f: F) -> F::Output {
        self.rt.block_on(f)
    }

    pub fn health(&self) -> Health<'_> {
        Health { client: self }
    }

    pub fn companies(&self) -> Companies<'_> {
        Companies { client: self }
    }

    pub fn auditors(&self) -> Auditors<'_> {
        Auditors { client: self }
    }

    pub fn dashboard(&self) -> Dashboard<'_> {
        Dashboard { client: self }
    }

    pub fn screening(&self) -> Screening<'_> {
        Screening { client: self }
    }

    pub fn watchlists(&self) -> Watchlists<'_> {
        Watchlists { client: self }
    }

    pub fn webhooks(&self) -> Webhooks<'_> {
        Webhooks { client: self }
    }

    pub fn exports(&self) -> Exports<'_> {
        Exports { client: self }
    }

    pub fn ai(&self) -> Ai<'_> {
        Ai { client: self }
    }
}

// ---------------------------------------------------------------------------
// Resource wrappers
// ---------------------------------------------------------------------------

pub struct Health<'a> {
    client: &'a Client,
}

impl Health<'_> {
    pub fn check(&self) -> Result<Response<HealthResponse>> {
        self.client.block_on(self.client.inner.health().check())
    }
}

pub struct Companies<'a> {
    client: &'a Client,
}

impl Companies<'_> {
    pub fn list(&self, params: &CompanyListParams) -> Result<Response<PagedResponse<Company>>> {
        self.client
            .block_on(self.client.inner.companies().list(params))
    }

    pub fn get(&self, uid: &str) -> Result<Response<Company>> {
        self.client.block_on(self.client.inner.companies().get(uid))
    }

    pub fn count(&self) -> Result<Response<CompanyCount>> {
        self.client.block_on(self.client.inner.companies().count())
    }

    pub fn events(&self, uid: &str, limit: Option<u32>) -> Result<Response<EventListResponse>> {
        self.client
            .block_on(self.client.inner.companies().events(uid, limit))
    }
}

pub struct Auditors<'a> {
    client: &'a Client,
}

impl Auditors<'_> {
    pub fn history(&self, uid: &str) -> Result<Response<AuditorHistoryResponse>> {
        self.client
            .block_on(self.client.inner.auditors().history(uid))
    }

    pub fn tenures(
        &self,
        params: &AuditorTenureParams,
    ) -> Result<Response<PagedResponse<AuditorTenure>>> {
        self.client
            .block_on(self.client.inner.auditors().tenures(params))
    }
}

pub struct Dashboard<'a> {
    client: &'a Client,
}

impl Dashboard<'_> {
    pub fn get(&self) -> Result<Response<DashboardResponse>> {
        self.client.block_on(self.client.inner.dashboard().get())
    }
}

pub struct Screening<'a> {
    client: &'a Client,
}

impl Screening<'_> {
    pub fn screen(&self, req: &ScreeningRequest) -> Result<Response<ScreeningResponse>> {
        self.client
            .block_on(self.client.inner.screening().screen(req))
    }
}

pub struct Watchlists<'a> {
    client: &'a Client,
}

impl Watchlists<'_> {
    pub fn list(&self) -> Result<Response<Vec<WatchlistSummary>>> {
        self.client
            .block_on(self.client.inner.watchlists().list())
    }

    pub fn create(&self, req: &CreateWatchlistRequest) -> Result<Response<Watchlist>> {
        self.client
            .block_on(self.client.inner.watchlists().create(req))
    }

    pub fn delete(&self, id: &str) -> Result<ResponseMeta> {
        self.client
            .block_on(self.client.inner.watchlists().delete(id))
    }

    pub fn companies(&self, id: &str) -> Result<Response<WatchlistCompaniesResponse>> {
        self.client
            .block_on(self.client.inner.watchlists().companies(id))
    }

    pub fn add_companies(
        &self,
        id: &str,
        req: &AddCompaniesRequest,
    ) -> Result<Response<AddCompaniesResponse>> {
        self.client
            .block_on(self.client.inner.watchlists().add_companies(id, req))
    }

    pub fn remove_company(&self, id: &str, uid: &str) -> Result<ResponseMeta> {
        self.client
            .block_on(self.client.inner.watchlists().remove_company(id, uid))
    }

    pub fn events(&self, id: &str, limit: Option<u32>) -> Result<Response<EventListResponse>> {
        self.client
            .block_on(self.client.inner.watchlists().events(id, limit))
    }
}

pub struct Webhooks<'a> {
    client: &'a Client,
}

impl Webhooks<'_> {
    pub fn list(&self) -> Result<Response<Vec<WebhookSubscription>>> {
        self.client.block_on(self.client.inner.webhooks().list())
    }

    pub fn create(
        &self,
        req: &CreateWebhookRequest,
    ) -> Result<Response<CreateWebhookResponse>> {
        self.client
            .block_on(self.client.inner.webhooks().create(req))
    }

    pub fn update(
        &self,
        id: &str,
        req: &UpdateWebhookRequest,
    ) -> Result<Response<WebhookSubscription>> {
        self.client
            .block_on(self.client.inner.webhooks().update(id, req))
    }

    pub fn delete(&self, id: &str) -> Result<ResponseMeta> {
        self.client
            .block_on(self.client.inner.webhooks().delete(id))
    }

    pub fn test(&self, id: &str) -> Result<Response<TestDeliveryResponse>> {
        self.client
            .block_on(self.client.inner.webhooks().test(id))
    }

    pub fn deliveries(
        &self,
        id: &str,
        limit: Option<u32>,
    ) -> Result<Response<Vec<WebhookDelivery>>> {
        self.client
            .block_on(self.client.inner.webhooks().deliveries(id, limit))
    }
}

pub struct Exports<'a> {
    client: &'a Client,
}

impl Exports<'_> {
    pub fn create(&self, req: &CreateExportRequest) -> Result<Response<ExportJob>> {
        self.client
            .block_on(self.client.inner.exports().create(req))
    }

    pub fn get(&self, id: &str) -> Result<Response<ExportDownload>> {
        self.client
            .block_on(self.client.inner.exports().get(id))
    }

    pub fn download(&self, id: &str) -> Result<ExportFile> {
        self.client
            .block_on(self.client.inner.exports().download(id))
    }
}

pub struct Ai<'a> {
    client: &'a Client,
}

impl Ai<'_> {
    pub fn dossier(&self, req: &DossierRequest) -> Result<Response<DossierResponse>> {
        self.client.block_on(self.client.inner.ai().dossier(req))
    }

    pub fn search(&self, req: &AiSearchRequest) -> Result<Response<AiSearchResponse>> {
        self.client.block_on(self.client.inner.ai().search(req))
    }

    pub fn risk_score(&self, req: &RiskScoreRequest) -> Result<Response<RiskScoreResponse>> {
        self.client
            .block_on(self.client.inner.ai().risk_score(req))
    }
}
```

- [ ] **Step 2: Verify compilation**

Run: `cargo check --features blocking 2>&1 | head -5`
Expected: Compiles successfully

- [ ] **Step 3: Commit**

```bash
git add src/blocking.rs
git commit -m "feat: rewrite blocking client for v2 — 9 resource wrappers"
```

---

### Task 15: Rewrite example CLI

**Files:**
- Rewrite: `examples/vynco_cli.rs`

- [ ] **Step 1: Write new example CLI**

```rust
//! # VynCo CLI Example
//!
//! A command-line tool demonstrating the VynCo Rust SDK.
//!
//! ## Usage
//!
//! ```bash
//! export VYNCO_API_KEY="vc_live_your_api_key"
//!
//! cargo run --example vynco_cli -- health
//! cargo run --example vynco_cli -- companies --canton ZH --search "Novartis"
//! cargo run --example vynco_cli -- company CHE-105.805.080
//! cargo run --example vynco_cli -- count
//! cargo run --example vynco_cli -- events CHE-105.805.080
//! cargo run --example vynco_cli -- screen "Novartis AG"
//! cargo run --example vynco_cli -- dashboard
//! cargo run --example vynco_cli -- auditors --min-years 10
//! cargo run --example vynco_cli -- risk CHE-105.805.080
//! ```

use clap::{Parser, Subcommand};
use vynco::{Client, CompanyListParams, AuditorTenureParams, VyncoError};

#[derive(Parser)]
#[command(name = "vynco")]
#[command(about = "VynCo Swiss Corporate Intelligence CLI — example for the vynco Rust SDK")]
struct Cli {
    /// API key (overrides VYNCO_API_KEY env var)
    #[arg(long, env = "VYNCO_API_KEY")]
    api_key: String,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Check API health status
    Health,

    /// List companies with optional filters
    Companies {
        /// Filter by Swiss canton (e.g. ZH, BE, GE)
        #[arg(long)]
        canton: Option<String>,

        /// Filter by name (substring match)
        #[arg(long)]
        search: Option<String>,

        /// Page number (default: 1)
        #[arg(long, default_value = "1")]
        page: i64,

        /// Page size (default: 10)
        #[arg(long, default_value = "10")]
        page_size: i64,
    },

    /// Get a specific company by UID (e.g. CHE-105.805.080)
    Company {
        /// Company UID
        uid: String,
    },

    /// Count total companies in the database
    Count,

    /// List events for a company
    Events {
        /// Company UID
        uid: String,

        /// Maximum events to return
        #[arg(long, default_value = "10")]
        limit: u32,
    },

    /// Screen an entity against sanctions databases
    Screen {
        /// Entity name to screen
        name: String,

        /// Optional UID for exact matching
        #[arg(long)]
        uid: Option<String>,
    },

    /// Show dashboard overview
    Dashboard,

    /// List long-tenure auditors
    Auditors {
        /// Minimum tenure years (default: 7)
        #[arg(long, default_value = "7")]
        min_years: f64,

        /// Filter by canton
        #[arg(long)]
        canton: Option<String>,
    },

    /// Get AI risk score for a company
    Risk {
        /// Company UID
        uid: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let client = match Client::builder(&cli.api_key).build() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to create client: {e}");
            std::process::exit(1);
        }
    };

    if let Err(e) = run(client, cli.command).await {
        eprintln!("\nError: {e}");
        match &e {
            VyncoError::Authentication(_) => {
                eprintln!("Hint: Check your VYNCO_API_KEY or pass --api-key");
            }
            VyncoError::RateLimit(_) => {
                eprintln!("Hint: You've hit the rate limit — wait a moment and retry");
            }
            _ => {}
        }
        std::process::exit(1);
    }
}

async fn run(client: Client, command: Command) -> Result<(), VyncoError> {
    match command {
        Command::Health => {
            let resp = client.health().check().await?;
            println!("Status:    {}", resp.data.status);
            println!("Database:  {}", resp.data.database);
            println!("Redis:     {}", resp.data.redis);
            println!("Version:   {}", resp.data.version);
            print_meta(&resp.meta);
        }

        Command::Companies {
            canton,
            search,
            page,
            page_size,
        } => {
            let params = CompanyListParams {
                page: Some(page),
                page_size: Some(page_size),
                canton,
                search,
                ..Default::default()
            };
            let resp = client.companies().list(&params).await?;
            let total_pages = ((resp.data.total as f64) / (resp.data.page_size as f64)).ceil() as i64;
            println!(
                "Companies: page {}/{} ({} total)\n",
                resp.data.page, total_pages, resp.data.total,
            );
            for c in &resp.data.items {
                println!(
                    "  {:<18} {:<45} {:<6} {}",
                    c.uid,
                    c.name,
                    c.canton.as_deref().unwrap_or("-"),
                    c.status.as_deref().unwrap_or("-"),
                );
            }
            print_meta(&resp.meta);
        }

        Command::Company { uid } => {
            let resp = client.companies().get(&uid).await?;
            let c = &resp.data;
            println!("UID:          {}", c.uid);
            println!("Name:         {}", c.name);
            println!("Legal form:   {}", c.legal_form.as_deref().unwrap_or("-"));
            println!("Status:       {}", c.status.as_deref().unwrap_or("-"));
            println!("Canton:       {}", c.canton.as_deref().unwrap_or("-"));
            println!("Industry:     {}", c.industry.as_deref().unwrap_or("-"));
            println!("Auditor cat:  {}", c.auditor_category.as_deref().unwrap_or("-"));
            if let Some(cap) = c.share_capital {
                println!("Share capital: CHF {:.2}", cap);
            }
            if let Some(ref d) = c.updated_at {
                println!("Updated:      {}", d);
            }
            print_meta(&resp.meta);
        }

        Command::Count => {
            let resp = client.companies().count().await?;
            println!("Total companies: {}", resp.data.count);
            print_meta(&resp.meta);
        }

        Command::Events { uid, limit } => {
            let resp = client.companies().events(&uid, Some(limit)).await?;
            println!("Events for {} ({} total):\n", uid, resp.data.count);
            for ev in &resp.data.events {
                println!(
                    "  {} {:<25} {:<8} {}",
                    &ev.ce_time[..10.min(ev.ce_time.len())],
                    ev.category,
                    ev.severity,
                    ev.summary,
                );
            }
            print_meta(&resp.meta);
        }

        Command::Screen { name, uid } => {
            let req = vynco::ScreeningRequest {
                name: name.clone(),
                uid,
                sources: None,
            };
            let resp = client.screening().screen(&req).await?;
            println!("Screening: {}", name);
            println!("Risk level: {}", resp.data.risk_level);
            println!("Hits: {}", resp.data.hit_count);
            println!("Sources: {}", resp.data.sources_checked.join(", "));
            for hit in &resp.data.hits {
                println!(
                    "\n  {} (score: {:.2})",
                    hit.matched_name, hit.score,
                );
                println!("    Source: {}, Type: {}", hit.source, hit.entity_type);
            }
            print_meta(&resp.meta);
        }

        Command::Dashboard => {
            let resp = client.dashboard().get().await?;
            let d = &resp.data;
            println!("Dashboard ({})\n", d.generated_at);
            println!("Data completeness: {:.1}%", d.data.completeness_pct);
            println!("  Total companies: {}", d.data.total_companies);
            println!("  With canton:     {}", d.data.with_canton);
            println!("  With auditor:    {}", d.data.with_auditor);
            println!("\nAuditor tenures:");
            println!("  Total: {}", d.auditor_tenures.total_tenures);
            println!("  Long (7+ yrs): {}", d.auditor_tenures.long_tenures_7plus);
            println!(
                "  Avg tenure: {:.1} yrs (max: {:.1})",
                d.auditor_tenures.avg_tenure_years, d.auditor_tenures.max_tenure_years
            );
            if !d.pipelines.is_empty() {
                println!("\nPipelines:");
                for p in &d.pipelines {
                    println!("  {:<20} {}", p.name, p.status);
                }
            }
            print_meta(&resp.meta);
        }

        Command::Auditors { min_years, canton } => {
            let params = AuditorTenureParams {
                min_years: Some(min_years),
                canton,
                page: Some(1),
                page_size: Some(20),
            };
            let resp = client.auditors().tenures(&params).await?;
            println!(
                "Long-tenure auditors (>= {:.0} yrs): {} total\n",
                min_years, resp.data.total
            );
            for t in &resp.data.items {
                println!(
                    "  {:<45} {:<30} {:.1} yrs",
                    t.company_name,
                    t.auditor_name,
                    t.tenure_years.unwrap_or(0.0),
                );
            }
            print_meta(&resp.meta);
        }

        Command::Risk { uid } => {
            let req = vynco::RiskScoreRequest { uid: uid.clone() };
            let resp = client.ai().risk_score(&req).await?;
            let r = &resp.data;
            println!("Risk Score: {} ({})", r.company_name, r.uid);
            println!("Overall:    {}/100 ({})", r.overall_score, r.risk_level);
            println!("\nBreakdown:");
            for f in &r.breakdown {
                println!(
                    "  {:<25} {:>3}/100  (weight: {:.0}%)  {}",
                    f.factor,
                    f.score,
                    f.weight * 100.0,
                    f.description,
                );
            }
            print_meta(&resp.meta);
        }
    }

    Ok(())
}

fn print_meta(meta: &vynco::ResponseMeta) {
    println!("\n--- Response metadata ---");
    if let Some(ref id) = meta.request_id {
        println!("Request ID:          {id}");
    }
    if let Some(used) = meta.credits_used {
        println!("Credits used:        {used}");
    }
    if let Some(remaining) = meta.credits_remaining {
        println!("Credits remaining:   {remaining}");
    }
    if let Some(limit) = meta.rate_limit_limit {
        println!("Rate limit:          {limit}/min");
    }
    if let Some(remaining) = meta.rate_limit_remaining {
        println!("Rate limit remaining:{remaining}");
    }
    if let Some(ref source) = meta.data_source {
        println!("Data source:         {source}");
    }
}
```

- [ ] **Step 2: Verify example compiles**

Run: `cargo check --example vynco_cli 2>&1 | head -5`
Expected: Compiles successfully

- [ ] **Step 3: Commit**

```bash
git add examples/vynco_cli.rs
git commit -m "feat: rewrite example CLI for v2 — health, companies, screening, AI"
```

---

### Task 16: Update README.md

**Files:**
- Rewrite: `README.md`

- [ ] **Step 1: Write new README.md**

```markdown
# vynco

[![Crates.io](https://img.shields.io/crates/v/vynco.svg)](https://crates.io/crates/vynco)
[![Documentation](https://docs.rs/vynco/badge.svg)](https://docs.rs/vynco)
[![CI](https://github.com/VynCorp/vc-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/VynCorp/vc-rust/actions/workflows/ci.yml)
[![License: Apache-2.0](https://img.shields.io/crates/l/vynco.svg)](LICENSE)

Rust SDK for the [VynCo](https://vynco.ch) Swiss Corporate Intelligence API.

Access 500,000+ Swiss companies with event tracking, sanctions screening,
AI-powered analysis, watchlists, webhooks, and bulk data exports.

## Installation

```toml
[dependencies]
vynco = "2.0"
```

For the synchronous (blocking) client:

```toml
[dependencies]
vynco = { version = "2.0", features = ["blocking"] }
```

## Quick Start

```rust
use vynco::{Client, CompanyListParams};

#[tokio::main]
async fn main() -> Result<(), vynco::VyncoError> {
    let client = Client::builder("vc_live_your_api_key")
        .build()?;

    // List companies with filtering
    let params = CompanyListParams {
        search: Some("Novartis".into()),
        canton: Some("BS".into()),
        ..Default::default()
    };
    let resp = client.companies().list(&params).await?;
    println!("Found {} companies", resp.data.total);

    // Get company by UID
    let company = client.companies().get("CHE-105.805.080").await?;
    println!("{}", company.data.name);

    // Screen against sanctions databases
    let screening = client.screening().screen(&vynco::ScreeningRequest {
        name: "Test Corp".into(),
        uid: None,
        sources: None,
    }).await?;
    println!("Risk level: {}", screening.data.risk_level);

    // AI risk score
    let risk = client.ai().risk_score(&vynco::RiskScoreRequest {
        uid: "CHE-105.805.080".into(),
    }).await?;
    println!("Risk: {}/100 ({})", risk.data.overall_score, risk.data.risk_level);

    Ok(())
}
```

### Blocking Client

```rust
use vynco::blocking::Client;

fn main() -> Result<(), vynco::VyncoError> {
    let client = Client::builder("vc_live_your_api_key")
        .build()?;

    let resp = client.companies().count()?;
    println!("Total companies: {}", resp.data.count);

    Ok(())
}
```

## API Coverage

9 resource modules covering 28 endpoints:

| Resource | Methods |
|----------|---------|
| `health()` | `check` |
| `companies()` | `list`, `get`, `count`, `events` |
| `auditors()` | `history`, `tenures` |
| `dashboard()` | `get` |
| `screening()` | `screen` |
| `watchlists()` | `list`, `create`, `delete`, `companies`, `add_companies`, `remove_company`, `events` |
| `webhooks()` | `list`, `create`, `update`, `delete`, `test`, `deliveries` |
| `exports()` | `create`, `get`, `download` |
| `ai()` | `dossier`, `search`, `risk_score` |

## Response Metadata

Every response includes header metadata for credit tracking and rate limiting:

```rust
let resp = client.companies().get("CHE-105.805.080").await?;

println!("Request ID: {:?}", resp.meta.request_id);
println!("Credits used: {:?}", resp.meta.credits_used);
println!("Rate limit: {:?}", resp.meta.rate_limit_limit);
println!("Rate limit remaining: {:?}", resp.meta.rate_limit_remaining);
```

## Example CLI

```bash
export VYNCO_API_KEY="vc_live_your_api_key"

cargo run --example vynco_cli -- health
cargo run --example vynco_cli -- companies --canton ZH --search "Novartis"
cargo run --example vynco_cli -- company CHE-105.805.080
cargo run --example vynco_cli -- count
cargo run --example vynco_cli -- events CHE-105.805.080
cargo run --example vynco_cli -- screen "Novartis AG"
cargo run --example vynco_cli -- dashboard
cargo run --example vynco_cli -- auditors --min-years 10
cargo run --example vynco_cli -- risk CHE-105.805.080
```

See [`examples/vynco_cli.rs`](examples/vynco_cli.rs) for the full source.

## Configuration

```rust
use std::time::Duration;

let client = Client::builder("vc_live_your_api_key")
    .base_url("https://api.vynco.ch")  // default
    .timeout(Duration::from_secs(60))   // default: 30s
    .max_retries(3)                     // default: 2
    .build()?;
```

The client automatically retries on HTTP 429 (rate limited) and 5xx (server error) with
exponential backoff (500ms x 2^attempt). It respects the `Retry-After` header when present.

## Error Handling

All API errors are mapped to typed variants:

```rust
use vynco::VyncoError;

match client.companies().get("CHE-000.000.000").await {
    Ok(resp) => println!("{}", resp.data.name),
    Err(VyncoError::Authentication(_)) => println!("Invalid API key"),
    Err(VyncoError::InsufficientCredits(_)) => println!("Top up credits"),
    Err(VyncoError::Forbidden(_)) => println!("Insufficient permissions"),
    Err(VyncoError::NotFound(body)) => println!("Not found: {:?}", body.detail),
    Err(VyncoError::Validation(body)) => println!("Bad request: {:?}", body.detail),
    Err(VyncoError::RateLimit(_)) => println!("Rate limited, retry later"),
    Err(VyncoError::Server(_)) => println!("Server error"),
    Err(e) => eprintln!("Error: {e}"),
}
```

Error bodies follow [RFC 7807 Problem Details](https://tools.ietf.org/html/rfc7807) with
`error_type`, `title`, `status`, `detail`, and `instance` fields.

## Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `rustls-tls` | Yes | Use rustls for TLS (no OpenSSL dependency) |
| `native-tls` | No | Use the platform's native TLS stack |
| `blocking` | No | Enable the synchronous blocking client |

## Minimum Supported Rust Version

Rust 1.83 or later.

## License

Apache-2.0
```

- [ ] **Step 2: Commit**

```bash
git add README.md
git commit -m "docs: rewrite README for v2 API coverage"
```

---

### Task 17: Update CLAUDE.md

**Files:**
- Modify: `CLAUDE.md`

- [ ] **Step 1: Update the architecture section and resource table**

Key changes:
- Base URL: `https://api.vynco.ch/api/v1` → `https://api.vynco.ch`
- Health path: `/health` (no `/v1` prefix)
- All other paths: `/v1/...`
- Remove reference to `extract_list<T>()`
- Remove `camelCase` serde convention — API now uses snake_case
- Update resource table from 14 modules / 52 endpoints to 9 modules / 28 endpoints
- Update serde conventions section
- Update API base URL

Replace the Resources table:

```markdown
### Resources (9 modules, 28 endpoints)

| Resource | Endpoints |
|----------|-----------|
| `health` | `check` |
| `companies` | `list`, `get`, `count`, `events` |
| `auditors` | `history`, `tenures` |
| `dashboard` | `get` |
| `screening` | `screen` |
| `watchlists` | `list`, `create`, `delete`, `companies`, `add_companies`, `remove_company`, `events` |
| `webhooks` | `list`, `create`, `update`, `delete`, `test`, `deliveries` |
| `exports` | `create`, `get`, `download` |
| `ai` | `dossier`, `search`, `risk_score` |
```

Update serde conventions:
```markdown
### Serde Conventions

- No `rename_all` needed — API uses snake_case natively (matching Rust field names)
- `#[serde(default)]` on fields that may be absent
- `#[serde(skip_serializing_if = "Option::is_none")]` on optional request params
```

Update Base URL:
```markdown
- **Base URL:** `https://api.vynco.ch` (health at `/health`, all other endpoints at `/v1/...`)
```

Remove the "Flexible list extraction" line about `Client::extract_list<T>()`.

Update the four request methods to include `request_bytes()`:
```markdown
**Five request methods on Client** (all `pub(crate)`):
- `request<T>()` — GET with no body
- `request_with_body<T, B>()` — POST/PUT with JSON body
- `request_with_params<T>()` — GET with query parameters
- `request_empty()` — DELETE, returns only `ResponseMeta`
- `request_bytes()` — GET returning raw bytes (for export downloads)
```

- [ ] **Step 2: Commit**

```bash
git add CLAUDE.md
git commit -m "docs: update CLAUDE.md for v2 architecture"
```

---

### Task 18: Update CHANGELOG.md

**Files:**
- Modify: `CHANGELOG.md`

- [ ] **Step 1: Add v2.0.0 entry at the top (after `## [Unreleased]`)**

Replace the `[Unreleased]` section and add v2.0.0:

```markdown
## [Unreleased]

## [2.0.0] - 2026-03-30

Major version update — SDK rewritten to align with the new Rust-based VynCo API.

### Added

- **6 new resource modules** (22 endpoints):
  - `auditors()` — history, tenures
  - `dashboard()` — get
  - `screening()` — screen (SECO, FINMA, OpenSanctions)
  - `watchlists()` — list, create, delete, companies, add_companies, remove_company, events
  - `webhooks()` — list, create, update, delete, test, deliveries
  - `exports()` — create, get, download (raw CSV/NDJSON)
  - `ai()` — dossier, search, risk_score
- **`Client::request_bytes()`** internal method for binary file downloads
- **`ExportFile`** type for downloaded export data with content-type and filename
- **Company events** via `companies().events(uid, limit)` (CloudEvents format)
- **`ErrorBody.instance`** field (RFC 7807)

### Changed

- **Base URL**: `https://api.vynco.ch/api/v1` → `https://api.vynco.ch`
- **Serde**: Dropped `rename_all = "camelCase"` — API now uses snake_case natively
- **`HealthResponse`**: `status`, `database`, `redis`, `version` (was `status`, `uptime`, `checks`)
- **`Company`**: Removed `address`, `purpose`; added `share_capital`, `industry`; fields now `Option<String>`
- **`PagedResponse<T>`**: `total_count: u64` → `total: i64`, `page_size: u32` → `page_size: i64`
- **`CompanyListParams`**: Removed `status`, `auditor_category`, `sort_by`, `sort_desc`, `target_status`; added `changed_since`; page types changed to `i64`
- **`ErrorBody`**: `detail: String` → `detail: Option<String>`; removed `message`; added `instance: Option<String>`
- **Companies `count()`**: No longer takes `CompanyCountParams` (API has no count filters)

### Removed

- **12 resource modules**: `analytics`, `api_keys`, `billing`, `changes`, `credits`, `dossiers`, `news`, `persons`, `relationships`, `reports`, `teams`, `watches`
- **`Client::extract_list()`** — no longer needed (API uses consistent response shapes)
- **All v1 types**: `CompanyStatistics`, `CompanyChange`, `ChangeStatistics`, `Person`, `Dossier`, `CompanyRelationship`, `CompanyWatch`, `ChangeNotification`, `ApiKey`, `CreditBalance`, `Team`, `BillingSummary`, and all associated request/param types
```

Update the version links at the bottom of the file:

```markdown
[2.0.0]: https://github.com/VynCorp/vc-rust/compare/v1.0.0...v2.0.0
[1.0.0]: https://github.com/VynCorp/vc-rust/compare/v0.1.0...v1.0.0
[0.1.0]: https://github.com/VynCorp/vc-rust/releases/tag/v0.1.0
```

- [ ] **Step 2: Commit**

```bash
git add CHANGELOG.md
git commit -m "docs: add v2.0.0 changelog entry"
```

---

### Task 19: Final verification and build

- [ ] **Step 1: Run full build**

Run: `cargo build 2>&1 | tail -5`
Expected: Compiles with no errors

- [ ] **Step 2: Run full build with blocking feature**

Run: `cargo build --features blocking 2>&1 | tail -5`
Expected: Compiles with no errors

- [ ] **Step 3: Run all tests**

Run: `cargo test 2>&1 | tail -20`
Expected: All tests pass (25+ tests)

- [ ] **Step 4: Run cargo fmt**

Run: `cargo fmt`

- [ ] **Step 5: Run cargo clippy**

Run: `cargo clippy --all-features 2>&1 | tail -10`
Expected: No warnings (or only minor ones)

- [ ] **Step 6: Dry-run publish**

Run: `cargo publish --dry-run 2>&1 | tail -10`
Expected: Packaged successfully

- [ ] **Step 7: Fix any issues found in steps 1-6**

Address any compilation errors, test failures, formatting issues, or clippy warnings.

- [ ] **Step 8: Commit final fixes (if any)**

```bash
git add -A
git commit -m "chore: final v2.0.0 polish — fmt, clippy, publish dry-run"
```

---

## Parallelization Notes

- **Tasks 5-13** (resource modules) are fully independent and can be executed in parallel after Task 4 completes.
- **Task 14** (blocking) depends on all resource modules (Tasks 5-13).
- **Task 15** (example CLI) depends on Task 14 (blocking) only if testing blocking, but can start after Tasks 5-13.
- **Tasks 16-18** (docs) are independent of each other but depend on Tasks 5-13 for accuracy.
- **Task 19** (verification) must be last.

```
Tasks 1-4 (sequential) → Tasks 5-13 (parallel) → Tasks 14-18 (parallel) → Task 19
```
