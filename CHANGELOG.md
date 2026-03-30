# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [2.0.0] - 2026-03-30

### Added

- 6 new resource modules (22 endpoints): auditors, dashboard, screening, watchlists, webhooks, exports, ai
- `Client::request_bytes()` for binary file downloads
- `ExportFile` type for downloaded export data
- Company events via `companies().events(uid, limit)` (CloudEvents format)
- `ErrorBody.instance` field (RFC 7807)

### Changed

- Base URL: `https://api.vynco.ch/api/v1` → `https://api.vynco.ch`
- Serde: Dropped `rename_all = "camelCase"` — API uses snake_case natively
- `HealthResponse`: now `status`, `database`, `redis`, `version` (was `status`, `uptime`, `checks`)
- `Company`: removed `address`, `purpose`; added `share_capital`, `industry`; fields now `Option<String>`
- `PagedResponse<T>`: `total_count: u64` → `total: i64`, `page_size: u32` → `page_size: i64`
- `CompanyListParams`: removed `status`, `auditor_category`, `sort_by`, `sort_desc`, `target_status`; added `changed_since`; page types i64
- `ErrorBody`: `detail: String` → `Option<String>`; removed `message`; added `instance: Option<String>`
- Companies `count()` no longer takes params

### Removed

- 12 resource modules: analytics, api_keys, billing, changes, credits, dossiers, news, persons, relationships, reports, teams, watches
- `Client::extract_list()`
- All v1-only types

## [1.0.0] - 2026-03-18

First stable release, aligned with the VynCo OpenAPI 1.0.0 specification.

### Added

- **7 new resource modules** (23 endpoints):
  - `changes()` — list, by_company, statistics, by_sogc, review, batch
  - `analytics()` — cluster, anomalies, cohorts, cantons, auditors, rfm_segments, velocity
  - `watches()` — list, create, remove, notifications
  - `news()` — for_company, recent
  - `reports()` — for_company
  - `relationships()` — for_company, hierarchy
  - `health()` — check
- **New company endpoints**: full-text search (POST), batch lookup, compare
- **Expanded persons**: list, roles, connections, board_members, network_stats
- **Expanded dossiers**: list, get, statistics
- **Expanded teams**: members, invite_member, update_member_role, remove_member, billing_summary
- **`VyncoError::Conflict`** variant for HTTP 409
- **RFC 7807 ProblemDetails** fields on `ErrorBody` (`error_type`, `title`)
- **Rate limit headers** on `ResponseMeta` (`rate_limit_remaining`, `rate_limit_reset`)
- **CI/CD** via GitHub Actions with automated crates.io publishing on tags

### Changed

- **Base URL**: `/v1` → `/api/v1`
- **`PaginatedResponse`**: `total` → `total_count`
- **`Company`** model: replaced `legal_seat`, `capital_nominal`, `capital_currency`, `auditor_name`, `registration_date`, `deletion_date`, `data_source`, `last_modified` with `address`, `auditor_category`, `created_at`, `updated_at`
- **`Person`** model: simplified to `name`, `roles: Vec<String>`, `companies: Vec<String>`
- **`Dossier`** model: now uses `company_name`, `summary`, `risk_score`
- **`GenerateDossierRequest`**: `level` → `dossier_type` (serialized as `"type"`)
- **Dossier generation path**: `POST /dossiers` → `POST /dossiers/{uid}/generate`
- **`ApiKeyInfo`** → `ApiKey`; `ApiKeyCreated.raw_key` → `key`
- **`CreateApiKeyRequest`**: simplified (removed `permissions`, `is_test` → `is_test_key`)
- **Billing types**: unified `CheckoutSessionResponse`/`PortalSessionResponse` → `SessionUrlResponse`
- **Billing paths**: `/billing/checkout` → `/billing/checkout-session`, `/billing/portal` → `/billing/portal-session`
- **Companies resource**: `search()` → `list()`, `count()` now takes `CompanyCountParams`
- **Tests**: expanded from 12 → 16 covering new endpoints and error variants

### Removed

- `webhooks()`, `users()`, `settings()` resources (not in final API)
- Types: `Webhook`, `WebhookCreated`, `CreateWebhookRequest`, `UpdateWebhookRequest`, `UserProfile`, `UpdateProfileRequest`, `PersonRole`, `PersonSearchParams`, `CompanySearchParams`, `UsagePeriod`

## [0.1.0] - 2026-03-17

Initial release (draft API).

### Added

- Async client with builder pattern
- Blocking client behind `blocking` feature flag
- 10 resource modules (30 endpoints)
- Response metadata wrapper (`Response<T>`) with 5 API headers
- Typed error handling with `VyncoError` enum (7 HTTP status variants)
- Automatic retry with exponential backoff on 429/5xx
- Retry-After header support
- TLS backend selection (rustls default, native-tls optional)
- 12 integration tests with mockito

[2.0.0]: https://github.com/VynCorp/vc-rust/compare/v1.0.0...v2.0.0
[1.0.0]: https://github.com/VynCorp/vc-rust/compare/v0.1.0...v1.0.0
[0.1.0]: https://github.com/VynCorp/vc-rust/releases/tag/v0.1.0
