use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Pagination
// ---------------------------------------------------------------------------

/// Paginated response wrapper used by list endpoints.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: u64,
    pub page: u32,
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
    pub legal_seat: String,
    #[serde(default)]
    pub canton: String,
    #[serde(default)]
    pub legal_form: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub purpose: String,
    pub capital_nominal: Option<f64>,
    pub capital_currency: Option<String>,
    pub auditor_name: Option<String>,
    pub registration_date: Option<String>,
    pub deletion_date: Option<String>,
    #[serde(default)]
    pub data_source: String,
    #[serde(default)]
    pub last_modified: String,
}

/// Search parameters for querying companies.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanySearchParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canton: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_form: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_desc: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
}

/// A change recorded against a company.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanyChange {
    pub id: String,
    pub company_uid: String,
    #[serde(default)]
    pub change_type: String,
    #[serde(default)]
    pub field_name: String,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    #[serde(default)]
    pub detected_at: String,
    pub source_date: Option<String>,
}

/// A person's role within a company (board member, director, etc.).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonRole {
    pub person_id: String,
    #[serde(default)]
    pub first_name: String,
    #[serde(default)]
    pub last_name: String,
    #[serde(default)]
    pub role: String,
    pub since: Option<String>,
    pub until: Option<String>,
}

/// Company count response.
#[derive(Debug, Clone, Deserialize)]
pub struct CompanyCount {
    pub count: u64,
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
    pub first_name: String,
    #[serde(default)]
    pub last_name: String,
    #[serde(default)]
    pub roles: Vec<PersonRole>,
}

/// Search parameters for finding persons.
#[derive(Debug, Clone, Serialize)]
pub struct PersonSearchParams {
    pub name: String,
}

// ---------------------------------------------------------------------------
// Dossiers
// ---------------------------------------------------------------------------

/// An AI-generated company dossier.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dossier {
    pub id: String,
    pub company_uid: String,
    #[serde(default)]
    pub status: String,
    pub executive_summary: Option<String>,
    #[serde(default)]
    pub key_insights: Option<Vec<String>>,
    #[serde(default)]
    pub risk_factors: Option<Vec<String>>,
    pub generated_at: Option<String>,
}

/// Request to generate a dossier.
#[derive(Debug, Clone, Serialize)]
pub struct GenerateDossierRequest {
    /// Dossier level: "summary", "standard", or "comprehensive".
    pub level: String,
}

// ---------------------------------------------------------------------------
// API Keys
// ---------------------------------------------------------------------------

/// An existing API key (secret is redacted).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyInfo {
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub key_prefix: String,
    #[serde(default)]
    pub key_hint: String,
    #[serde(default)]
    pub permissions: Vec<String>,
    #[serde(default)]
    pub is_active: bool,
    pub last_used_at: Option<String>,
    #[serde(default)]
    pub created_at: String,
    pub expires_at: Option<String>,
}

/// A newly created API key (includes the full secret, shown only once).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyCreated {
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub raw_key: String,
    #[serde(default)]
    pub key_prefix: String,
    #[serde(default)]
    pub permissions: Vec<String>,
    #[serde(default)]
    pub created_at: String,
    pub expires_at: Option<String>,
}

/// Request to create an API key.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateApiKeyRequest {
    pub name: String,
    pub is_test: bool,
    pub permissions: Vec<String>,
}

// ---------------------------------------------------------------------------
// Credits & Billing
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
    pub period: Option<UsagePeriod>,
}

/// A single operation's credit usage.
#[derive(Debug, Clone, Deserialize)]
pub struct UsageOperation {
    #[serde(default)]
    pub operation: String,
    #[serde(default)]
    pub count: u64,
    #[serde(default)]
    pub credits: i64,
}

/// Time period for usage data.
#[derive(Debug, Clone, Deserialize)]
pub struct UsagePeriod {
    pub start: String,
    pub end: String,
}

/// Response containing a Stripe checkout session URL.
#[derive(Debug, Clone, Deserialize)]
pub struct CheckoutSessionResponse {
    pub url: String,
}

/// Response containing a Stripe billing portal URL.
#[derive(Debug, Clone, Deserialize)]
pub struct PortalSessionResponse {
    pub url: String,
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
}

/// Request to create a new team.
#[derive(Debug, Clone, Serialize)]
pub struct CreateTeamRequest {
    pub name: String,
    pub slug: String,
}

// ---------------------------------------------------------------------------
// Webhooks
// ---------------------------------------------------------------------------

/// A webhook subscription.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Webhook {
    pub id: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub events: Vec<String>,
    #[serde(default)]
    pub status: String,
    pub secret: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

/// A newly created webhook (includes the signing secret, shown only once).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookCreated {
    pub id: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub events: Vec<String>,
    #[serde(default)]
    pub secret: String,
    pub created_at: Option<String>,
}

/// Request to create a webhook.
#[derive(Debug, Clone, Serialize)]
pub struct CreateWebhookRequest {
    pub url: String,
    pub events: Vec<String>,
}

/// Request to update an existing webhook.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWebhookRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

// ---------------------------------------------------------------------------
// Users & Settings
// ---------------------------------------------------------------------------

/// The authenticated user's profile.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserProfile {
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub avatar: String,
    #[serde(default)]
    pub plan: String,
    #[serde(default)]
    pub credit_balance: i64,
}

/// Request to update the user's profile.
#[derive(Debug, Clone, Serialize)]
pub struct UpdateProfileRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}
