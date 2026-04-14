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
    pub currency: Option<String>,
    #[serde(default)]
    pub purpose: Option<String>,
    #[serde(default)]
    pub founding_date: Option<String>,
    #[serde(default)]
    pub registration_date: Option<String>,
    #[serde(default)]
    pub deletion_date: Option<String>,
    #[serde(default)]
    pub legal_seat: Option<String>,
    #[serde(default)]
    pub municipality: Option<String>,
    #[serde(default)]
    pub data_source: Option<String>,
    #[serde(default)]
    pub enrichment_level: Option<String>,
    #[serde(default)]
    pub address_street: Option<String>,
    #[serde(default)]
    pub address_house_number: Option<String>,
    #[serde(default)]
    pub address_zip_code: Option<String>,
    #[serde(default)]
    pub address_city: Option<String>,
    #[serde(default)]
    pub address_canton: Option<String>,
    #[serde(default)]
    pub website: Option<String>,
    #[serde(default)]
    pub industry: Option<String>,
    #[serde(default)]
    pub sub_industry: Option<String>,
    #[serde(default)]
    pub employee_count: Option<i32>,
    #[serde(default)]
    pub auditor_name: Option<String>,
    #[serde(default)]
    pub auditor_category: Option<String>,
    #[serde(default)]
    pub latitude: Option<f64>,
    #[serde(default)]
    pub longitude: Option<f64>,
    #[serde(default)]
    pub geo_precision: Option<String>,
    #[serde(default)]
    pub noga_code: Option<String>,
    #[serde(default)]
    pub sanctions_hit: Option<bool>,
    #[serde(default)]
    pub last_screened_at: Option<String>,
    #[serde(default)]
    pub is_finma_regulated: Option<bool>,
    #[serde(default)]
    pub ehraid: Option<i64>,
    #[serde(default)]
    pub chid: Option<String>,
    #[serde(default)]
    pub cantonal_excerpt_url: Option<String>,
    #[serde(default)]
    pub old_names: Option<Vec<String>>,
    #[serde(default)]
    pub translations: Option<Vec<String>>,
    #[serde(default)]
    pub updated_at: Option<String>,
    // Enrichment provenance (v3.1+)
    #[serde(default)]
    pub direct_parent_lei: Option<String>,
    #[serde(default)]
    pub ultimate_parent_lei: Option<String>,
    #[serde(default)]
    pub ultimate_parent_name: Option<String>,
    #[serde(default)]
    pub gleif_parent_enriched_at: Option<String>,
    #[serde(default)]
    pub industry_source: Option<String>,
    #[serde(default)]
    pub industry_confidence: Option<f32>,
    #[serde(default)]
    pub industry_classified_at: Option<String>,
    // External identifiers
    #[serde(default)]
    pub lei: Option<String>,
    #[serde(default)]
    pub duns: Option<String>,
    #[serde(default)]
    pub isin: Option<String>,
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
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_form: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capital_min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capital_max: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auditor_category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_desc: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lei: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duns: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isin: Option<String>,
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
    pub enriched_companies: i64,
    #[serde(default)]
    pub companies_with_industry: i64,
    #[serde(default)]
    pub companies_with_geo: i64,
    #[serde(default)]
    pub total_persons: i64,
    #[serde(default)]
    pub total_changes: i64,
    #[serde(default)]
    pub total_sogc_publications: i64,
}

/// Pipeline run status.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PipelineStatus {
    pub id: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub items_processed: i32,
    #[serde(default)]
    pub last_completed_at: Option<String>,
}

/// Auditor tenure aggregate statistics.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditorTenureStats {
    #[serde(default)]
    pub total_tracked: i64,
    #[serde(default)]
    pub current_auditors: i64,
    #[serde(default)]
    pub tenures_over_10_years: i64,
    #[serde(default)]
    pub tenures_over_7_years: i64,
    #[serde(default)]
    pub avg_tenure_years: f64,
    #[serde(default)]
    pub longest_tenure: Option<LongestTenure>,
}

/// The longest current auditor tenure.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LongestTenure {
    #[serde(default)]
    pub company_uid: String,
    #[serde(default)]
    pub company_name: String,
    #[serde(default)]
    pub auditor_name: String,
    #[serde(default)]
    pub tenure_years: f64,
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
    /// Enriched company entries with name/status/canton (v3.1+).
    #[serde(default)]
    pub companies: Vec<WatchlistCompanyEntry>,
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
    pub results: Vec<serde_json::Value>,
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
    pub used_this_month: i64,
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

/// Request body for joining a team via invitation token.
#[derive(Debug, Clone, Serialize)]
pub struct JoinTeamRequest {
    pub token: String,
}

