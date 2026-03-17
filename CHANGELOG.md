# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-03-17

### Added

- **Async client** with builder pattern (`Client::builder(api_key).build()`)
- **Blocking client** behind `blocking` feature flag for synchronous usage
- **10 resource modules** covering the VynCo public API:
  - `companies()` — search, get by UID, count, statistics, change history, board members, dossier
  - `persons()` — get by ID, search by name
  - `dossiers()` — generate AI company reports (summary/standard/comprehensive)
  - `api_keys()` — list, create, revoke API keys
  - `credits()` — balance, usage breakdown, transaction history
  - `billing()` — Stripe checkout and portal sessions
  - `webhooks()` — list, create, get, update, delete, test
  - `teams()` — get current team, create team
  - `users()` — get profile, update profile
  - `settings()` — get and update user preferences
- **Response metadata** via `Response<T>` wrapper exposing API headers:
  - `X-Request-Id` — request tracing
  - `X-Credits-Used` — credits consumed
  - `X-Credits-Remaining` — remaining balance
  - `X-Rate-Limit-Limit` — tier rate limit
  - `X-Data-Source` — OGD compliance (Zefix/LINDAS)
- **Typed error handling** with `VyncoError` enum mapping HTTP status codes:
  - `Authentication` (401), `InsufficientCredits` (402), `Forbidden` (403)
  - `NotFound` (404), `Validation` (400/422), `RateLimit` (429), `Server` (5xx)
- **Automatic retry** with exponential backoff on 429 and 5xx responses
- **Retry-After header** support for rate-limited requests
- **TLS backend selection** via `rustls-tls` (default) or `native-tls` features
- **12 integration tests** with mockito covering auth, error mapping, response parsing, and header metadata
