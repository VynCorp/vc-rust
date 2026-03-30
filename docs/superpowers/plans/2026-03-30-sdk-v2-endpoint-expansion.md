# SDK v2.0.0 Endpoint Expansion Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add 41 new endpoints across 8 new resource modules + extend companies module, bringing the SDK from 28 to 69 endpoints.

**Architecture:** Additive — follows the exact patterns established in the v2.0.0 rewrite. All new API response types use `#[serde(rename_all = "camelCase")]`. New resource modules follow the existing `Resource<'a>` borrowing pattern with mockito tests.

**Tech Stack:** Rust, reqwest, serde, tokio, mockito

---

## File Structure

| File | Action | Content |
|------|--------|---------|
| `src/types.rs` | Extend | ~35 new types for all new endpoints |
| `src/client.rs` | Modify | Add 8 new resource accessors |
| `src/resources/mod.rs` | Modify | Add 8 new module declarations + re-exports |
| `src/lib.rs` | Modify | Add new re-exports |
| `src/resources/api_keys.rs` | Create | 3 endpoints: create, list, revoke |
| `src/resources/credits.rs` | Create | 3 endpoints: balance, usage, history |
| `src/resources/teams.rs` | Create | 7 endpoints: me, create, members, invite, update_role, remove, billing_summary |
| `src/resources/billing.rs` | Create | 2 endpoints: checkout, portal |
| `src/resources/changes.rs` | Create | 3 endpoints: list, by_company, statistics |
| `src/resources/persons.rs` | Create | 1 endpoint: board_members |
| `src/resources/analytics.rs` | Create | 8 endpoints: statistics, cantons, auditors, cluster, anomalies, rfm, cohorts, candidates |
| `src/resources/dossiers.rs` | Create | 4 endpoints: create, list, get, delete |
| `src/resources/graph.rs` | Create | 3 endpoints: get, export, analyze |
| `src/resources/companies.rs` | Extend | +8 endpoints: statistics, compare, news, reports, relationships, hierarchy, fingerprint, nearby |
| `src/blocking.rs` | Extend | Add blocking wrappers for all new resources + extended companies methods |
| `examples/vynco_cli.rs` | Extend | Add representative new commands |
| `README.md` | Update | New endpoint count and resource table |
| `CLAUDE.md` | Update | Resource table, endpoint count |
| `CHANGELOG.md` | Update | Add expansion entry |

---

### Task 1: Add all new types to types.rs

**Files:** Modify `src/types.rs`

- [ ] **Step 1: Append all new types to src/types.rs**

Add the following sections after the existing AI types. ALL response types use `#[serde(rename_all = "camelCase")]` since the API returns camelCase.