/// Response from joining a team.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JoinTeamResponse {
    #[serde(default)]
    pub team_id: String,
    #[serde(default)]
    pub team_name: String,
    #[serde(default)]
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
    pub used_this_month: i64,
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
    pub credits_used: i64,
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
    // Enrichment provenance (v3.1+)
    pub role_source: Option<String>,
    pub role_confidence: Option<f32>,
    pub role_inferred_at: Option<String>,
}

/// Query parameters for searching persons.
#[derive(Debug, Clone, Default, Serialize)]
pub struct PersonSearchParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

/// A person search result.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonSearchResult {
    pub id: String,
    #[serde(default)]
    pub full_name: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub place_of_origin: Option<String>,
    pub nationality: Option<String>,
    pub role_count: Option<i64>,
}

/// Detailed person record with roles across companies.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonDetail {
    pub id: String,
    #[serde(default)]
    pub full_name: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub place_of_origin: Option<String>,
    pub residence: Option<String>,
    pub nationality: Option<String>,
    #[serde(default)]
    pub roles: Vec<PersonRoleDetail>,
}

/// A person's role at a specific company.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonRoleDetail {
    #[serde(default)]
    pub company_uid: String,
    pub company_name: Option<String>,
    #[serde(default)]
    pub role_function: String,
    #[serde(default)]
    pub role_category: String,
    pub signing_authority: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub change_action: Option<String>,
    pub is_current: Option<bool>,
    // Enrichment provenance (v3.1+)
    pub role_source: Option<String>,
    pub role_confidence: Option<f32>,
    pub role_inferred_at: Option<String>,
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
    pub parent: Option<HierarchyEntity>,
    #[serde(default)]
    pub subsidiaries: Vec<HierarchyEntity>,
    #[serde(default)]
    pub siblings: Vec<HierarchyEntity>,
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
    // Swiss register entry date (v3.1+)
    pub registration_date: Option<String>,
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
// Company full (composite endpoint)
// ---------------------------------------------------------------------------

/// Full company details with persons, changes, and relationships.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanyFullResponse {
    #[serde(default)]
    pub company: Company,
    #[serde(default)]
    pub persons: Vec<PersonEntry>,
    #[serde(default)]
    pub recent_changes: Vec<ChangeEntry>,
    #[serde(default)]
    pub relationships: Vec<RelationshipEntry>,
}

/// A person entry in a company's full response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonEntry {
    pub person_id: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    #[serde(default)]
    pub role: String,
    pub since: Option<String>,
    pub until: Option<String>,
    // Enrichment provenance (v3.1+)
    pub role_source: Option<String>,
    pub role_confidence: Option<f32>,
    pub role_inferred_at: Option<String>,
}

/// A recent change entry in a company's full response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeEntry {
    pub id: String,
    #[serde(default)]
    pub company_uid: String,
    pub change_type: Option<String>,
    pub field_name: Option<String>,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    #[serde(default)]
    pub detected_at: String,
    pub source_date: Option<String>,
}

/// A relationship entry in a company's full response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipEntry {
    #[serde(default)]
    pub related_uid: String,
    pub related_name: Option<String>,
    #[serde(default)]
    pub relationship_type: String,
}

// ---------------------------------------------------------------------------
// Classification
// ---------------------------------------------------------------------------

/// Industry classification for a company.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Classification {
    #[serde(default)]
    pub company_uid: String,
    pub sector_code: Option<String>,
    pub sector_name: Option<String>,
    pub group_code: Option<String>,
    pub group_name: Option<String>,
    pub industry_code: Option<String>,
    pub industry_name: Option<String>,
    pub sub_industry_code: Option<String>,
    pub sub_industry_name: Option<String>,
    #[serde(default)]
    pub method: String,
    #[serde(default)]
    pub classified_at: String,
    pub auditor_category: Option<String>,
    #[serde(default)]
    pub is_finma_regulated: bool,
    // Enrichment provenance (v3.1+)
    #[serde(default)]
    pub industry_source: Option<String>,
    #[serde(default)]
    pub industry_confidence: Option<f32>,
}

// ---------------------------------------------------------------------------
// Corporate structure
// ---------------------------------------------------------------------------

/// Corporate structure showing head offices, branches, and M&A relationships.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CorporateStructure {
    #[serde(default)]
    pub head_offices: Vec<RelatedCompanyEntry>,
    #[serde(default)]
    pub branch_offices: Vec<RelatedCompanyEntry>,
    #[serde(default)]
    pub acquisitions: Vec<RelatedCompanyEntry>,
    #[serde(default)]
    pub acquired_by: Vec<RelatedCompanyEntry>,
}

/// A related company entry in a corporate structure.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelatedCompanyEntry {
    #[serde(default)]
    pub uid: String,
    #[serde(default)]
    pub name: String,
}

