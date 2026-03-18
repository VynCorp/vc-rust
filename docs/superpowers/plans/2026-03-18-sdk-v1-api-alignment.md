# VynCo SDK v1.0 API Alignment Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Update the `vynco` Rust SDK from draft (v0.1.0) to match the finalized VynCo OpenAPI 1.0.0 spec — new endpoints, corrected models, updated paths, and production-ready polish.

**Architecture:** Same core architecture (Client builder → Resource borrows &Client → Response<T> wrapper). Changes are: base URL fix, 3 resources removed (webhooks/users/settings), 7 new resources added (changes/analytics/watches/news/reports/relationships/health), existing resources updated to match final API paths and schemas. Blocking client mirrors all async changes.

**Tech Stack:** Rust 2021, reqwest 0.12, serde 1, tokio 1, mockito 1 (dev), thiserror 2

**OpenAPI spec:** `/home/michael/DEV/Repos/ZefixMiner/EY.EW.ASU.ZefixMiner/src/ZefixMiner.Functions.Api/openapi.json`

---

## File Structure

### Files to Modify
- `src/client.rs` — base URL change, new resource accessors, remove old accessors
- `src/error.rs` — add ProblemDetails fields, Conflict variant
- `src/response.rs` — add rate limit remaining/reset headers
- `src/types.rs` — complete model overhaul
- `src/lib.rs` — update re-exports
- `src/blocking.rs` — mirror all async resource changes
- `src/resources/mod.rs` — update module list
- `src/resources/companies.rs` — restructure endpoints
- `src/resources/persons.rs` — expand to 6 endpoints
- `src/resources/dossiers.rs` — expand to 4 endpoints
- `src/resources/api_keys.rs` — update types
- `src/resources/credits.rs` — typed history response
- `src/resources/billing.rs` — fix endpoint paths
- `src/resources/teams.rs` — expand to 7 endpoints
- `tests/test_client.rs` — update all tests for new types/paths
- `Cargo.toml` — bump version to 1.0.0
- `README.md` — update for new API surface
- `CHANGELOG.md` — document all changes
- `CLAUDE.md` — update architecture docs

### Files to Create
- `src/resources/changes.rs` — 6 change-tracking endpoints
- `src/resources/analytics.rs` — 7 analytics endpoints
- `src/resources/watches.rs` — 4 watch/notification endpoints
- `src/resources/news.rs` — 2 news endpoints
- `src/resources/reports.rs` — 1 reports endpoint
- `src/resources/relationships.rs` — 2 relationship endpoints
- `src/resources/health.rs` — 1 health check endpoint

### Files to Delete
- `src/resources/webhooks.rs` — replaced by watches
- `src/resources/users.rs` — not in final API
- `src/resources/settings.rs` — not in final API

---

## Task 1: Core Infrastructure Updates

**Files:**
- Modify: `src/client.rs`
- Modify: `src/error.rs`
- Modify: `src/response.rs`

- [ ] **Step 1: Update base URL in client.rs**

Change `DEFAULT_BASE_URL` from `"https://api.vynco.ch/v1"` to `"https://api.vynco.ch/api/v1"`.

- [ ] **Step 2: Update error.rs — add ProblemDetails fields and Conflict variant**

Update `ErrorBody` to include `type` and `title` fields from RFC 7807 ProblemDetails:

```rust
#[derive(Debug, Clone, Deserialize)]
pub struct ErrorBody {
    #[serde(default, rename = "type")]
    pub error_type: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub detail: String,
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub status: u16,
}
```

Add `Conflict` variant to `VyncoError`:

```rust
#[error("conflict: {0}")]
Conflict(ErrorBody),
```

Add 409 mapping in `client.rs::map_error`:

```rust
StatusCode::CONFLICT => VyncoError::Conflict(body),
```

- [ ] **Step 3: Update response.rs — add rate limit headers**

Add two new fields to `ResponseMeta`:

```rust
pub rate_limit_remaining: Option<u32>,  // X-RateLimit-Remaining
pub rate_limit_reset: Option<u64>,      // X-RateLimit-Reset (unix timestamp)
```

Parse them in `from_headers`:

```rust
rate_limit_remaining: headers
    .get("X-RateLimit-Remaining")
    .and_then(|v| v.to_str().ok())
    .and_then(|v| v.parse().ok()),
rate_limit_reset: headers
    .get("X-RateLimit-Reset")
    .and_then(|v| v.to_str().ok())
    .and_then(|v| v.parse().ok()),
```

- [ ] **Step 4: Verify compilation**

Run: `cargo build 2>&1 | head -20`
Expected: Build succeeds (tests may fail, that's OK for now)

- [ ] **Step 5: Commit**

```bash
git add src/client.rs src/error.rs src/response.rs
git commit -m "fix: update base URL to /api/v1, add ProblemDetails fields and rate limit headers"
```

---

## Task 2: Complete Model Types Overhaul

**Files:**
- Modify: `src/types.rs`

Replace the entire `types.rs` with models matching the OpenAPI spec. All types use `#[serde(rename_all = "camelCase")]` and `#[serde(default)]` on optional fields.

- [ ] **Step 1: Write new types.rs**

The complete type definitions (see OpenAPI `components/schemas`):

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// --- Pagination ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    #[serde(default)]
    pub total_count: u64,
    #[serde(default)]
    pub page: u32,
    #[serde(default)]
    pub page_size: u32,
}

// --- Companies ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Company {
    pub uid: String,
    pub name: String,
    #[serde(default)]
    pub legal_form: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub canton: String,
    #[serde(default)]
    pub address: String,
    #[serde(default)]
    pub purpose: String,
    #[serde(default)]
    pub auditor_category: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

/// Query parameters for listing companies (GET /companies).
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanyListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canton: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auditor_category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_desc: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_status: Option<String>,
}