```rust
// ---------------------------------------------------------------------------
// API Keys
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
pub struct CreateApiKeyRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyCreated {
    pub key: String,
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub prefix: String,
    #[serde(default)]
    pub environment: String,
    #[serde(default)]
    pub scopes: Vec<String>,
    pub expires_at: Option<String>,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub warning: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKey {
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub prefix: String,
    #[serde(default)]
    pub environment: String,
    #[serde(default)]
    pub scopes: Vec<String>,
    #[serde(default)]
    pub status: String,
    pub expires_at: Option<String>,
    #[serde(default)]
    pub created_at: String,
    pub last_used_at: Option<String>,
}

// ---------------------------------------------------------------------------
// Credits
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreditBalance {
    #[serde(default)]
    pub balance: i32,
    #[serde(default)]
    pub monthly_credits: i64,
    #[serde(default)]
    pub used_this_month: i32,
    #[serde(default)]
    pub tier: String,
    #[serde(default)]
    pub overage_rate: f64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreditUsage {
    #[serde(default)]
    pub operations: Vec<UsageRow>,
    #[serde(default)]
    pub total: i64,
    #[serde(default)]
    pub period: UsagePeriod,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsagePeriod {
    #[serde(default)]
    pub since: String,
    #[serde(default)]
    pub until: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsageRow {
    #[serde(default)]
    pub operation: String,
    #[serde(default)]
    pub count: i64,
    #[serde(default)]
    pub total_credits: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreditHistory {
    #[serde(default)]
    pub items: Vec<CreditLedgerEntry>,
    #[serde(default)]
    pub total: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreditLedgerEntry {
    pub id: i64,
    #[serde(default)]
    pub entry_type: String,
    #[serde(default)]
    pub amount: i32,
    #[serde(default)]
    pub balance: i32,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub created_at: String,
}

// ---------------------------------------------------------------------------
// Teams
// ---------------------------------------------------------------------------

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
    pub credit_balance: i32,
    #[serde(default)]
    pub monthly_credits: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateTeamRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
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
    pub last_login_at: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InviteMemberRequest {
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Invitation {
    pub id: String,
    #[serde(default)]
    pub team_id: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub role: String,
    #[serde(default)]
    pub token: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub expires_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateMemberRoleRequest {
    pub role: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BillingSummary {
    #[serde(default)]
    pub tier: String,
    #[serde(default)]
    pub credit_balance: i32,
    #[serde(default)]
    pub monthly_credits: i64,
    #[serde(default)]
    pub used_this_month: i32,
    #[serde(default)]
    pub members: Vec<MemberUsage>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemberUsage {
    pub user_id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub credits_used: i32,
}

// ---------------------------------------------------------------------------
// Billing
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
pub struct CheckoutRequest {
    pub tier: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionUrl {
    pub url: String,
}

// ---------------------------------------------------------------------------
// Changes
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Default, Serialize)]
pub struct ChangeListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanyChange {
    pub id: String,
    #[serde(default)]
    pub company_uid: String,
    pub company_name: Option<String>,
    #[serde(default)]
    pub change_type: String,
    pub field_name: Option<String>,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub description: Option<String>,
    pub source: Option<String>,
    #[serde(default)]
    pub detected_at: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeStatistics {
    #[serde(default)]
    pub total_changes: i64,
    #[serde(default)]
    pub changes_this_week: i64,
    #[serde(default)]
    pub changes_this_month: i64,
    #[serde(default)]
    pub by_type: serde_json::Value,
}

// ---------------------------------------------------------------------------
// Persons
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoardMember {
    pub id: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    #[serde(default)]
    pub role: String,
    #[serde(default)]
    pub role_category: String,
    pub origin: Option<String>,
    pub residence: Option<String>,
    pub signing_authority: Option<String>,
    pub since: Option<String>,
}

// ---------------------------------------------------------------------------
// Analytics
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanyStatistics {
    #[serde(default)]
    pub total: i64,
    #[serde(default)]
    pub by_status: std::collections::HashMap<String, i64>,
    #[serde(default)]
    pub by_canton: std::collections::HashMap<String, i64>,
    #[serde(default)]
    pub by_legal_form: std::collections::HashMap<String, i64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CantonDistribution {
    #[serde(default)]
    pub canton: String,
    #[serde(default)]
    pub count: i64,
    #[serde(default)]
    pub percentage: f64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditorMarketShare {
    #[serde(default)]
    pub auditor_name: String,
    #[serde(default)]
    pub company_count: i64,
    #[serde(default)]
    pub percentage: f64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterRequest {
    pub algorithm: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterResponse {
    #[serde(default)]
    pub clusters: Vec<ClusterResult>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterResult {
    #[serde(default)]
    pub id: u32,
    #[serde(default)]
    pub centroid: serde_json::Value,
    #[serde(default)]
    pub company_count: i64,
    #[serde(default)]
    pub sample_companies: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AnomalyRequest {
    pub algorithm: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnomalyResponse {
    #[serde(default)]
    pub anomalies: Vec<serde_json::Value>,
    #[serde(default)]
    pub total_scanned: i64,
    #[serde(default)]
    pub threshold: f64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RfmSegmentsResponse {
    #[serde(default)]
    pub segments: Vec<RfmSegment>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RfmSegment {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub count: i64,
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CohortParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CohortResponse {
    #[serde(default)]
    pub cohorts: Vec<CohortEntry>,
    #[serde(default)]
    pub group_by: String,
    #[serde(default)]
    pub metric: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CohortEntry {
    #[serde(default)]
    pub group: String,
    #[serde(default)]
    pub count: i64,
    #[serde(default)]
    pub metric: String,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CandidateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canton: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditCandidate {
    pub uid: String,
    pub name: String,
    pub canton: Option<String>,
    pub legal_form: Option<String>,
    pub share_capital: Option<f64>,
    pub auditor_name: Option<String>,
    pub auditor_category: Option<String>,
}

// ---------------------------------------------------------------------------
// Dossiers (managed)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDossierRequest {
    pub uid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dossier {
    pub id: String,
    #[serde(default)]
    pub user_id: String,
    #[serde(default)]
    pub company_uid: String,
    #[serde(default)]
    pub company_name: String,
    #[serde(default)]
    pub level: String,
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub sources: Vec<String>,
    #[serde(default)]
    pub created_at: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DossierSummary {
    pub id: String,
    #[serde(default)]
    pub company_uid: String,
    #[serde(default)]
    pub company_name: String,
    #[serde(default)]
    pub level: String,
    #[serde(default)]
    pub created_at: String,
}

// ---------------------------------------------------------------------------
// Graph / Network
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphResponse {
    #[serde(default)]
    pub nodes: Vec<GraphNode>,
    #[serde(default)]
    pub links: Vec<GraphLink>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphNode {
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub uid: String,
    #[serde(default, rename = "type")]
    pub node_type: String,
    pub capital: Option<f64>,
    pub canton: Option<String>,
    pub status: Option<String>,
    pub role: Option<String>,
    pub person_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphLink {
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub target: String,
    #[serde(default, rename = "type")]
    pub link_type: String,
    #[serde(default)]
    pub label: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkAnalysisRequest {
    pub uids: Vec<String>,
    pub overlay: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkAnalysisResponse {
    #[serde(default)]
    pub nodes: Vec<GraphNode>,
    #[serde(default)]
    pub links: Vec<GraphLink>,
    #[serde(default)]
    pub clusters: Vec<NetworkCluster>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkCluster {
    #[serde(default)]
    pub id: i32,
    #[serde(default)]
    pub company_uids: Vec<String>,
    #[serde(default)]
    pub shared_persons: Vec<String>,
}

// ---------------------------------------------------------------------------
// Company Extended (news, reports, relationships, hierarchy, fingerprint, nearby)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewsItem {
    pub id: String,
    #[serde(default)]
    pub title: String,
    pub summary: Option<String>,
    pub source: Option<String>,
    #[serde(default)]
    pub source_type: String,
    #[serde(default)]
    pub published_at: String,
    #[serde(default, rename = "sourceUrl")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanyReport {
    #[serde(default)]
    pub report_type: String,
    pub fiscal_year: Option<i32>,
    #[serde(default)]
    pub description: String,
    pub source_url: Option<String>,
    #[serde(default)]
    pub publication_date: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Relationship {
    #[serde(default)]
    pub related_uid: String,
    #[serde(default)]
    pub related_name: String,
    #[serde(default)]
    pub relationship_type: String,
    #[serde(default)]
    pub shared_persons: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HierarchyResponse {
    pub parent: Option<serde_json::Value>,
    #[serde(default)]
    pub subsidiaries: Vec<serde_json::Value>,
    #[serde(default)]
    pub siblings: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fingerprint {
    #[serde(default)]
    pub company_uid: String,
    #[serde(default)]
    pub name: String,
    pub industry_sector: Option<String>,
    pub industry_group: Option<String>,
    pub industry: Option<String>,
    pub size_category: Option<String>,
    pub employee_count_estimate: Option<i32>,
    pub capital_amount: Option<f64>,
    pub capital_currency: Option<String>,
    pub revenue: Option<f64>,
    pub net_income: Option<f64>,
    pub auditor_tier: Option<String>,
    #[serde(default)]
    pub change_frequency: i64,
    #[serde(default)]
    pub board_size: i64,
    #[serde(default)]
    pub company_age: i64,
    #[serde(default)]
    pub canton: String,
    #[serde(default)]
    pub legal_form: String,
    #[serde(default)]
    pub has_parent_company: bool,
    #[serde(default)]
    pub subsidiary_count: i64,
    #[serde(default)]
    pub generated_at: String,
    #[serde(default)]
    pub fingerprint_version: String,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct NearbyParams {
    pub lat: f64,
    pub lng: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius_km: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NearbyCompany {
    pub uid: String,
    pub name: String,
    #[serde(default)]
    pub distance: f64,
    #[serde(default)]
    pub latitude: f64,
    #[serde(default)]
    pub longitude: f64,
}

// ---------------------------------------------------------------------------
// Compare
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompareRequest {
    pub uids: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompareResponse {
    #[serde(default)]
    pub uids: Vec<String>,
    #[serde(default)]
    pub names: Vec<String>,
    #[serde(default)]
    pub dimensions: Vec<ComparisonDimension>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComparisonDimension {
    #[serde(default)]
    pub field: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub values: Vec<Option<String>>,
}
```