// ---------------------------------------------------------------------------
// Notes
// ---------------------------------------------------------------------------

/// A user note on a company.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub id: String,
    #[serde(default)]
    pub company_uid: String,
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub note_type: String,
    #[serde(default)]
    pub rating: Option<i32>,
    #[serde(default)]
    pub is_private: bool,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
}

/// Request body for creating a note on a company.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateNoteRequest {
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,
}

/// Request body for updating a note.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateNoteRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,
}

// ---------------------------------------------------------------------------
// Tags
// ---------------------------------------------------------------------------

/// A user tag on a company.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub id: String,
    #[serde(default)]
    pub company_uid: String,
    #[serde(default)]
    pub tag_name: String,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub created_at: String,
}

/// Request body for creating a tag on a company.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTagRequest {
    pub tag_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

/// Summary of a user's tag usage across companies.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagSummary {
    #[serde(default)]
    pub tag_name: String,
    #[serde(default)]
    pub count: i64,
}

// ---------------------------------------------------------------------------
// Excel export
// ---------------------------------------------------------------------------

/// Request body for Excel/CSV export of companies.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExcelExportRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<ExcelExportFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<String>>,
}

/// Filter criteria for Excel export.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExcelExportFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canton: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auditor_category: Option<String>,
}

// ---------------------------------------------------------------------------
// Acquisitions (M&A)
// ---------------------------------------------------------------------------

/// An M&A relationship record.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Acquisition {
    #[serde(default)]
    pub acquirer_uid: String,
    #[serde(default)]
    pub acquired_uid: String,
    pub acquirer_name: Option<String>,
    pub acquired_name: Option<String>,
    #[serde(default)]
    pub created_at: String,
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

// ---------------------------------------------------------------------------
// Hierarchy (typed entity — replaces the untyped Any on HierarchyResponse)
// ---------------------------------------------------------------------------

/// A company entity in a hierarchy response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HierarchyEntity {
    #[serde(default)]
    pub uid: String,
    #[serde(default)]
    pub name: String,
    pub confidence: Option<String>,
    pub shared_person_count: Option<i64>,
    pub shared_persons: Option<Vec<String>>,
}

// ---------------------------------------------------------------------------
// Timeline (v3.1+)
// ---------------------------------------------------------------------------

/// A single event on a company timeline.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimelineEvent {
    pub id: String,
    #[serde(default)]
    pub category: String,
    pub field_name: Option<String>,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub summary: Option<String>,
    pub source: Option<String>,
    pub severity: Option<String>,
    #[serde(default)]
    pub date: String,
}

/// Chronological timeline of a company's changes and events.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimelineResponse {
    #[serde(default)]
    pub uid: String,
    #[serde(default)]
    pub company_name: String,
    #[serde(default)]
    pub events: Vec<TimelineEvent>,
    #[serde(default)]
    pub total_events: i64,
}

/// Query parameters for timeline endpoints.
#[derive(Debug, Clone, Default, Serialize)]
pub struct TimelineParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "changeType")]
    pub change_type: Option<String>,
}

/// AI-generated narrative summary of a company timeline.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimelineSummaryResponse {
    #[serde(default)]
    pub uid: String,
    #[serde(default)]
    pub company_name: String,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub event_count: i64,
    #[serde(default)]
    pub generated_at: String,
}

// ---------------------------------------------------------------------------
// Similar companies (v3.1+)
// ---------------------------------------------------------------------------

/// A company similar to a given query company.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimilarCompanyResult {
    pub uid: String,
    pub name: String,
    pub canton: Option<String>,
    pub industry: Option<String>,
    pub legal_form: Option<String>,
    pub share_capital: Option<f64>,
    pub status: Option<String>,
    #[serde(default)]
    pub similarity_score: i32,
    #[serde(default)]
    pub matching_dimensions: Vec<String>,
}

/// Response containing companies similar to a query company.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimilarCompaniesResponse {
    #[serde(default)]
    pub company_uid: String,
    #[serde(default)]
    pub company_name: String,
    #[serde(default)]
    pub results: Vec<SimilarCompanyResult>,
}

// ---------------------------------------------------------------------------
// UBO / Ownership (v3.1+)
//
// Non-Swiss parent entities resolved via GLEIF appear with synthetic
// identifiers of the form `LEI:<20-char-lei>` in the `*_uid` fields of
// `UboPerson`, `ChainLink`, and `OwnershipLink`. These are NOT resolvable
// via `companies().get()`.
// ---------------------------------------------------------------------------