/// Request body for full-text search (POST /companies/search).
#[derive(Debug, Clone, Serialize)]
pub struct CompanySearchRequest {
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Request body for batch lookup (POST /companies/batch).
#[derive(Debug, Clone, Serialize)]
pub struct BatchCompanyRequest {
    pub uids: Vec<String>,
}

/// Request body for company comparison (POST /companies/compare).
#[derive(Debug, Clone, Serialize)]
pub struct CompareCompaniesRequest {
    pub uids: Vec<String>,
}

/// Company count response from GET /companies/count.
#[derive(Debug, Clone, Deserialize)]
pub struct CompanyCount {
    pub count: u64,
}

/// Company count query parameters.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanyCountParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canton: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auditor_category: Option<String>,
}

/// Company statistics from GET /companies/statistics.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanyStatistics {
    #[serde(default)]
    pub total_count: u64,
    #[serde(default)]
    pub enriched_count: u64,
    #[serde(default)]
    pub canton_counts: HashMap<String, u64>,
    #[serde(default)]
    pub auditor_category_counts: HashMap<String, u64>,
}

// --- Changes ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanyChange {
    pub id: String,
    pub company_uid: String,
    #[serde(default)]
    pub company_name: String,
    #[serde(default)]
    pub change_type: String,
    #[serde(default)]
    pub field_name: String,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub sogc_id: Option<String>,
    #[serde(default)]
    pub is_reviewed: bool,
    #[serde(default)]
    pub is_flagged: bool,
    #[serde(default)]
    pub detected_at: String,
}

/// Query parameters for listing changes (GET /changes).
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_uid: Option<String>,
}

/// Change statistics from GET /changes/statistics.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeStatistics {
    #[serde(default)]
    pub total_count: u64,
    #[serde(default)]
    pub reviewed_count: u64,
    #[serde(default)]
    pub flagged_count: u64,
    #[serde(default)]
    pub change_type_counts: HashMap<String, u64>,
}

/// Request body for reviewing a change (PUT /changes/{id}/review).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewChangeRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_notes: Option<String>,
}

/// Response from reviewing a change.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewChangeResponse {
    pub reviewed: bool,
    pub change_id: String,
}

/// Request body for batch change fetch (POST /changes/batch).
#[derive(Debug, Clone, Serialize)]
pub struct BatchChangeRequest {
    pub uids: Vec<String>,
}

// --- Persons ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub roles: Vec<String>,
    #[serde(default)]
    pub companies: Vec<String>,
}

/// Query parameters for listing persons (GET /persons).
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
}

// --- Dossiers ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dossier {
    pub company_uid: String,
    #[serde(default)]
    pub company_name: String,
    pub summary: Option<String>,
    pub risk_score: Option<f64>,
    pub generated_at: Option<String>,
}

/// Request body for generating a dossier (POST /dossiers/{uid}/generate).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateDossierRequest {
    /// Dossier type: "standard" or "comprehensive".
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub dossier_type: Option<String>,
}

// --- Relationships ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanyRelationship {
    pub id: String,
    pub source_company_uid: String,
    #[serde(default)]
    pub source_company_name: String,
    pub target_company_uid: String,
    #[serde(default)]
    pub target_company_name: String,
    #[serde(default)]
    pub relationship_type: String,
    pub source_lei: Option<String>,
    pub target_lei: Option<String>,
    #[serde(default)]
    pub data_source: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    #[serde(default)]
    pub is_active: bool,
}

/// Response wrapper for relationship endpoints.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipResponse {
    pub company_uid: String,
    #[serde(default)]
    pub total: u64,
    #[serde(default)]
    pub relationships: Vec<CompanyRelationship>,
}

// --- News ---

/// Response wrapper for company news.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanyNewsResponse {
    pub company_uid: String,
    #[serde(default)]
    pub count: u64,
    #[serde(default)]
    pub items: Vec<serde_json::Value>,
}

/// Response wrapper for recent news.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentNewsResponse {
    #[serde(default)]
    pub count: u64,
    #[serde(default)]
    pub items: Vec<serde_json::Value>,
}

// --- Reports ---

/// Response wrapper for company reports.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanyReportsResponse {
    pub company_uid: String,
    #[serde(default)]
    pub count: u64,
    #[serde(default)]
    pub reports: Vec<serde_json::Value>,
}

// --- Watches & Notifications ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanyWatch {
    pub id: String,
    pub team_id: String,
    pub company_uid: String,
    #[serde(default)]
    pub company_name: String,
    #[serde(default)]
    pub channel: String,
    pub webhook_url: Option<String>,
    #[serde(default)]
    pub watched_change_types: Vec<String>,
    #[serde(default)]
    pub created_at: String,
}

/// Request body for adding a company watch.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWatchRequest {
    pub company_uid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watched_change_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeNotification {
    pub id: String,
    pub company_uid: String,
    #[serde(default)]
    pub company_name: String,
    pub change_id: String,
    #[serde(default)]
    pub change_type: String,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub channel: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub created_at: String,
    pub sent_at: Option<String>,
}

// --- Analytics ---

/// Request body for K-Means clustering (POST /analytics/cluster).
#[derive(Debug, Clone, Serialize)]
pub struct ClusterRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canton: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Request body for anomaly detection (POST /analytics/anomalies).
#[derive(Debug, Clone, Serialize)]
pub struct AnomalyRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canton: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Query parameters for cohort analytics (GET /analytics/cohorts).
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CohortParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canton: Option<String>,
}

// --- API Keys ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKey {
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub prefix: String,
    #[serde(default)]
    pub is_test_key: bool,
    #[serde(default)]
    pub created_at: String,
    pub last_used_at: Option<String>,
}

/// A newly created API key (includes the full key value, shown only once).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyCreated {
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub prefix: String,
    #[serde(default)]
    pub is_test_key: bool,
    #[serde(default)]
    pub created_at: String,
    pub last_used_at: Option<String>,
    /// Full API key value — only returned at creation time.
    #[serde(default)]
    pub key: String,
}

