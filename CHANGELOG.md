# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **Example CLI app** (`examples/vynco_cli.rs`): interactive command-line tool demonstrating
  health checks, credit balance, team info, company listing/search/lookup/count, statistics,
  and change tracking — serves as both an integration test and a usage reference

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

[1.0.0]: https://github.com/VynCorp/vc-rust/compare/v0.1.0...v1.0.0
[0.1.0]: https://github.com/VynCorp/vc-rust/releases/tag/v0.1.0