/// A natural person identified as an ultimate beneficial owner.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UboPerson {
    #[serde(default)]
    pub person_id: i64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub controlling_entity_uid: String,
    #[serde(default)]
    pub controlling_entity_name: String,
    #[serde(default)]
    pub role: String,
    pub signing_authority: Option<String>,
    #[serde(default)]
    pub path_length: i32,
}

/// A single link in an ownership chain.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChainLink {
    #[serde(default)]
    pub from_uid: String,
    #[serde(default)]
    pub from_name: String,
    #[serde(default)]
    pub to_uid: String,
    #[serde(default)]
    pub to_name: String,
    #[serde(default)]
    pub depth: i32,
}

/// Ultimate beneficial owner resolution response.
///
/// When the backend cannot fully resolve the chain, `data_coverage_note`
/// contains a human-readable explanation (e.g. missing GLEIF enrichment).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UboResponse {
    #[serde(default)]
    pub uid: String,
    #[serde(default)]
    pub company_name: String,
    #[serde(default)]
    pub ubo_persons: Vec<UboPerson>,
    #[serde(default)]
    pub ownership_chain: Vec<ChainLink>,
    #[serde(default)]
    pub chain_depth: i32,
    #[serde(default)]
    pub risk_flags: Vec<String>,
    pub data_coverage_note: Option<String>,
}

/// Request body for ownership.trace.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OwnershipRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_depth: Option<i32>,
}

/// A company entity in an ownership chain.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OwnershipEntity {
    #[serde(default)]
    pub uid: String,
    #[serde(default)]
    pub name: String,
    pub canton: Option<String>,
    pub status: Option<String>,
    pub legal_form: Option<String>,
    pub share_capital: Option<f64>,
}

/// A single directional relationship in an ownership chain.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OwnershipLink {
    #[serde(default)]
    pub source_uid: String,
    #[serde(default)]
    pub source_name: String,
    #[serde(default)]
    pub target_uid: String,
    #[serde(default)]
    pub target_name: String,
    #[serde(default)]
    pub relationship_type: String,
    #[serde(default)]
    pub depth: i32,
}

/// A person's role at a specific company in an ownership chain.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonCompanyRole {
    #[serde(default)]
    pub company_uid: String,
    #[serde(default)]
    pub company_name: String,
    #[serde(default)]
    pub role: String,
}

/// A person with significant roles across the ownership chain.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyPerson {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub companies: Vec<PersonCompanyRole>,
}

/// A detected circular ownership pattern.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CircularFlag {
    #[serde(default)]
    pub loop_uids: Vec<String>,
    #[serde(default)]
    pub description: String,
}

/// Full ownership trace response from `POST /ownership/{uid}`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OwnershipResponse {
    #[serde(default)]
    pub uid: String,
    #[serde(default)]
    pub company_name: String,
    #[serde(default)]
    pub ownership_chain: Vec<OwnershipLink>,
    pub ultimate_parent: Option<OwnershipEntity>,
    #[serde(default)]
    pub key_persons: Vec<KeyPerson>,
    #[serde(default)]
    pub circular_flags: Vec<CircularFlag>,
    #[serde(default)]
    pub risk_level: String,
    #[serde(default)]
    pub assessed_at: String,
}

// ---------------------------------------------------------------------------
// Media (v3.1+)
// ---------------------------------------------------------------------------

/// A media/news item with optional sentiment analysis.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaItem {
    pub id: String,
    #[serde(default)]
    pub title: String,
    pub summary: Option<String>,
    pub source: Option<String>,
    pub published_at: Option<String>,
    pub url: Option<String>,
    pub sentiment_score: Option<f32>,
    pub sentiment_label: Option<String>,
    pub topics: Option<Vec<String>>,
    pub risk_relevance: Option<f32>,
}

/// Response containing a list of media items.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaResponse {
    #[serde(default)]
    pub items: Vec<MediaItem>,
    #[serde(default)]
    pub total: i64,
}

/// Query parameters for `companies.media()`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct MediaParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentiment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

/// Response from triggering LLM sentiment analysis on media items.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaAnalysisResponse {
    #[serde(default)]
    pub analyzed_count: i32,
    #[serde(default)]
    pub message: String,
}

// ---------------------------------------------------------------------------
// Alerts (v3.1+)
// ---------------------------------------------------------------------------

/// A saved alert that triggers on matching query results.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Alert {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub query_params: serde_json::Value,
    pub webhook_url: Option<String>,
    #[serde(default)]
    pub frequency: String,
    #[serde(default)]
    pub is_active: bool,
    pub saved_search_id: Option<String>,
    pub last_triggered_at: Option<String>,
    pub last_result_count: Option<i32>,
    #[serde(default)]
    pub trigger_count: i32,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
}