/// Request body for creating an API key.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateApiKeyRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_test_key: Option<bool>,
}

// --- Credits ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreditBalance {
    pub balance: i64,
    pub monthly_credits: i64,
    pub used_this_month: i64,
    #[serde(default)]
    pub tier: String,
    #[serde(default)]
    pub overage_rate: f64,
}

/// Credit usage breakdown from GET /credits/usage.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsageBreakdown {
    #[serde(default)]
    pub operations: Vec<UsageOperation>,
    #[serde(default)]
    pub total_debited: i64,
    pub period: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UsageOperation {
    #[serde(default)]
    pub operation: String,
    #[serde(default)]
    pub credits: i64,
}

/// Credit ledger entry from GET /credits/history.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreditLedgerEntry {
    pub id: String,
    #[serde(default, rename = "type")]
    pub entry_type: String,
    #[serde(default)]
    pub amount: i64,
    #[serde(default)]
    pub operation: String,
    #[serde(default)]
    pub created_at: String,
}

// --- Billing ---

#[derive(Debug, Clone, Deserialize)]
pub struct SessionUrlResponse {
    pub url: String,
}

/// Request body for checkout session.
#[derive(Debug, Clone, Serialize)]
pub struct CheckoutRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}

// --- Teams ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub slug: String,
    #[serde(default)]
    pub tier: String,
    #[serde(default)]
    pub credit_balance: i64,
    #[serde(default)]
    pub monthly_credits: i64,
    #[serde(default)]
    pub overage_rate: f64,
    #[serde(default)]
    pub created_at: String,
    pub updated_at: Option<String>,
}

/// Request body for creating a team.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTeamRequest {
    pub name: String,
    pub slug: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamMember {
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub role: String,
    #[serde(default)]
    pub is_active: bool,
    #[serde(default)]
    pub invited_at: String,
    pub joined_at: Option<String>,
}

/// Request body for inviting a team member.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InviteMemberRequest {
    pub email: String,
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invited_by: Option<String>,
}

/// Request body for updating a member's role.
#[derive(Debug, Clone, Serialize)]
pub struct UpdateMemberRoleRequest {
    pub role: String,
}

/// Team billing summary from GET /teams/me/billing-summary.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BillingSummary {
    #[serde(default)]
    pub members: Vec<MemberUsage>,
    #[serde(default)]
    pub total_credits_used: i64,
    #[serde(default)]
    pub period: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemberUsage {
    pub member_id: Option<String>,
    #[serde(default)]
    pub member_name: String,
    pub member_email: Option<String>,
    #[serde(default)]
    pub credits_used: i64,
    #[serde(default)]
    pub percentage: f64,
}

// --- Health ---

