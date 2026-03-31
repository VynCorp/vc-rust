use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Pagination
// ---------------------------------------------------------------------------

/// Generic paginated response wrapper.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PagedResponse<T> {
    #[serde(default)]
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
#[serde(rename_all = "camelCase")]
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
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub struct CompanyCount {
    #[serde(default)]
    pub count: i64,
}

// ---------------------------------------------------------------------------
// Events
// ---------------------------------------------------------------------------

/// Response wrapper for event listing.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventListResponse {
    #[serde(default)]
    pub events: Vec<CompanyEvent>,
    #[serde(default)]
    pub count: i64,
}

/// A CloudEvent-style company event.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub struct AuditorHistoryResponse {
    pub company_uid: String,
    pub company_name: String,
    #[serde(default)]
    pub current_auditor: Option<AuditorTenure>,
    #[serde(default)]
    pub history: Vec<AuditorTenure>,
}

/// A single auditor tenure record.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditorTenure {
    pub id: String,
    #[serde(default)]
    pub company_uid: String,
    #[serde(default)]
    pub company_name: String,
    #[serde(default)]
    pub auditor_name: String,
    #[serde(default)]
    pub appointed_at: Option<String>,
    #[serde(default)]
    pub resigned_at: Option<String>,
    #[serde(default)]
    pub tenure_years: Option<f64>,
    #[serde(default)]
    pub is_current: bool,
    #[serde(default)]
    pub source: String,
}

/// Query parameters for auditor tenure listing.
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

/// Admin dashboard response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DashboardResponse {
    #[serde(default)]
    pub generated_at: String,
    #[serde(default)]
    pub data: DataCompleteness,
    #[serde(default)]
    pub pipelines: Vec<PipelineStatus>,
    #[serde(default)]
    pub auditor_tenures: AuditorTenureStats,
}

/// Data completeness metrics.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
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

/// Pipeline run status.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PipelineStatus {
    pub name: String,
    #[serde(default)]
    pub last_run: Option<String>,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub records_processed: Option<i64>,
    #[serde(default)]
    pub duration_seconds: Option<f64>,
}

/// Auditor tenure aggregate statistics.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
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

/// Request body for compliance screening.
#[derive(Debug, Clone, Serialize)]
pub struct ScreeningRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<String>>,
}

/// Screening result response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreeningResponse {
    #[serde(default)]
    pub query_name: String,
    #[serde(default)]
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
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub struct Watchlist {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
}

/// Watchlist summary (used in list responses).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchlistSummary {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub company_count: i64,
    #[serde(default)]
    pub created_at: String,
}

/// Request body for creating a watchlist.
#[derive(Debug, Clone, Serialize)]
pub struct CreateWatchlistRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Response containing UIDs of companies in a watchlist.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchlistCompaniesResponse {
    #[serde(default)]
    pub uids: Vec<String>,
}

/// Request body for adding companies to a watchlist.
#[derive(Debug, Clone, Serialize)]
pub struct AddCompaniesRequest {
    pub uids: Vec<String>,
}

/// Response from adding companies to a watchlist.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddCompaniesResponse {
    #[serde(default)]
    pub added: i64,
}

// ---------------------------------------------------------------------------
// Webhooks
// ---------------------------------------------------------------------------

/// A webhook subscription.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
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

/// Request body for creating a webhook subscription.
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
#[serde(rename_all = "camelCase")]
pub struct CreateWebhookResponse {
    #[serde(default)]
    pub webhook: WebhookSubscription,
    #[serde(default)]
    pub signing_secret: String,
}

/// Request body for updating a webhook subscription.
#[derive(Debug, Clone, Default, Serialize)]
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
#[serde(rename_all = "camelCase")]
pub struct TestDeliveryResponse {
    #[serde(default)]
    pub success: bool,
    #[serde(default)]
    pub http_status: Option<i32>,
    #[serde(default)]
    pub error: Option<String>,
}

/// A webhook delivery record.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookDelivery {
    pub id: String,
    #[serde(default)]
    pub event_id: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub attempt: i32,
    #[serde(default)]
    pub http_status: Option<i32>,
    #[serde(default)]
    pub error_message: Option<String>,
    #[serde(default)]
    pub delivered_at: Option<String>,
    #[serde(default)]
    pub created_at: String,
}

// ---------------------------------------------------------------------------
// Exports
// ---------------------------------------------------------------------------

/// Request body for creating a data export.
#[derive(Debug, Clone, Default, Serialize)]
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

/// An export job record.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportJob {
    pub id: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub format: String,
    #[serde(default)]
    pub total_rows: Option<i32>,
    #[serde(default)]
    pub file_size_bytes: Option<i64>,
    #[serde(default)]
    pub error_message: Option<String>,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub completed_at: Option<String>,
    #[serde(default)]
    pub expires_at: Option<String>,
}

/// Export download response (job metadata + optional data).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportDownload {
    #[serde(default)]
    pub job: ExportJob,
    #[serde(default)]
    pub data: Option<String>,
}

// ---------------------------------------------------------------------------
// AI
// ---------------------------------------------------------------------------

/// Request body for generating an AI dossier.
#[derive(Debug, Clone, Serialize)]
pub struct DossierRequest {
    pub uid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<String>,
}

/// AI dossier response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DossierResponse {
    #[serde(default)]
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

/// Request body for AI-powered search.
#[derive(Debug, Clone, Serialize)]
pub struct AiSearchRequest {
    pub query: String,
}

/// AI search response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
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

/// Request body for risk score assessment.
#[derive(Debug, Clone, Serialize)]
pub struct RiskScoreRequest {
    pub uid: String,
}

/// Risk score response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskScoreResponse {
    #[serde(default)]
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
#[serde(rename_all = "camelCase")]
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

#[derive(Debug, Clone, Default, Deserialize)]
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
    pub by_status: HashMap<String, i64>,
    #[serde(default)]
    pub by_canton: HashMap<String, i64>,
    #[serde(default)]
    pub by_legal_form: HashMap<String, i64>,
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

#[derive(Debug, Clone, Default, Deserialize)]
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
