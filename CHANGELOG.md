# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [2.3.0] - 2026-04-12

Alignment release matching the reference Python SDK (`vynco` v3.1.0). Adds the
full v3.1 API surface: historical timelines, UBO resolution, similar companies,
media-with-sentiment, person-centric networks, market flow analytics, canton
migrations, industry benchmarking, batch operations, and saved alerts.

### Added

**2 new resources:**

- **`client.alerts()`** — saved queries with optional webhook delivery (`list`, `create`, `delete`)
- **`client.ownership()`** — ownership-chain trace with circular-ownership detection (`trace`)

**14 new methods on existing resources:**

- `companies.timeline(uid, &params)` — chronological event timeline
- `companies.timeline_summary(uid, &params)` — AI-generated narrative summary
- `companies.similar(uid, limit)` — similar-company scoring on industry/canton/capital/legal form/auditor tier
- `companies.ubo(uid)` — ultimate beneficial owner resolution
- `companies.media(uid, &params)` — news items with optional sentiment filter
- `companies.media_analyze(uid)` — trigger LLM sentiment analysis
- `companies.export_csv(req)` — canonical CSV export (replaces `export_excel`, kept as deprecated alias)
- `persons.network(id)` — person-centric network (companies + co-directors + stats)
- `persons.board_members_paged(uid, &params)` — paginated variant (unpaginated `board_members` unchanged)
- `analytics.flows(&params)` — registration/dissolution flows over time
- `analytics.migrations(since)` — canton-to-canton legal-seat migrations
- `analytics.benchmark(&params)` — company vs industry-peer percentile ranks
- `screening.batch(req)` — batch sanctions screening (up to 100 UIDs)
- `ai.risk_score_batch(req)` — batch AI risk scoring (up to 50 UIDs)

**New types (25):**

- `HierarchyEntity` (replaces `serde_json::Value` on `HierarchyResponse.parent/subsidiaries/siblings`)
- Timeline: `TimelineEvent`, `TimelineResponse`, `TimelineParams`, `TimelineSummaryResponse`
- Similar: `SimilarCompanyResult`, `SimilarCompaniesResponse`
- UBO/Ownership: `UboPerson`, `ChainLink`, `UboResponse`, `OwnershipRequest`, `OwnershipEntity`, `OwnershipLink`, `PersonCompanyRole`, `KeyPerson`, `CircularFlag`, `OwnershipResponse`
- Media: `MediaItem`, `MediaResponse`, `MediaParams`, `MediaAnalysisResponse`
- Alerts: `Alert`, `CreateAlertRequest`
- Flows/Migrations/Benchmark: `FlowDataPoint`, `FlowsResponse`, `FlowsParams`, `MigrationFlow`, `MigrationResponse`, `BenchmarkDimension`, `BenchmarkResponse`, `BenchmarkParams`
- Batch: `BatchScreeningRequest`, `BatchScreeningHitSummary`, `BatchScreeningResultByUid`, `BatchScreeningResponse`, `BatchRiskScoreRequest`, `RiskScoreResult`, `BatchRiskScoreResponse`
- Person network: `NetworkPerson`, `NetworkCompany`, `CoDirectorCompany`, `CoDirector`, `NetworkStats`, `PersonNetworkResponse`, `BoardMemberParams`
- Enriched watchlist: `WatchlistCompanyEntry`

**Enrichment provenance fields** (all optional, backwards-compatible — populated
by new backend pipelines):

- `Company.direct_parent_lei`, `ultimate_parent_lei`, `ultimate_parent_name`, `gleif_parent_enriched_at` — GLEIF parent linkage
- `Company.industry_source`, `industry_confidence`, `industry_classified_at` — industry-classification provenance
- `Classification.industry_source`, `industry_confidence` — same on the classification endpoint
- `Fingerprint.registration_date` — Swiss register entry date
- `BoardMember.role_source`, `role_confidence`, `role_inferred_at` — role extraction provenance
- `PersonRoleDetail`, `PersonEntry`, `NetworkCompany` — same role provenance fields

