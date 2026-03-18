# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2026-03-18

### Breaking Changes

- **Base URL** changed from `/v1` to `/api/v1` to match production API
- **`PaginatedResponse`** field renamed: `total` → `total_count`
- **`Company`** model restructured: removed `legal_seat`, `capital_nominal`, `capital_currency`, `auditor_name`, `registration_date`, `deletion_date`, `data_source`, `last_modified`; added `address`, `auditor_category`, `created_at`, `updated_at`
- **`Person`** model simplified: `first_name`/`last_name` → `name`, `roles` is now `Vec<String>`, added `companies: Vec<String>`
- **`Dossier`** model restructured: removed `id`, `status`, `executive_summary`, `key_insights`, `risk_factors`; added `company_name`, `summary`, `risk_score`
- **`GenerateDossierRequest`** field renamed: `level` → `dossier_type` (serialized as `"type"`)
- **Dossier generation** path changed from `POST /dossiers` to `POST /dossiers/{uid}/generate`
- **`ApiKeyInfo`** renamed to `ApiKey` with simplified fields; `ApiKeyCreated.raw_key` → `key`
- **`CreateApiKeyRequest`** simplified: removed `permissions`, `is_test` → `is_test_key`
- **`UsageBreakdown.period`** changed from `Option<UsagePeriod>` to `Option<String>`; removed `UsagePeriod` and `UsageOperation.count`
- **Billing types** unified: `CheckoutSessionResponse`/`PortalSessionResponse` → `SessionUrlResponse`; billing methods now take `CheckoutRequest` instead of `&str`
- **Billing paths** changed: `/billing/checkout` → `/billing/checkout-session`, `/billing/portal` → `/billing/portal-session`
- **`CreateTeamRequest`** now includes optional `owner_email` and `owner_name`
- **`Team`** now includes `updated_at`
- **Companies resource**: `search()` renamed to `list()`, old `count()` now takes `CompanyCountParams` instead of `CompanySearchParams`
- **Removed resources**: `webhooks()`, `users()`, `settings()` — not in final API spec

### Added

- **7 new resource modules** with 23 new endpoints:
  - `changes()` — `list`, `by_company`, `statistics`, `by_sogc`, `review`, `batch`
  - `analytics()` — `cluster`, `anomalies`, `cohorts`, `cantons`, `auditors`, `rfm_segments`, `velocity`
  - `watches()` — `list`, `create`, `remove`, `notifications`
  - `news()` — `for_company`, `recent`
  - `reports()` — `for_company`
  - `relationships()` — `for_company`, `hierarchy`
  - `health()` — `check`
- **New company endpoints**: `search` (POST full-text), `batch` (multi-UID lookup), `compare` (side-by-side)
- **Expanded persons**: `list`, `roles`, `connections`, `board_members`, `network_stats`
- **Expanded dossiers**: `list`, `get`, `statistics`
- **Expanded teams**: `members`, `invite_member`, `update_member_role`, `remove_member`, `billing_summary`
- **New types**: `CompanyStatistics`, `CompanyRelationship`, `RelationshipResponse`, `CompanyWatch`, `CreateWatchRequest`, `ChangeNotification`, `TeamMember`, `InviteMemberRequest`, `UpdateMemberRoleRequest`, `BillingSummary`, `MemberUsage`, `CreditLedgerEntry`, `HealthResponse`, `HealthCheck`, `ChangeStatistics`, `ReviewChangeRequest`, `ReviewChangeResponse`, `BatchChangeRequest`, `CompanySearchRequest`, `BatchCompanyRequest`, `CompareCompaniesRequest`, `ClusterRequest`, `AnomalyRequest`, `CohortParams`, `CompanyNewsResponse`, `RecentNewsResponse`, `CompanyReportsResponse`
- **`ErrorBody`** now includes `error_type` and `title` fields for RFC 7807 compliance
- **`VyncoError::Conflict`** variant for HTTP 409
- **`ResponseMeta`** now includes `rate_limit_remaining` (`X-RateLimit-Remaining`) and `rate_limit_reset` (`X-RateLimit-Reset`)
- **16 integration tests** covering all error variants, company endpoints, changes, health, API keys, credits, and response metadata

### Removed

- `Webhooks` resource and related types (`Webhook`, `WebhookCreated`, `CreateWebhookRequest`, `UpdateWebhookRequest`)
- `Users` resource and related types (`UserProfile`, `UpdateProfileRequest`)
- `Settings` resource
- `PersonRole` type (person roles are now `Vec<String>`)
- `PersonSearchParams` type (replaced by `PersonListParams`)
- `CompanySearchParams` type (replaced by `CompanyListParams` and `CompanyCountParams`)
- `UsagePeriod` type

## [0.1.0] - 2026-03-17

### Added

- **Async client** with builder pattern (`Client::builder(api_key).build()`)
- **Blocking client** behind `blocking` feature flag for synchronous usage
- **10 resource modules** covering the VynCo draft API
- **Response metadata** via `Response<T>` wrapper exposing API headers
- **Typed error handling** with `VyncoError` enum mapping HTTP status codes
- **Automatic retry** with exponential backoff on 429 and 5xx responses
- **Retry-After header** support for rate-limited requests
- **TLS backend selection** via `rustls-tls` (default) or `native-tls` features
- **12 integration tests** with mockito
