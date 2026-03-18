use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Pagination
// ---------------------------------------------------------------------------

/// Paginated response wrapper used by list endpoints.
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

// ---------------------------------------------------------------------------
// Companies
// ---------------------------------------------------------------------------

/// A Swiss company record from the Zefix registry.
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

// ---------------------------------------------------------------------------
// Changes
// ---------------------------------------------------------------------------

/// A change recorded against a company.
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

// ---------------------------------------------------------------------------
// Persons
// ---------------------------------------------------------------------------

/// A person with their associated company roles.
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

// ---------------------------------------------------------------------------
// Dossiers
// ---------------------------------------------------------------------------

/// An AI-generated company dossier.
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

// ---------------------------------------------------------------------------
// Relationships
// ---------------------------------------------------------------------------

/// A relationship between two companies.
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

// ---------------------------------------------------------------------------
// News
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// Reports
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// Watches & Notifications
// ---------------------------------------------------------------------------

/// A company watch subscription.
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

/// A change notification.
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

// ---------------------------------------------------------------------------
// Analytics
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// API Keys
// ---------------------------------------------------------------------------

/// An existing API key (key value is redacted).
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

// ---------------------------------------------------------------------------
// Credits
// ---------------------------------------------------------------------------

/// Current credit balance and tier information.
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

/// Credit usage breakdown by operation type.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsageBreakdown {
    #[serde(default)]
    pub operations: Vec<UsageOperation>,
    #[serde(default)]
    pub total_debited: i64,
    pub period: Option<String>,
}

/// A single operation's credit usage.
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

// ---------------------------------------------------------------------------
// Billing
// ---------------------------------------------------------------------------

/// Response containing a Stripe session URL (checkout or portal).
#[derive(Debug, Clone, Deserialize)]
pub struct SessionUrlResponse {
    pub url: String,
}

/// Request body for creating a checkout session.
#[derive(Debug, Clone, Serialize)]
pub struct CheckoutRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}

// ---------------------------------------------------------------------------
// Teams
// ---------------------------------------------------------------------------

/// A team/organization in VynCo.
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

/// Request to create a new team.
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

/// A team member.
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

/// Credit usage breakdown per team member.
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

// ---------------------------------------------------------------------------
// Health
// ---------------------------------------------------------------------------

/// API health status response.
#[derive(Debug, Clone, Deserialize)]
pub struct HealthResponse {
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub uptime: String,
    #[serde(default)]
    pub checks: Vec<HealthCheck>,
}

/// Individual health check result.
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