/// Request body for creating an alert.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAlertRequest {
    pub name: String,
    pub query_params: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saved_search_id: Option<String>,
}

// ---------------------------------------------------------------------------
// Analytics — flows, migrations, benchmark (v3.1+)
// ---------------------------------------------------------------------------

/// A single period of company registration/dissolution flow.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowDataPoint {
    #[serde(default)]
    pub period: String,
    #[serde(default)]
    pub group: String,
    #[serde(default)]
    pub registrations: i64,
    #[serde(default)]
    pub dissolutions: i64,
    #[serde(default)]
    pub net: i64,
}

/// Market flow analytics response.
///
/// `data_coverage_note` surfaces known asymmetries (e.g. dissolution detection
/// started later than registration detection).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowsResponse {
    #[serde(default)]
    pub flows: Vec<FlowDataPoint>,
    pub data_coverage_note: Option<String>,
}

/// Query parameters for `analytics.flows()`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct FlowsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "groupBy")]
    pub group_by: Option<String>,
}

/// A single canton-to-canton migration flow.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MigrationFlow {
    #[serde(default)]
    pub from_canton: String,
    #[serde(default)]
    pub to_canton: String,
    #[serde(default)]
    pub count: i64,
}

/// Canton migration analytics response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MigrationResponse {
    #[serde(default)]
    pub flows: Vec<MigrationFlow>,
    #[serde(default)]
    pub top_flows: Vec<MigrationFlow>,
}

/// A single benchmarking dimension.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BenchmarkDimension {
    #[serde(default)]
    pub name: String,
    pub company_value: Option<f64>,
    pub industry_median: Option<f64>,
    pub percentile: Option<f64>,
    pub peers_with_data: Option<i64>,
}

/// Industry benchmarking response — how a company compares to peers.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BenchmarkResponse {
    #[serde(default)]
    pub uid: String,
    #[serde(default)]
    pub company_name: String,
    pub industry: Option<String>,
    #[serde(default)]
    pub peer_count: i64,
    #[serde(default)]
    pub dimensions: Vec<BenchmarkDimension>,
}

/// Query parameters for `analytics.benchmark()`.
#[derive(Debug, Clone, Serialize)]
pub struct BenchmarkParams {
    pub uid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<String>,
}

// ---------------------------------------------------------------------------
// Batch screening & batch risk scoring (v3.1+)
// ---------------------------------------------------------------------------

/// Batch screening request body (up to 100 UIDs).
#[derive(Debug, Clone, Serialize)]
pub struct BatchScreeningRequest {
    pub uids: Vec<String>,
}

/// A summary of a single screening hit within a batch result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchScreeningHitSummary {
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub matched_name: String,
    #[serde(default)]
    pub score: f64,
}

/// Screening result for a single company in a batch request.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchScreeningResultByUid {
    #[serde(default)]
    pub uid: String,
    #[serde(default)]
    pub company_name: String,
    #[serde(default)]
    pub risk_level: String,
    #[serde(default)]
    pub total_hits: i32,
    #[serde(default)]
    pub sources_checked: Vec<String>,
    #[serde(default)]
    pub hits: Vec<BatchScreeningHitSummary>,
}

/// Response from a batch screening request.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchScreeningResponse {
    #[serde(default)]
    pub results: Vec<BatchScreeningResultByUid>,
}

/// Batch risk-score request body (up to 50 UIDs).
#[derive(Debug, Clone, Serialize)]
pub struct BatchRiskScoreRequest {
    pub uids: Vec<String>,
}

/// A summary risk score for a single company in a batch request.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskScoreResult {
    #[serde(default)]
    pub uid: String,
    #[serde(default)]
    pub company_name: String,
    #[serde(default)]
    pub overall_score: i32,
    #[serde(default)]
    pub risk_level: String,
}

/// Response from a batch risk scoring request.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchRiskScoreResponse {
    #[serde(default)]
    pub results: Vec<RiskScoreResult>,
}

// ---------------------------------------------------------------------------
// Person network (v3.1+)
// ---------------------------------------------------------------------------

/// Summary of a person in a network response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPerson {
    pub id: String,
    #[serde(default)]
    pub full_name: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

/// A company in a person's network.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkCompany {
    #[serde(default)]
    pub uid: String,
    pub name: Option<String>,
    #[serde(default)]
    pub role: String,
    #[serde(default)]
    pub role_category: String,
    pub is_current: Option<bool>,
    pub since: Option<String>,
    pub until: Option<String>,
    // Enrichment provenance (v3.1+)
    pub role_source: Option<String>,
    pub role_confidence: Option<f32>,
    pub role_inferred_at: Option<String>,
}

/// A company shared between a person and a co-director.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoDirectorCompany {
    #[serde(default)]
    pub uid: String,
    pub name: Option<String>,
}