**Data-coverage disclosure:**

- `UboResponse.data_coverage_note` — human-readable explanation when chain is incomplete
- `FlowsResponse.data_coverage_note` — surfaces asymmetric-accuracy notes (e.g. historical dissolution under-counting)

**Documentation:**

- Non-Swiss GLEIF parents appear as synthetic `LEI:<20-char-lei>` identifiers on
  `UboPerson.controlling_entity_uid`, `ChainLink.from_uid`/`to_uid`, and
  `OwnershipLink.source_uid`/`target_uid`. Documented on each struct.

### Changed

- `HierarchyResponse.parent`/`subsidiaries`/`siblings` now use the typed `HierarchyEntity` struct instead of `serde_json::Value` — this is a typed-output improvement; callers that read through the values via `.get("name")` style access must switch to field access (e.g. `entity.name`).
- `WatchlistCompaniesResponse` now includes a typed `companies: Vec<WatchlistCompanyEntry>` field alongside the existing `uids`. The server populates it with name/status/canton for each watched company (eliminates the need for an extra round trip to resolve UIDs).

### Deprecated

- `companies.export_excel(req)` — kept as a deprecated wrapper around `export_csv` (the endpoint has always returned CSV; the new name reflects reality). Emits a `#[deprecated]` warning. Will be removed in 3.0.

## [2.2.0] - 2026-04-08

Production-ready release with full API coverage and live-verified e2e tests.

### Added

- **3 new endpoint bindings** (83 → 86 total):
  - `companies` — `classification`
  - `persons` — `search`, `get`
- **5 new types**: `Classification`, `PersonSearchParams`, `PersonSearchResult`, `PersonDetail`, `PersonRoleDetail`
- **59 e2e integration tests** against the live API, covering all 86 endpoints across 4 tiers (free, session, starter, professional)
- 3 new unit tests (45 → 48 total)

### Changed

- **Base URL**: `api.vynco.ch` → `vynco.ch/api` (matches production deployment)
- **Retry logic**: now falls back to `X-RateLimit-Reset` header when `Retry-After` is absent, capped at 60s — prevents 429 exhaustion under tight rate limits

### Fixed

- Rate-limited requests no longer fail after max retries when the API omits the `Retry-After` header but provides `X-RateLimit-Reset`

## [2.1.0] - 2026-04-02

Aligned SDK with stabilized VynCo API v1.6.0.

### Added

- **14 new endpoint bindings** (69 → 83 total):
  - `companies` — `get_full`, `structure`, `acquisitions`, `notes`, `create_note`, `update_note`, `delete_note`, `tags`, `create_tag`, `delete_tag`, `all_tags`, `export_excel`
  - `dossiers` — `generate`
  - `teams` — `join`
- **18 new types**: `CompanyFullResponse`, `PersonEntry`, `ChangeEntry`, `RelationshipEntry`, `CorporateStructure`, `RelatedCompanyEntry`, `Note`, `CreateNoteRequest`, `UpdateNoteRequest`, `Tag`, `CreateTagRequest`, `TagSummary`, `ExcelExportRequest`, `ExcelExportFilter`, `Acquisition`, `LongestTenure`, `JoinTeamRequest`, `JoinTeamResponse`
- `Client::request_bytes_with_body()` for POST endpoints returning raw bytes (CSV export)
- 8 new tests (37 → 45 total)

### Changed