- [ ] **Step 2: Verify compilation**
Run: `cargo check --lib`

- [ ] **Step 3: Commit**
```bash
git add src/types.rs
git commit -m "feat: add types for 41 new API endpoints"
```

---

### Task 2: Scaffold new modules and update client

**Files:** Modify `src/client.rs`, `src/resources/mod.rs`, `src/lib.rs`. Create 9 new stub resource files.

- [ ] **Step 1: Add 8 new resource accessors to client.rs** (after existing `ai()` accessor)

```rust
    pub fn api_keys(&self) -> ApiKeys<'_> { ApiKeys::new(self) }
    pub fn credits(&self) -> Credits<'_> { Credits::new(self) }
    pub fn teams(&self) -> Teams<'_> { Teams::new(self) }
    pub fn billing(&self) -> Billing<'_> { Billing::new(self) }
    pub fn changes(&self) -> Changes<'_> { Changes::new(self) }
    pub fn persons(&self) -> Persons<'_> { Persons::new(self) }
    pub fn analytics(&self) -> Analytics<'_> { Analytics::new(self) }
    pub fn dossiers(&self) -> Dossiers<'_> { Dossiers::new(self) }
    pub fn graph(&self) -> Graph<'_> { Graph::new(self) }
```

- [ ] **Step 2: Update resources/mod.rs** — add all new module declarations and re-exports
- [ ] **Step 3: Update lib.rs** — add new resource types to re-exports
- [ ] **Step 4: Create stub files** for all 9 new resource modules (struct + new() only)
- [ ] **Step 5: Verify**: `cargo check --lib`
- [ ] **Step 6: Commit**