/// A person who shares company directorships with the primary person.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoDirector {
    #[serde(default)]
    pub person_id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub shared_companies: i64,
    #[serde(default)]
    pub companies: Vec<CoDirectorCompany>,
}

/// Aggregate statistics for a person's network.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkStats {
    #[serde(default)]
    pub total_companies: i64,
    #[serde(default)]
    pub active_roles: i64,
    #[serde(default)]
    pub co_director_count: i64,
}

/// Response for a person-centric network view.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonNetworkResponse {
    pub person: NetworkPerson,
    #[serde(default)]
    pub companies: Vec<NetworkCompany>,
    #[serde(default)]
    pub co_directors: Vec<CoDirector>,
    pub stats: NetworkStats,
}

/// Query parameters for `persons.board_members()`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct BoardMemberParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "pageSize")]
    pub page_size: Option<i64>,
}

// ---------------------------------------------------------------------------
// Enriched watchlist entry (v3.1+)
// ---------------------------------------------------------------------------

/// An enriched company entry in a watchlist response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchlistCompanyEntry {
    pub uid: String,
    pub name: Option<String>,
    pub status: Option<String>,
    pub canton: Option<String>,
}

// ---------------------------------------------------------------------------
// Comparative AI
// ---------------------------------------------------------------------------

/// Request body for the AI comparative dossier endpoint.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComparativeRequest {
    pub uids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus: Option<String>,
}

/// A company-role entry within a board overlap.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OverlapCompanyRole {
    #[serde(default)]
    pub uid: String,
    #[serde(default)]
    pub company_name: String,
    #[serde(default)]
    pub role: String,
}

/// A board member appearing in multiple compared companies.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoardOverlap {
    #[serde(default)]
    pub person_name: String,
    #[serde(default)]
    pub companies: Vec<OverlapCompanyRole>,
}

/// An auditor serving one or more compared companies.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompAuditorEntry {
    #[serde(default)]
    pub auditor_name: String,
    #[serde(default)]
    pub company_count: i64,
    #[serde(default)]
    pub company_uids: Vec<String>,
    #[serde(default)]
    pub group_share: f64,
}

/// Auditor analysis across compared companies.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditorAnalysis {
    #[serde(default)]
    pub auditor_distribution: Vec<CompAuditorEntry>,
    #[serde(default)]
    pub unique_auditor_count: i64,
    #[serde(default)]
    pub concentration_flag: bool,
}

/// A single governance score factor.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GovernanceFactor {
    #[serde(default)]
    pub factor: String,
    #[serde(default)]
    pub score: i64,
    #[serde(default)]
    pub description: String,
}

/// Per-company governance score.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GovernanceScore {
    #[serde(default)]
    pub uid: String,
    #[serde(default)]
    pub company_name: String,
    #[serde(default)]
    pub score: i64,
    #[serde(default)]
    pub factors: Vec<GovernanceFactor>,
}

/// AI-generated comparative dossier for multiple companies.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComparativeResponse {
    #[serde(default)]
    pub uids: Vec<String>,
    #[serde(default)]
    pub focus: String,
    #[serde(default)]
    pub report: String,
    #[serde(default)]
    pub board_overlaps: Vec<BoardOverlap>,
    #[serde(default)]
    pub auditor_analysis: Option<AuditorAnalysis>,
    #[serde(default)]
    pub governance_scores: Vec<GovernanceScore>,
    #[serde(default)]
    pub generated_at: String,
}

// ---------------------------------------------------------------------------
// Predictive Risk
// ---------------------------------------------------------------------------

/// Request body for the predictive risk scoring endpoint.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PredictiveRiskRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookback_days: Option<i64>,
}

/// A pre-dissolution risk indicator.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PredictiveRiskIndicator {
    #[serde(default)]
    pub signal: String,
    #[serde(default)]
    pub triggered: bool,
    #[serde(default)]
    pub weight: f64,
    #[serde(default)]
    pub contribution: f64,
    #[serde(default)]
    pub severity: String,
    #[serde(default)]
    pub description: String,
}

/// Predictive risk scoring response with dissolution probability.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PredictiveRiskResponse {
    #[serde(default)]
    pub uid: String,
    #[serde(default)]
    pub company_name: String,
    #[serde(default)]
    pub dissolution_probability: f64,
    #[serde(default)]
    pub risk_level: String,
    #[serde(default)]
    pub pre_dissolution_indicators: Vec<PredictiveRiskIndicator>,
    #[serde(default)]
    pub credit_risk_score: i64,
    #[serde(default)]
    pub recommendation: String,
    #[serde(default)]
    pub assessed_at: String,
}

// ---------------------------------------------------------------------------
// PDF Profile
// ---------------------------------------------------------------------------

