# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What This Is

Rust SDK (`vynco` crate) for the VynCo Swiss Corporate Intelligence API. Covers 52 public API endpoints across 14 resource modules. Aligned with the VynCo OpenAPI 1.0.0 specification.

## Commands

```bash
cargo build                    # Build (async client only)
cargo build --features blocking # Build with synchronous client
cargo test                     # Run all tests
cargo test test_name           # Run a single test by name
cargo test -- --nocapture      # Run tests with stdout visible
```

## Architecture

**Core flow:** `Client` (builder + HTTP + retry) → `Resource<'a>` (borrows client) → `Response<T>` (data + header metadata)

### Key Patterns

**Resource borrowing:** All 14 resources borrow `&Client` via lifetime `'a`. No cloning. Access via `client.companies().list(params).await?`.

**Response wrapper:** Every API call returns `Response<T>` containing both `data: T` and `meta: ResponseMeta` (parsed from `X-Request-Id`, `X-Credits-Used`, `X-Credits-Remaining`, `X-Rate-Limit-Limit`, `X-RateLimit-Remaining`, `X-RateLimit-Reset`, `X-Data-Source` headers).

**Four request methods on Client** (all `pub(crate)`):
- `request<T>()` — GET with no body
- `request_with_body<T, B>()` — POST/PUT with JSON body
- `request_with_params<T>()` — GET with query parameters
- `request_empty()` — DELETE, returns only `ResponseMeta`

**Retry logic:** Exponential backoff (500ms × 2^attempt) on HTTP 429 and 5xx. Respects `Retry-After` header. Configurable via `max_retries` (default: 2).

**Flexible list extraction:** `Client::extract_list<T>()` handles three API response shapes: bare JSON arrays, `{"data": [...]}`, or first array-valued key in an object.

**Blocking client:** `src/blocking.rs` wraps the async client with a single-threaded Tokio runtime. Each resource gets a synchronous wrapper that calls `block_on()`. Behind `blocking` feature flag. Cannot be used from within an existing async context.

**Error mapping:** HTTP status → `VyncoError` variant: 401→Authentication, 402→InsufficientCredits, 403→Forbidden, 404→NotFound, 400/422→Validation, 409→Conflict, 429→RateLimit, 5xx→Server. Error bodies follow RFC 7807 ProblemDetails with `error_type`, `title`, `detail`, `message`, `status` fields.

### Resources (14 modules, 52 endpoints)

| Resource | Endpoints |
|----------|-----------|
| `companies` | `list`, `get`, `count`, `statistics`, `search` (FTS), `batch`, `compare` |
| `changes` | `list`, `by_company`, `statistics`, `by_sogc`, `review`, `batch` |
| `persons` | `list`, `get`, `roles`, `connections`, `board_members`, `network_stats` |
| `dossiers` | `list`, `get`, `generate`, `statistics` |
| `relationships` | `for_company`, `hierarchy` |
| `news` | `for_company`, `recent` |
| `reports` | `for_company` |
| `analytics` | `cluster`, `anomalies`, `cohorts`, `cantons`, `auditors`, `rfm_segments`, `velocity` |
| `watches` | `list`, `create`, `remove`, `notifications` |
| `api_keys` | `list`, `create`, `revoke` |
| `credits` | `balance`, `usage`, `history` |
| `billing` | `create_checkout`, `create_portal` |
| `teams` | `me`, `create`, `members`, `invite_member`, `update_member_role`, `remove_member`, `billing_summary` |
| `health` | `check` |

### Serde Conventions

- `#[serde(rename_all = "camelCase")]` on all types (API uses camelCase)
- `#[serde(default)]` on fields that may be absent
- `#[serde(skip_serializing_if = "Option::is_none")]` on optional request params

## Testing

Tests use `mockito` for HTTP mocking. Pattern:

```rust
#[tokio::test]
async fn test_name() {
    let mut server = mockito::Server::new_async().await;
    let mock = server.mock("GET", "/path").with_status(200).with_body(json).create_async().await;
    let client = Client::builder("vc_test_key").base_url(server.url()).build().unwrap();
    // ... assertions ...
    mock.assert_async().await;
}
```

Set `max_retries(0)` when testing error status codes to avoid retry delays.

## API Details

- **Base URL:** `https://api.vynco.ch/api/v1`
- **Auth:** Bearer tokens — API keys (`vc_live_*` production, `vc_test_*` sandbox) or Entra ID JWTs
- **OpenAPI spec:** Located at `/home/michael/DEV/Repos/ZefixMiner/EY.EW.ASU.ZefixMiner/src/ZefixMiner.Functions.Api/openapi.json`
- **Design spec:** `docs/superpowers/specs/2026-03-17-vynco-rust-sdk-design.md`