---

### Task 3: API Keys resource (3 endpoints)

**Files:** Create `src/resources/api_keys.rs`

Endpoints:
- `POST /v1/api-keys` → `create(&CreateApiKeyRequest)` → `Response<ApiKeyCreated>`
- `GET /v1/api-keys` → `list()` → `Response<Vec<ApiKey>>`
- `DELETE /v1/api-keys/{id}` → `revoke(&str)` → `Result<ResponseMeta>`

Tests: 1 test for create, 1 test for list.

---

### Task 4: Credits resource (3 endpoints)

**Files:** Create `src/resources/credits.rs`

Endpoints:
- `GET /v1/credits/balance` → `balance()` → `Response<CreditBalance>`
- `GET /v1/credits/usage` → `usage(since: Option<&str>)` → `Response<CreditUsage>`
- `GET /v1/credits/history` → `history(limit: Option<i64>, offset: Option<i64>)` → `Response<CreditHistory>`

Tests: 1 test for balance, 1 test for history with params.

---

### Task 5: Billing resource (2 endpoints)

**Files:** Create `src/resources/billing.rs`

Endpoints:
- `POST /v1/billing/checkout-session` → `create_checkout(&CheckoutRequest)` → `Response<SessionUrl>`
- `POST /v1/billing/portal-session` → `create_portal()` → `Response<SessionUrl>`

Tests: 1 test for checkout.

---

### Task 6: Teams resource (7 endpoints)

**Files:** Create `src/resources/teams.rs`

Endpoints:
- `GET /v1/teams/me` → `me()` → `Response<Team>`
- `POST /v1/teams` → `create(&CreateTeamRequest)` → `Response<Team>`
- `GET /v1/teams/me/members` → `members()` → `Response<Vec<TeamMember>>`
- `POST /v1/teams/me/members` → `invite_member(&InviteMemberRequest)` → `Response<Invitation>`
- `PUT /v1/teams/me/members/{id}` → `update_member_role(&str, &UpdateMemberRoleRequest)` → `Response<TeamMember>`
- `DELETE /v1/teams/me/members/{id}` → `remove_member(&str)` → `Result<ResponseMeta>`
- `GET /v1/teams/me/billing-summary` → `billing_summary()` → `Response<BillingSummary>`

Tests: 1 for me, 1 for invite_member.

---

### Task 7: Changes resource (3 endpoints)

**Files:** Create `src/resources/changes.rs`

Endpoints:
- `GET /v1/changes` → `list(&ChangeListParams)` → `Response<PagedResponse<CompanyChange>>`
  Query params built manually: `type`, `since`, `until`, `companySearch`, `page`, `pageSize`
- `GET /v1/changes/{uid}` → `by_company(&str)` → `Response<Vec<CompanyChange>>`
- `GET /v1/changes/statistics` → `statistics()` → `Response<ChangeStatistics>`

Tests: 1 for list, 1 for statistics.

---

### Task 8: Persons resource (1 endpoint)

**Files:** Create `src/resources/persons.rs`

Endpoints:
- `GET /v1/persons/board-members/{uid}` → `board_members(&str)` → `Response<Vec<BoardMember>>`

Tests: 1 test.

---

### Task 9: Analytics resource (8 endpoints)

**Files:** Create `src/resources/analytics.rs`