/// A board member entry in a PDF profile response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PdfBoardMember {
    #[serde(default)]
    pub first_name: Option<String>,
    #[serde(default)]
    pub last_name: Option<String>,
    #[serde(default)]
    pub role: String,
    #[serde(default)]
    pub signing_authority: Option<String>,
    #[serde(default)]
    pub since: Option<String>,
    #[serde(default)]
    pub until: Option<String>,
}

/// A company event in a PDF profile response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PdfEvent {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub severity: String,
    #[serde(default)]
    pub detected_at: String,
    #[serde(default)]
    pub source_date: Option<String>,
}

/// An auditor tenure entry in a PDF profile response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PdfAuditorTenure {
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
}

/// Core company data within a PDF profile response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PdfCompanyData {
    #[serde(default)]
    pub uid: String,
    #[serde(default)]
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
    pub currency: Option<String>,
    #[serde(default)]
    pub purpose: Option<String>,
    #[serde(default)]
    pub founding_date: Option<String>,
    #[serde(default)]
    pub registration_date: Option<String>,
    #[serde(default)]
    pub legal_seat: Option<String>,
    #[serde(default)]
    pub municipality: Option<String>,
    #[serde(default)]
    pub address_street: Option<String>,
    #[serde(default)]
    pub address_house_number: Option<String>,
    #[serde(default)]
    pub address_zip_code: Option<String>,
    #[serde(default)]
    pub address_city: Option<String>,
    #[serde(default)]
    pub website: Option<String>,
    #[serde(default)]
    pub industry: Option<String>,
    #[serde(default)]
    pub sub_industry: Option<String>,
    #[serde(default)]
    pub employee_count: Option<i32>,
    #[serde(default)]
    pub auditor_name: Option<String>,
    #[serde(default)]
    pub auditor_category: Option<String>,
    #[serde(default)]
    pub sanctions_hit: Option<bool>,
    #[serde(default)]
    pub is_finma_regulated: Option<bool>,
    #[serde(default)]
    pub old_names: Option<Vec<String>>,
    #[serde(default)]
    pub translations: Option<Vec<String>>,
}

/// Structured company profile data suitable for PDF rendering.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PdfProfileResponse {
    pub company: PdfCompanyData,
    #[serde(default)]
    pub board_members: Vec<PdfBoardMember>,
    #[serde(default)]
    pub recent_events: Vec<PdfEvent>,
    #[serde(default)]
    pub auditor_history: Vec<PdfAuditorTenure>,
    #[serde(default)]
    pub generated_at: String,
}

// ---------------------------------------------------------------------------
// Reports (Industry)
// ---------------------------------------------------------------------------

/// Summary of an industry with company count.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndustrySummary {
    #[serde(default)]
    pub industry: String,
    #[serde(default)]
    pub company_count: i64,
}

/// List of available industries.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndustryListResponse {
    #[serde(default)]
    pub industries: Vec<IndustrySummary>,
    #[serde(default)]
    pub total: i64,
}

/// A company entry within an industry report.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndustryCompanyEntry {
    #[serde(default)]
    pub uid: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub canton: Option<String>,
    #[serde(default)]
    pub share_capital: Option<f64>,
    #[serde(default)]
    pub status: Option<String>,
}

/// Canton distribution entry in a report.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportCantonCount {
    #[serde(default)]
    pub canton: String,
    #[serde(default)]
    pub count: i64,
}

/// Auditor concentration entry in a report.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportAuditorCount {
    #[serde(default)]
    pub auditor_name: String,
    #[serde(default)]
    pub count: i64,
}

/// Status distribution entry.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusCount {
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub count: i64,
}

/// Detailed industry report with analytics.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndustryReportResponse {
    #[serde(default)]
    pub industry: String,
    #[serde(default)]
    pub company_count: i64,
    #[serde(default)]
    pub avg_capital: Option<f64>,
    #[serde(default)]
    pub median_capital: Option<f64>,
    #[serde(default)]
    pub top_companies: Vec<IndustryCompanyEntry>,
    #[serde(default)]
    pub canton_distribution: Vec<ReportCantonCount>,
    #[serde(default)]
    pub recent_changes: i64,
    #[serde(default)]
    pub auditor_concentration: Vec<ReportAuditorCount>,
    #[serde(default)]
    pub status_distribution: Vec<StatusCount>,
    #[serde(default)]
    pub generated_at: String,
}

/// AI-generated industry narrative report.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneratedIndustryReport {
    #[serde(default)]
    pub industry: String,
    #[serde(default)]
    pub report: String,
    #[serde(default)]
    pub sources: Vec<String>,
    #[serde(default)]
    pub generated_at: String,
}