#[derive(Debug, Clone, Deserialize)]
pub struct HealthResponse {
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub uptime: String,
    #[serde(default)]
    pub checks: Vec<HealthCheck>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthCheck {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub duration_ms: u64,
    #[serde(default)]
    pub message: String,
}
```

- [ ] **Step 2: Verify compilation**

Run: `cargo build 2>&1 | head -20`
Expected: May have compile errors from resources referencing old types — that's expected, will be fixed in next tasks.

- [ ] **Step 3: Commit**

```bash
git add src/types.rs
git commit -m "feat: overhaul model types to match OpenAPI 1.0.0 spec"
```

---

## Task 3: Remove Obsolete Resources & Update Module Registry

**Files:**
- Delete: `src/resources/webhooks.rs`
- Delete: `src/resources/users.rs`
- Delete: `src/resources/settings.rs`
- Modify: `src/resources/mod.rs`
- Modify: `src/client.rs` — remove `webhooks()`, `users()`, `settings()` accessors

- [ ] **Step 1: Delete obsolete resource files**

```bash
rm src/resources/webhooks.rs src/resources/users.rs src/resources/settings.rs
```

- [ ] **Step 2: Update resources/mod.rs**

Remove the three deleted modules and add placeholders for the seven new ones (will be created in subsequent tasks):

```rust
mod api_keys;
mod analytics;
mod billing;
mod changes;
mod companies;
mod credits;
mod dossiers;
mod health;
mod news;
mod persons;
mod relationships;
mod reports;
mod teams;
mod watches;

pub use api_keys::ApiKeys;
pub use analytics::Analytics;
pub use billing::Billing;
pub use changes::Changes;
pub use companies::Companies;
pub use credits::Credits;
pub use dossiers::Dossiers;
pub use health::Health;
pub use news::News;
pub use persons::Persons;
pub use relationships::Relationships;
pub use reports::Reports;
pub use teams::Teams;
pub use watches::Watches;
```

- [ ] **Step 3: Update client.rs — remove old accessors, add new ones**

Remove `webhooks()`, `users()`, `settings()` accessors. Add:

```rust
pub fn changes(&self) -> Changes<'_> { Changes::new(self) }
pub fn analytics(&self) -> Analytics<'_> { Analytics::new(self) }
pub fn watches(&self) -> Watches<'_> { Watches::new(self) }
pub fn news(&self) -> News<'_> { News::new(self) }
pub fn reports(&self) -> Reports<'_> { Reports::new(self) }
pub fn relationships(&self) -> Relationships<'_> { Relationships::new(self) }
pub fn health(&self) -> Health<'_> { Health::new(self) }
```

- [ ] **Step 4: Update lib.rs re-exports**

Replace the resource re-exports:

```rust
pub use resources::{
    Analytics, ApiKeys, Billing, Changes, Companies, Credits, Dossiers,
    Health, News, Persons, Relationships, Reports, Teams, Watches,
};
```

- [ ] **Step 5: Commit**

```bash
git add -A src/resources/ src/client.rs src/lib.rs
git commit -m "refactor: remove obsolete resources (webhooks/users/settings), update module registry"
```

---

## Task 4: Update Existing Resources — Companies, Persons, Dossiers

**Files:**
- Modify: `src/resources/companies.rs`
- Modify: `src/resources/persons.rs`
- Modify: `src/resources/dossiers.rs`

- [ ] **Step 1: Rewrite companies.rs**

Companies now has 7 endpoints:
- `list(params)` — GET /companies (was `search`)
- `get(uid)` — GET /companies/{uid}
- `count(params)` — GET /companies/count
- `statistics()` — GET /companies/statistics (now typed)
- `search(req)` — POST /companies/search (FTS)
- `batch(req)` — POST /companies/batch
- `compare(req)` — POST /companies/compare

Remove `changes()`, `persons()`, `dossier()` (moved to own resources).

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
    pub async fn list(&self, params: &CompanyListParams) -> Result<Response<PaginatedResponse<Company>>> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(p) = params.page { query.push(("page", p.to_string())); }
        if let Some(ps) = params.page_size { query.push(("pageSize", ps.to_string())); }
        if let Some(ref c) = params.canton { query.push(("canton", c.clone())); }
        if let Some(ref s) = params.search { query.push(("search", s.clone())); }
        if let Some(ref st) = params.status { query.push(("status", st.clone())); }
        if let Some(ref ac) = params.auditor_category { query.push(("auditorCategory", ac.clone())); }
        if let Some(ref sb) = params.sort_by { query.push(("sortBy", sb.clone())); }
        if let Some(sd) = params.sort_desc { query.push(("sortDesc", sd.to_string())); }
        if let Some(ref ts) = params.target_status { query.push(("targetStatus", ts.clone())); }

        if query.is_empty() {
            self.client.request(Method::GET, "/companies").await
        } else {
            self.client.request_with_params(Method::GET, "/companies", &query).await
        }
    }

    /// Get a company by its Swiss UID.
    pub async fn get(&self, uid: &str) -> Result<Response<Company>> {
        self.client.request(Method::GET, &format!("/companies/{uid}")).await
    }

    /// Get the count of companies matching optional filters.
    pub async fn count(&self, params: &CompanyCountParams) -> Result<Response<CompanyCount>> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(ref c) = params.canton { query.push(("canton", c.clone())); }
        if let Some(ref s) = params.status { query.push(("status", s.clone())); }
        if let Some(ref ac) = params.auditor_category { query.push(("auditorCategory", ac.clone())); }

        if query.is_empty() {
            self.client.request(Method::GET, "/companies/count").await
        } else {
            self.client.request_with_params(Method::GET, "/companies/count", &query).await
        }
    }

    /// Get aggregate statistics about companies.
    pub async fn statistics(&self) -> Result<Response<CompanyStatistics>> {
        self.client.request(Method::GET, "/companies/statistics").await
    }

    /// Full-text search companies (FTS5).
    pub async fn search(&self, req: &CompanySearchRequest) -> Result<Response<Vec<Company>>> {
        let resp: Response<serde_json::Value> = self.client
            .request_with_body(Method::POST, "/companies/search", req)
            .await?;
        let data = Client::extract_list(resp.data)?;
        Ok(Response { data, meta: resp.meta })
    }

    /// Batch lookup up to 50 companies by UID.
    pub async fn batch(&self, req: &BatchCompanyRequest) -> Result<Response<Vec<Company>>> {
        let resp: Response<serde_json::Value> = self.client
            .request_with_body(Method::POST, "/companies/batch", req)
            .await?;
        let data = Client::extract_list(resp.data)?;
        Ok(Response { data, meta: resp.meta })
    }

    /// Compare two or more companies side-by-side.
    pub async fn compare(&self, req: &CompareCompaniesRequest) -> Result<Response<serde_json::Value>> {
        self.client.request_with_body(Method::POST, "/companies/compare", req).await
    }
}
```

- [ ] **Step 2: Rewrite persons.rs**

Persons now has 6 endpoints:
- `list(params)` — GET /persons
- `get(id)` — GET /persons/{id}
- `roles(id)` — GET /persons/{id}/roles
- `connections(id)` — GET /persons/{id}/connections
- `board_members(uid)` — GET /persons/board-members/{uid}
- `network_stats()` — GET /persons/network-stats

```rust
use reqwest::Method;
use crate::client::Client;
use crate::error::Result;
use crate::response::Response;
use crate::types::*;

pub struct Persons<'a> {
    client: &'a Client,
}

impl<'a> Persons<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List persons with optional search and pagination.
    pub async fn list(&self, params: &PersonListParams) -> Result<Response<Vec<Person>>> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(p) = params.page { query.push(("page", p.to_string())); }
        if let Some(ps) = params.page_size { query.push(("pageSize", ps.to_string())); }
        if let Some(ref s) = params.search { query.push(("search", s.clone())); }

        let resp: Response<serde_json::Value> = if query.is_empty() {
            self.client.request(Method::GET, "/persons").await?
        } else {
            self.client.request_with_params(Method::GET, "/persons", &query).await?
        };
        let data = Client::extract_list(resp.data)?;
        Ok(Response { data, meta: resp.meta })
    }

    /// Get a person by their ID.
    pub async fn get(&self, id: &str) -> Result<Response<Person>> {
        self.client.request(Method::GET, &format!("/persons/{id}")).await
    }

    /// Get all roles held by a person.
    pub async fn roles(&self, id: &str) -> Result<Response<Vec<serde_json::Value>>> {
        let resp: Response<serde_json::Value> = self.client
            .request(Method::GET, &format!("/persons/{id}/roles"))
            .await?;
        let data = Client::extract_list(resp.data)?;
        Ok(Response { data, meta: resp.meta })
    }

    /// Get the network of companies connected to a person.
    pub async fn connections(&self, id: &str) -> Result<Response<Vec<serde_json::Value>>> {
        let resp: Response<serde_json::Value> = self.client
            .request(Method::GET, &format!("/persons/{id}/connections"))
            .await?;
        let data = Client::extract_list(resp.data)?;
        Ok(Response { data, meta: resp.meta })
    }

    /// Get board members of a company.
    pub async fn board_members(&self, uid: &str) -> Result<Response<Vec<Person>>> {
        let resp: Response<serde_json::Value> = self.client
            .request(Method::GET, &format!("/persons/board-members/{uid}"))
            .await?;
        let data = Client::extract_list(resp.data)?;
        Ok(Response { data, meta: resp.meta })
    }

    /// Get person network statistics.
    pub async fn network_stats(&self) -> Result<Response<serde_json::Value>> {
        self.client.request(Method::GET, "/persons/network-stats").await
    }
}
```

- [ ] **Step 3: Rewrite dossiers.rs**

Dossiers now has 4 endpoints:
- `list()` — GET /dossiers
- `get(uid)` — GET /dossiers/{uid}
- `generate(uid, req)` — POST /dossiers/{uid}/generate
- `statistics()` — GET /dossiers/statistics

```rust
use reqwest::Method;
use crate::client::Client;
use crate::error::Result;
use crate::response::Response;
use crate::types::*;

pub struct Dossiers<'a> {
    client: &'a Client,
}

impl<'a> Dossiers<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List all generated dossiers.
    pub async fn list(&self) -> Result<Response<Vec<Dossier>>> {
        let resp: Response<serde_json::Value> = self.client
            .request(Method::GET, "/dossiers")
            .await?;
        let data = Client::extract_list(resp.data)?;
        Ok(Response { data, meta: resp.meta })
    }

    /// Get the most recent dossier for a company.
    pub async fn get(&self, uid: &str) -> Result<Response<Dossier>> {
        self.client.request(Method::GET, &format!("/dossiers/{uid}")).await
    }

    /// Generate an AI dossier for a company.
    pub async fn generate(&self, uid: &str, req: &GenerateDossierRequest) -> Result<Response<Dossier>> {
        self.client
            .request_with_body(Method::POST, &format!("/dossiers/{uid}/generate"), req)
            .await
    }

    /// Get dossier generation statistics.
    pub async fn statistics(&self) -> Result<Response<serde_json::Value>> {
        self.client.request(Method::GET, "/dossiers/statistics").await
    }
}
```

- [ ] **Step 4: Verify compilation**

Run: `cargo build 2>&1 | head -30`

- [ ] **Step 5: Commit**

```bash
git add src/resources/companies.rs src/resources/persons.rs src/resources/dossiers.rs
git commit -m "feat: update companies/persons/dossiers resources to match final API"
```

---

## Task 5: Update Existing Resources — API Keys, Credits, Billing, Teams

**Files:**
- Modify: `src/resources/api_keys.rs`
- Modify: `src/resources/credits.rs`
- Modify: `src/resources/billing.rs`
- Modify: `src/resources/teams.rs`

- [ ] **Step 1: Update api_keys.rs**

Types changed: `ApiKeyInfo` → `ApiKey`, `ApiKeyCreated` now has `key` field instead of `raw_key`, `CreateApiKeyRequest` simplified.

```rust
use reqwest::Method;
use crate::client::Client;
use crate::error::Result;
use crate::response::{Response, ResponseMeta};
use crate::types::*;

pub struct ApiKeys<'a> {
    client: &'a Client,
}

impl<'a> ApiKeys<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List all API keys for the current team.
    pub async fn list(&self) -> Result<Response<Vec<ApiKey>>> {
        let resp: Response<serde_json::Value> = self.client
            .request(Method::GET, "/api-keys")
            .await?;
        let data = Client::extract_list(resp.data)?;
        Ok(Response { data, meta: resp.meta })
    }

    /// Create a new API key.
    pub async fn create(&self, req: &CreateApiKeyRequest) -> Result<Response<ApiKeyCreated>> {
        self.client.request_with_body(Method::POST, "/api-keys", req).await
    }

    /// Revoke (delete) an API key by its ID.
    pub async fn revoke(&self, id: &str) -> Result<ResponseMeta> {
        self.client.request_empty(Method::DELETE, &format!("/api-keys/{id}")).await
    }
}
```

- [ ] **Step 2: Update credits.rs**

History now returns typed `Vec<CreditLedgerEntry>`.

```rust
use reqwest::Method;
use crate::client::Client;
use crate::error::Result;
use crate::response::Response;
use crate::types::*;

pub struct Credits<'a> {
    client: &'a Client,
}

impl<'a> Credits<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Get the current credit balance and tier information.
    pub async fn balance(&self) -> Result<Response<CreditBalance>> {
        self.client.request(Method::GET, "/credits/balance").await
    }

    /// Get credit usage breakdown by operation type.
    pub async fn usage(&self, since: Option<&str>) -> Result<Response<UsageBreakdown>> {
        match since {
            Some(s) => {
                let params = [("since", s.to_string())];
                self.client.request_with_params(Method::GET, "/credits/usage", &params).await
            }
            None => self.client.request(Method::GET, "/credits/usage").await,
        }
    }

    /// Get credit ledger history.
    pub async fn history(&self, limit: Option<u32>, offset: Option<u32>) -> Result<Response<Vec<CreditLedgerEntry>>> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(l) = limit { query.push(("limit", l.to_string())); }
        if let Some(o) = offset { query.push(("offset", o.to_string())); }

        let resp: Response<serde_json::Value> = if query.is_empty() {
            self.client.request(Method::GET, "/credits/history").await?
        } else {
            self.client.request_with_params(Method::GET, "/credits/history", &query).await?
        };
        let data = Client::extract_list(resp.data)?;
        Ok(Response { data, meta: resp.meta })
    }
}
```

- [ ] **Step 3: Update billing.rs**

Paths changed: `/billing/checkout` → `/billing/checkout-session`, `/billing/portal` → `/billing/portal-session`. Types unified to `SessionUrlResponse`.

```rust
use reqwest::Method;
use crate::client::Client;
use crate::error::Result;
use crate::response::Response;
use crate::types::*;

pub struct Billing<'a> {
    client: &'a Client,
}

impl<'a> Billing<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Create a Stripe checkout session for upgrading to a tier.
    pub async fn create_checkout(&self, req: &CheckoutRequest) -> Result<Response<SessionUrlResponse>> {
        self.client
            .request_with_body(Method::POST, "/billing/checkout-session", req)
            .await
    }

    /// Create a Stripe billing portal session.
    pub async fn create_portal(&self) -> Result<Response<SessionUrlResponse>> {
        self.client
            .request_with_body(Method::POST, "/billing/portal-session", &serde_json::json!({}))
            .await
    }
}
```

- [ ] **Step 4: Rewrite teams.rs**

Teams expands from 2 to 7 endpoints:

```rust
use reqwest::Method;
use crate::client::Client;
use crate::error::Result;
use crate::response::{Response, ResponseMeta};
use crate::types::*;

pub struct Teams<'a> {
    client: &'a Client,
}

impl<'a> Teams<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Get the current team.
    pub async fn me(&self) -> Result<Response<Team>> {
        self.client.request(Method::GET, "/teams/me").await
    }

    /// Create a new team.
    pub async fn create(&self, req: &CreateTeamRequest) -> Result<Response<Team>> {
        self.client.request_with_body(Method::POST, "/teams", req).await
    }

    /// List team members.
    pub async fn members(&self) -> Result<Response<Vec<TeamMember>>> {
        let resp: Response<serde_json::Value> = self.client
            .request(Method::GET, "/teams/me/members")
            .await?;
        let data = Client::extract_list(resp.data)?;
        Ok(Response { data, meta: resp.meta })
    }

    /// Invite a new team member.
    pub async fn invite_member(&self, req: &InviteMemberRequest) -> Result<Response<TeamMember>> {
        self.client
            .request_with_body(Method::POST, "/teams/me/members", req)
            .await
    }

    /// Update a team member's role.
    pub async fn update_member_role(&self, id: &str, req: &UpdateMemberRoleRequest) -> Result<Response<TeamMember>> {
        self.client
            .request_with_body(Method::PUT, &format!("/teams/me/members/{id}"), req)
            .await
    }

    /// Remove a team member.
    pub async fn remove_member(&self, id: &str) -> Result<ResponseMeta> {
        self.client
            .request_empty(Method::DELETE, &format!("/teams/me/members/{id}"))
            .await
    }

    /// Get team billing summary.
    pub async fn billing_summary(&self) -> Result<Response<BillingSummary>> {
        self.client.request(Method::GET, "/teams/me/billing-summary").await
    }
}
```

- [ ] **Step 5: Verify compilation**

Run: `cargo build 2>&1 | head -30`

- [ ] **Step 6: Commit**

```bash
git add src/resources/api_keys.rs src/resources/credits.rs src/resources/billing.rs src/resources/teams.rs
git commit -m "feat: update api_keys/credits/billing/teams resources to match final API"
```

---

## Task 6: Create New Resources — Changes, Analytics, Watches

**Files:**
- Create: `src/resources/changes.rs`
- Create: `src/resources/analytics.rs`
- Create: `src/resources/watches.rs`

- [ ] **Step 1: Create changes.rs (6 endpoints)**

```rust
use reqwest::Method;
use crate::client::Client;
use crate::error::Result;
use crate::response::Response;
use crate::types::*;

pub struct Changes<'a> {
    client: &'a Client,
}

impl<'a> Changes<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List company changes with optional filtering.
    pub async fn list(&self, params: &ChangeListParams) -> Result<Response<PaginatedResponse<CompanyChange>>> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(p) = params.page { query.push(("page", p.to_string())); }
        if let Some(ps) = params.page_size { query.push(("pageSize", ps.to_string())); }
        if let Some(ref uid) = params.company_uid { query.push(("companyUid", uid.clone())); }

        if query.is_empty() {
            self.client.request(Method::GET, "/changes").await
        } else {
            self.client.request_with_params(Method::GET, "/changes", &query).await
        }
    }

    /// Get all changes for a specific company.
    pub async fn by_company(&self, uid: &str) -> Result<Response<Vec<CompanyChange>>> {
        let resp: Response<serde_json::Value> = self.client
            .request(Method::GET, &format!("/changes/{uid}"))
            .await?;
        let data = Client::extract_list(resp.data)?;
        Ok(Response { data, meta: resp.meta })
    }

    /// Get change statistics.
    pub async fn statistics(&self) -> Result<Response<ChangeStatistics>> {
        self.client.request(Method::GET, "/changes/statistics").await
    }

    /// Get changes by SOGC publication ID.
    pub async fn by_sogc(&self, sogc_id: &str) -> Result<Response<Vec<CompanyChange>>> {
        let resp: Response<serde_json::Value> = self.client
            .request(Method::GET, &format!("/changes/sogc/{sogc_id}"))
            .await?;
        let data = Client::extract_list(resp.data)?;
        Ok(Response { data, meta: resp.meta })
    }

    /// Mark a change as reviewed.
    pub async fn review(&self, id: &str, req: &ReviewChangeRequest) -> Result<Response<ReviewChangeResponse>> {
        self.client
            .request_with_body(Method::PUT, &format!("/changes/{id}/review"), req)
            .await
    }

    /// Batch fetch changes for multiple companies.
    pub async fn batch(&self, req: &BatchChangeRequest) -> Result<Response<Vec<CompanyChange>>> {
        let resp: Response<serde_json::Value> = self.client
            .request_with_body(Method::POST, "/changes/batch", req)
            .await?;
        let data = Client::extract_list(resp.data)?;
        Ok(Response { data, meta: resp.meta })
    }
}
```

- [ ] **Step 2: Create analytics.rs (7 endpoints)**

```rust
use reqwest::Method;
use crate::client::Client;
use crate::error::Result;
use crate::response::Response;
use crate::types::*;

pub struct Analytics<'a> {
    client: &'a Client,
}

impl<'a> Analytics<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Run K-Means clustering on companies.
    pub async fn cluster(&self, req: &ClusterRequest) -> Result<Response<serde_json::Value>> {
        self.client.request_with_body(Method::POST, "/analytics/cluster", req).await
    }

    /// Run on-demand anomaly detection.
    pub async fn anomalies(&self, req: &AnomalyRequest) -> Result<Response<serde_json::Value>> {
        self.client.request_with_body(Method::POST, "/analytics/anomalies", req).await
    }

    /// Get cohort analytics.
    pub async fn cohorts(&self, params: &CohortParams) -> Result<Response<serde_json::Value>> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(ref g) = params.group_by { query.push(("groupBy", g.clone())); }
        if let Some(ref c) = params.canton { query.push(("canton", c.clone())); }

        if query.is_empty() {
            self.client.request(Method::GET, "/analytics/cohorts").await
        } else {
            self.client.request_with_params(Method::GET, "/analytics/cohorts", &query).await
        }
    }

    /// Get canton analytics.
    pub async fn cantons(&self) -> Result<Response<serde_json::Value>> {
        self.client.request(Method::GET, "/analytics/cantons").await
    }

    /// Get auditor analytics.
    pub async fn auditors(&self) -> Result<Response<serde_json::Value>> {
        self.client.request(Method::GET, "/analytics/auditors").await
    }

    /// Get RFM segmentation.
    pub async fn rfm_segments(&self) -> Result<Response<serde_json::Value>> {
        self.client.request(Method::GET, "/analytics/rfm-segments").await
    }

    /// Get change velocity analytics.
    pub async fn velocity(&self, days: Option<u32>) -> Result<Response<serde_json::Value>> {
        match days {
            Some(d) => {
                let params = [("days", d.to_string())];
                self.client.request_with_params(Method::GET, "/analytics/velocity", &params).await
            }
            None => self.client.request(Method::GET, "/analytics/velocity").await,
        }
    }
}
```

- [ ] **Step 3: Create watches.rs (4 endpoints)**

```rust
use reqwest::Method;
use crate::client::Client;
use crate::error::Result;
use crate::response::{Response, ResponseMeta};
use crate::types::*;

pub struct Watches<'a> {
    client: &'a Client,
}

impl<'a> Watches<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List all watched companies.
    pub async fn list(&self) -> Result<Response<Vec<CompanyWatch>>> {
        let resp: Response<serde_json::Value> = self.client
            .request(Method::GET, "/watches")
            .await?;
        let data = Client::extract_list(resp.data)?;
        Ok(Response { data, meta: resp.meta })
    }

    /// Add a company watch.
    pub async fn create(&self, req: &CreateWatchRequest) -> Result<Response<CompanyWatch>> {
        self.client.request_with_body(Method::POST, "/watches", req).await
    }

    /// Remove a company watch.
    pub async fn remove(&self, company_uid: &str) -> Result<ResponseMeta> {
        self.client
            .request_empty(Method::DELETE, &format!("/watches/{company_uid}"))
            .await
    }

    /// List change notifications.
    pub async fn notifications(&self, limit: Option<u32>) -> Result<Response<Vec<ChangeNotification>>> {
        let resp: Response<serde_json::Value> = match limit {
            Some(l) => {
                let params = [("limit", l.to_string())];
                self.client.request_with_params(Method::GET, "/notifications", &params).await?
            }
            None => self.client.request(Method::GET, "/notifications").await?,
        };
        let data = Client::extract_list(resp.data)?;
        Ok(Response { data, meta: resp.meta })
    }
}
```

- [ ] **Step 4: Verify compilation**

Run: `cargo build 2>&1 | head -30`

- [ ] **Step 5: Commit**

```bash
git add src/resources/changes.rs src/resources/analytics.rs src/resources/watches.rs
git commit -m "feat: add changes, analytics, and watches resources"
```

---

## Task 7: Create New Resources — News, Reports, Relationships, Health

**Files:**
- Create: `src/resources/news.rs`
- Create: `src/resources/reports.rs`
- Create: `src/resources/relationships.rs`
- Create: `src/resources/health.rs`

- [ ] **Step 1: Create news.rs (2 endpoints)**

```rust
use reqwest::Method;
use crate::client::Client;
use crate::error::Result;
use crate::response::Response;
use crate::types::*;

pub struct News<'a> {
    client: &'a Client,
}

impl<'a> News<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Get news for a specific company.
    pub async fn for_company(&self, uid: &str, limit: Option<u32>) -> Result<Response<CompanyNewsResponse>> {
        match limit {
            Some(l) => {
                let params = [("limit", l.to_string())];
                self.client.request_with_params(Method::GET, &format!("/companies/{uid}/news"), &params).await
            }
            None => self.client.request(Method::GET, &format!("/companies/{uid}/news")).await,
        }
    }

    /// Get recent news across all companies.
    pub async fn recent(&self, limit: Option<u32>) -> Result<Response<RecentNewsResponse>> {
        match limit {
            Some(l) => {
                let params = [("limit", l.to_string())];
                self.client.request_with_params(Method::GET, "/news/recent", &params).await
            }
            None => self.client.request(Method::GET, "/news/recent").await,
        }
    }
}
```

- [ ] **Step 2: Create reports.rs (1 endpoint)**

```rust
use reqwest::Method;
use crate::client::Client;
use crate::error::Result;
use crate::response::Response;
use crate::types::*;

pub struct Reports<'a> {
    client: &'a Client,
}

impl<'a> Reports<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Get financial reports for a company.
    pub async fn for_company(&self, uid: &str, limit: Option<u32>) -> Result<Response<CompanyReportsResponse>> {
        match limit {
            Some(l) => {
                let params = [("limit", l.to_string())];
                self.client.request_with_params(Method::GET, &format!("/companies/{uid}/reports"), &params).await
            }
            None => self.client.request(Method::GET, &format!("/companies/{uid}/reports")).await,
        }
    }
}
```

- [ ] **Step 3: Create relationships.rs (2 endpoints)**

```rust
use reqwest::Method;
use crate::client::Client;
use crate::error::Result;
use crate::response::Response;
use crate::types::*;

pub struct Relationships<'a> {
    client: &'a Client,
}

impl<'a> Relationships<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Get company relationships (parent, subsidiaries, board overlaps).
    pub async fn for_company(&self, uid: &str) -> Result<Response<RelationshipResponse>> {
        self.client
            .request(Method::GET, &format!("/companies/{uid}/relationships"))
            .await
    }

    /// Get corporate hierarchy (full recursive parent/subsidiary tree).
    pub async fn hierarchy(&self, uid: &str) -> Result<Response<RelationshipResponse>> {
        self.client
            .request(Method::GET, &format!("/companies/{uid}/hierarchy"))
            .await
    }
}
```

- [ ] **Step 4: Create health.rs (1 endpoint)**

```rust
use reqwest::Method;
use crate::client::Client;
use crate::error::Result;
use crate::response::Response;
use crate::types::*;

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
```

- [ ] **Step 5: Verify compilation of async SDK**

Run: `cargo build 2>&1 | head -30`
Expected: Build succeeds. Blocking client and tests may still fail.

- [ ] **Step 6: Commit**

```bash
git add src/resources/news.rs src/resources/reports.rs src/resources/relationships.rs src/resources/health.rs
git commit -m "feat: add news, reports, relationships, and health resources"
```

---

## Task 8: Update Blocking Client

**Files:**
- Modify: `src/blocking.rs`

Complete rewrite to mirror all async resource changes. Remove Webhooks/Users/Settings. Add Changes/Analytics/Watches/News/Reports/Relationships/Health.

- [ ] **Step 1: Rewrite blocking.rs**

The full blocking client should mirror every async resource method. See the companion async resources for exact method signatures — every async method gets a synchronous wrapper that calls `self.client.block_on(...)`.

Key changes:
- Remove `Webhooks`, `Users`, `Settings` structs and impls
- Update `Companies` to use new method signatures (`list` instead of `search`, `count` takes `CompanyCountParams`, add `search`/`batch`/`compare`)
- Update `Persons` (6 methods), `Dossiers` (4 methods), `Teams` (7 methods)
- Update `Billing` (new request type), `ApiKeys` (new types), `Credits` (typed history)
- Add `Changes`, `Analytics`, `Watches`, `News`, `Reports`, `Relationships`, `Health` structs

- [ ] **Step 2: Verify compilation with blocking feature**

Run: `cargo build --features blocking 2>&1 | head -30`
Expected: Build succeeds.

- [ ] **Step 3: Commit**

```bash
git add src/blocking.rs
git commit -m "feat: update blocking client to mirror all async API changes"
```

---

## Task 9: Update Tests

**Files:**
- Modify: `tests/test_client.rs`

Update all existing tests for new types/paths and add tests for new resources.

- [ ] **Step 1: Update existing tests**

Key changes needed:
- `company_search_parses_paginated_response` → rename to `company_list_parses_paginated_response`, use `CompanyListParams`, update JSON to match new Company schema (`totalCount` not `total`, remove `legalSeat`/`capitalNominal` etc., add `address`/`auditorCategory`/`createdAt`/`updatedAt`)
- `company_get_by_uid` → update JSON for new Company fields
- `api_key_creation_returns_secret` → update types (`CreateApiKeyRequest` simplified, `key` instead of `rawKey`)
- `insufficient_credits_returns_error` → update dossier path from `POST /dossiers` to `POST /dossiers/CHE-100.000.000/generate`, update request type
- All other tests remain structurally the same

- [ ] **Step 2: Add new tests**

Add tests for:
- Company full-text search (POST /companies/search)
- Changes list endpoint
- Health check endpoint
- Rate limit remaining/reset headers in ResponseMeta
- Conflict (409) error mapping

- [ ] **Step 3: Run all tests**

Run: `cargo test 2>&1`
Expected: All tests pass.

- [ ] **Step 4: Commit**

```bash
git add tests/test_client.rs
git commit -m "test: update and expand tests for API v1.0.0 alignment"
```

---

## Task 10: Update Documentation & Version

**Files:**
- Modify: `Cargo.toml`
- Modify: `README.md`
- Modify: `CHANGELOG.md`
- Modify: `CLAUDE.md`

- [ ] **Step 1: Bump version in Cargo.toml**

Change `version = "0.1.0"` to `version = "1.0.0"`.

- [ ] **Step 2: Update README.md**

- Update base URL reference to `https://api.vynco.ch/api/v1`
- Update resource table to reflect new resources and methods
- Update Quick Start examples for new type names (`CompanyListParams`, `CompanySearchRequest`)
- Update Response Metadata section with new `rate_limit_remaining` and `rate_limit_reset` fields
- Update Error Handling to include `Conflict` variant
- Remove references to removed resources (webhooks, users, settings)

- [ ] **Step 3: Update CHANGELOG.md**

Add v1.0.0 entry documenting all changes.

- [ ] **Step 4: Update CLAUDE.md**

Update architecture section to reflect new resources, paths, models.

- [ ] **Step 5: Final verification**

Run: `cargo build && cargo build --features blocking && cargo test`
Expected: Everything compiles and all tests pass.

- [ ] **Step 6: Commit**

```bash
git add Cargo.toml README.md CHANGELOG.md CLAUDE.md
git commit -m "docs: update README, CHANGELOG, CLAUDE.md for v1.0.0 release"
```