Endpoints:
- `GET /v1/companies/statistics` → `statistics()` → `Response<CompanyStatistics>`
- `GET /v1/analytics/cantons` → `cantons()` → `Response<Vec<CantonDistribution>>`
- `GET /v1/analytics/auditors` → `auditors()` → `Response<Vec<AuditorMarketShare>>`
- `POST /v1/analytics/cluster` → `cluster(&ClusterRequest)` → `Response<ClusterResponse>`
- `POST /v1/analytics/anomalies` → `anomalies(&AnomalyRequest)` → `Response<AnomalyResponse>`
- `GET /v1/analytics/rfm-segments` → `rfm_segments()` → `Response<RfmSegmentsResponse>`
- `GET /v1/analytics/cohorts` → `cohorts(&CohortParams)` → `Response<CohortResponse>`
  Query params: `groupBy`, `metric`
- `GET /v1/analytics/candidates` → `candidates(&CandidateParams)` → `Response<PagedResponse<AuditCandidate>>`
  Query params: `sortBy`, `canton`, `page`, `pageSize`

Tests: 1 for statistics, 1 for cluster.

---

### Task 10: Dossiers resource (4 endpoints)

**Files:** Create `src/resources/dossiers.rs`

Endpoints:
- `POST /v1/dossiers` → `create(&CreateDossierRequest)` → `Response<Dossier>`
- `GET /v1/dossiers` → `list()` → `Response<Vec<DossierSummary>>`
- `GET /v1/dossiers/{id_or_uid}` → `get(&str)` → `Response<Dossier>`
- `DELETE /v1/dossiers/{id}` → `delete(&str)` → `Result<ResponseMeta>`

Tests: 1 for create, 1 for list.

---

### Task 11: Graph resource (3 endpoints)

**Files:** Create `src/resources/graph.rs`

Endpoints:
- `GET /v1/graph/{uid}` → `get(&str)` → `Response<GraphResponse>`
- `GET /v1/graph/{uid}/export?format=X` → `export(&str, &str)` → `Result<ExportFile>` (uses `request_bytes` since response is XML)
- `POST /v1/network/analyze` → `analyze(&NetworkAnalysisRequest)` → `Response<NetworkAnalysisResponse>`

Tests: 1 for get, 1 for analyze.

---

### Task 12: Extend Companies resource (+8 endpoints)

**Files:** Modify `src/resources/companies.rs`

Add methods:
- `GET /v1/companies/statistics` → `statistics()` → `Response<CompanyStatistics>`
- `POST /v1/companies/compare` → `compare(&CompareRequest)` → `Response<CompareResponse>`
- `GET /v1/companies/{uid}/news` → `news(&str)` → `Response<Vec<NewsItem>>`
- `GET /v1/companies/{uid}/reports` → `reports(&str)` → `Response<Vec<CompanyReport>>`
- `GET /v1/companies/{uid}/relationships` → `relationships(&str)` → `Response<Vec<Relationship>>`
- `GET /v1/companies/{uid}/hierarchy` → `hierarchy(&str)` → `Response<HierarchyResponse>`
- `GET /v1/companies/{uid}/fingerprint` → `fingerprint(&str)` → `Response<Fingerprint>`
- `GET /v1/companies/nearby` → `nearby(&NearbyParams)` → `Response<Vec<NearbyCompany>>`
  Query params: `lat`, `lng`, `radiusKm`, `limit`

Tests: 1 for compare, 1 for fingerprint.

---

### Task 13: Update blocking.rs

**Files:** Modify `src/blocking.rs`

Add blocking wrappers for all 9 new resource modules + extended companies methods. Same `block_on()` delegation pattern.

---

### Task 14: Update example CLI + docs

**Files:** Modify `examples/vynco_cli.rs`, `README.md`, `CLAUDE.md`, `CHANGELOG.md`

Add 3-4 new CLI commands: `credits`, `team`, `changes`, `board-members`.
Update README resource table (17 modules, 69 endpoints).
Update CLAUDE.md resource table.
Add CHANGELOG entry for endpoint expansion.

---

### Task 15: Final verification

- [ ] `cargo build`
- [ ] `cargo build --features blocking`
- [ ] `cargo test`
- [ ] `cargo fmt`
- [ ] `cargo clippy --all-features --all-targets`
- [ ] `cargo publish --dry-run`

---

## Parallelization

```
Task 1 → Task 2 → Tasks 3-12 (parallel) → Task 13 → Task 14 → Task 15
```

Tasks 3-12 touch independent files and can all run in parallel after scaffolding.
