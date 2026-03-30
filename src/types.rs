use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Pagination
// ---------------------------------------------------------------------------

/// Generic paginated response wrapper.
#[derive(Debug, Clone, Deserialize)]
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

/// Company detail response (single company with extended fields).
#[derive(Debug, Clone, Deserialize)]
pub struct CompanyDetail {
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
    pub purpose: Option<String>,
    #[serde(default)]
    pub address: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

/// Request body for company search.
#[derive(Debug, Clone, Serialize)]
pub struct CompanySearchRequest {
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
}

/// Response wrapper for company search results.
#[derive(Debug, Clone, Deserialize)]
pub struct CompanySearchResponse {
    #[serde(default)]
    pub data: Vec<CompanyDetail>,
    #[serde(default)]
    pub total: i64,
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
    #[serde(default)]
    pub count: i64,
}

// ---------------------------------------------------------------------------
// Events
// ---------------------------------------------------------------------------

/// Response wrapper for event listing.
#[derive(Debug, Clone, Deserialize)]
pub struct EventListResponse {
    #[serde(default)]
    pub events: Vec<CompanyEvent>,
    #[serde(default)]
    pub count: i64,
}

/// A CloudEvent-style company event.
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
    pub company_name: String,
    #[serde(default)]
    pub current_auditor: Option<AuditorTenure>,
    #[serde(default)]
    pub history: Vec<AuditorTenure>,
}

/// A single auditor tenure record.
#[derive(Debug, Clone, Default, Deserialize)]
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
pub struct AddCompaniesResponse {
    #[serde(default)]
    pub added: i64,
}

// ---------------------------------------------------------------------------
// Webhooks
// ---------------------------------------------------------------------------

/// A webhook subscription.
#[derive(Debug, Clone, Default, Deserialize)]
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