- **`Company`**: expanded from 9 to 30 fields — added `currency`, `purpose`, `founding_date`, `registration_date`, `deletion_date`, `legal_seat`, `municipality`, `data_source`, `enrichment_level`, `address_street`, `address_house_number`, `address_zip_code`, `address_city`, `address_canton`, `website`, `sub_industry`, `employee_count`, `auditor_name`, `latitude`, `longitude`, `geo_precision`, `noga_code`, `sanctions_hit`, `last_screened_at`, `is_finma_regulated`, `ehraid`, `chid`, `cantonal_excerpt_url`, `old_names`, `translations`
- **`CompanyListParams`**: added `status`, `legal_form`, `capital_min`, `capital_max`, `auditor_category`, `sort_by`, `sort_desc`
- **`DashboardResponse`** sub-types rebuilt to match API:
  - `AuditorTenureStats` — new fields: `total_tracked`, `current_auditors`, `tenures_over_10_years`, `tenures_over_7_years`, `longest_tenure`
  - `DataCompleteness` — new fields: `enriched_companies`, `companies_with_industry`, `companies_with_geo`, `total_persons`, `total_changes`, `total_sogc_publications`
  - `PipelineStatus` — `name`→`id`, `records_processed`→`items_processed`, `last_run`→`last_completed_at`
- **`AiSearchResponse.results`**: `Vec<Company>` → `Vec<serde_json::Value>`
- **Type widening**: `CreditBalance.used_this_month`, `BillingSummary.used_this_month`, `MemberUsage.credits_used` — `i32` → `i64`

### Breaking

- `DashboardResponse` sub-types have different field names (see Changed above)
- `AiSearchResponse.results` is now `Vec<serde_json::Value>` instead of `Vec<Company>`

## [2.0.0] - 2026-03-30

Major version — SDK rewritten to align with the new Rust-based VynCo API.

### Added

- **18 resource modules** covering 69 endpoints:
  - `companies` — list, get, count, events, statistics, compare, news, reports, relationships, hierarchy, fingerprint, nearby
  - `auditors` — history, tenures
  - `dashboard` — get
  - `screening` — screen
  - `watchlists` — list, create, delete, companies, add_companies, remove_company, events
  - `webhooks` — list, create, update, delete, test, deliveries
  - `exports` — create, get, download
  - `ai` — dossier, search, risk_score
  - `api_keys` — list, create, revoke
  - `credits` — balance, usage, history
  - `billing` — create_checkout, create_portal
  - `teams` — me, create, members, invite_member, update_member_role, remove_member, billing_summary
  - `changes` — list, by_company, statistics
  - `persons` — board_members
  - `analytics` — statistics, cantons, auditors, cluster, anomalies, rfm_segments, cohorts, candidates
  - `dossiers` — create, list, get, delete
  - `graph` — get, export, analyze
  - `health` — check
- `Client::request_bytes()` for binary file downloads (exports, graph XML)
- `ExportFile` type for downloaded export data with content-type and filename
- Company events via `companies().events(uid, limit)` (CloudEvents format)
- Network graph and analysis endpoints
- Sanctions screening against SECO, FINMA, OpenSanctions
- AI-powered dossier generation, natural language search, multi-signal risk scoring

### Changed (from v1.0.0)

- **Base URL**: `https://api.vynco.ch/api/v1` → `https://api.vynco.ch`
- **`Company`**: fields now `Option<String>`; added `share_capital`, `industry`; removed `address`, `purpose`
- **`Company` + `PagedResponse<T>`**: now use `#[serde(rename_all = "camelCase")]`
- **`PagedResponse<T>`**: `total_count: u64` → `total: i64`
- **`HealthResponse`**: now `status`, `database`, `redis`, `version` (was `status`, `uptime`, `checks`)
- **`ErrorBody`**: `detail` is now `Option<String>`; removed `message`; added `instance`
- **`ResponseMeta`**: fixed `X-RateLimit-Limit` header name (was `X-Rate-Limit-Limit`)

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

[2.2.0]: https://github.com/VynCorp/vc-rust/compare/v2.1.0...v2.2.0
[2.1.0]: https://github.com/VynCorp/vc-rust/compare/v2.0.0...v2.1.0
[2.0.0]: https://github.com/VynCorp/vc-rust/compare/v1.0.0...v2.0.0
[1.0.0]: https://github.com/VynCorp/vc-rust/compare/v0.1.0...v1.0.0
[0.1.0]: https://github.com/VynCorp/vc-rust/releases/tag/v0.1.0