// ---------------------------------------------------------------------------
// Sanctions
// ---------------------------------------------------------------------------

/// Query parameters for browsing the sanctions database.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SanctionsSearchParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "pageSize")]
    pub page_size: Option<i64>,
}

/// A single sanctions list entry.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SanctionEntry {
    #[serde(default)]
    pub seco_id: String,
    #[serde(default)]
    pub entity_type: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub aliases: Vec<String>,
    #[serde(default)]
    pub nationality: Option<String>,
    #[serde(default)]
    pub date_of_birth: Option<String>,
    #[serde(default)]
    pub address: Option<String>,
    #[serde(default)]
    pub program: String,
    #[serde(default)]
    pub listed_since: Option<String>,
    #[serde(default)]
    pub source_url: String,
}

/// Paginated sanctions browse response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SanctionsListResponse {
    #[serde(default)]
    pub items: Vec<SanctionEntry>,
    #[serde(default)]
    pub total: i64,
    #[serde(default)]
    pub page: i64,
    #[serde(default)]
    pub page_size: i64,
}

// ---------------------------------------------------------------------------
// Pipelines
// ---------------------------------------------------------------------------

/// A sales/tracking pipeline.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pipeline {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub team_id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub stages: Vec<String>,
    #[serde(default)]
    pub created_at: i64,
}

/// An entry (company) within a pipeline stage.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PipelineEntry {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub pipeline_id: String,
    #[serde(default)]
    pub company_uid: String,
    #[serde(default)]
    pub company_name: String,
    #[serde(default)]
    pub canton: Option<String>,
    #[serde(default)]
    pub stage: String,
    #[serde(default)]
    pub assigned_to_user_id: Option<String>,
    #[serde(default)]
    pub assigned_to_name: Option<String>,
    #[serde(default)]
    pub tier: i64,
    #[serde(default)]
    pub score: Option<f64>,
    #[serde(default)]
    pub notes: Option<String>,
    #[serde(default)]
    pub created_at: i64,
    #[serde(default)]
    pub updated_at: i64,
}

/// A pipeline with its entries loaded.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PipelineWithEntries {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub team_id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub stages: Vec<String>,
    #[serde(default)]
    pub created_at: i64,
    #[serde(default)]
    pub entries: Vec<PipelineEntry>,
    #[serde(default)]
    pub total_entries: i64,
}

/// Aggregate statistics for a pipeline.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PipelineStats {
    #[serde(default)]
    pub by_stage: HashMap<String, i64>,
    #[serde(default)]
    pub by_tier: HashMap<String, i64>,
    #[serde(default)]
    pub total: i64,
}

/// Request body for creating a pipeline.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePipelineRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stages: Option<Vec<String>>,
}

/// Request body for adding an entry to a pipeline.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddEntryRequest {
    pub company_uid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to_user_id: Option<String>,
}

/// Request body for updating a pipeline entry.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEntryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

// ---------------------------------------------------------------------------
// Saved Searches
// ---------------------------------------------------------------------------

/// A saved search query that can be scheduled or linked to alerts.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedSearch {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub search_params: Option<serde_json::Value>,
    #[serde(default)]
    pub is_scheduled: bool,
    #[serde(default)]
    pub schedule_frequency: Option<String>,
    #[serde(default)]
    pub last_run_at: Option<String>,
    #[serde(default)]
    pub last_result_count: Option<i64>,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
}

/// Request body for creating a saved search.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSavedSearchRequest {
    pub name: String,
    pub search_params: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub is_scheduled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_frequency: Option<String>,
}

/// Request body for updating a saved search.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSavedSearchRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_params: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_scheduled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_frequency: Option<String>,
}

// ---------------------------------------------------------------------------
// Company Diff
// ---------------------------------------------------------------------------

/// A single field-level change in a company diff.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiffEntry {
    pub field: String,
    #[serde(default)]
    pub from: Option<String>,
    #[serde(default)]
    pub to: Option<String>,
    #[serde(default)]
    pub changed_at: String,
    #[serde(default)]
    pub change_type: String,
}

/// Response for a company diff request.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanyDiffResponse {
    pub uid: String,
    pub since: String,
    pub until: String,
    #[serde(default)]
    pub changes: Vec<DiffEntry>,
    #[serde(default)]
    pub total_changes: i64,
}

// ---------------------------------------------------------------------------
// Bulk Profiles Export
// ---------------------------------------------------------------------------

/// Request body for bulk profile export.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkProfilesRequest {
    pub uids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_timeline: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_board: Option<bool>,
}

// ---------------------------------------------------------------------------
// Clone derives for newly-introduced request types (ergonomics)
// ---------------------------------------------------------------------------
