# vynco

[![Crates.io](https://img.shields.io/crates/v/vynco.svg)](https://crates.io/crates/vynco)
[![Documentation](https://docs.rs/vynco/badge.svg)](https://docs.rs/vynco)
[![CI](https://github.com/VynCorp/vc-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/VynCorp/vc-rust/actions/workflows/ci.yml)
[![License: Apache-2.0](https://img.shields.io/crates/l/vynco.svg)](LICENSE)

Rust SDK for the [VynCo](https://vynco.ch) Swiss Corporate Intelligence API.

Access 320,000+ Swiss companies from the Zefix commercial registry with full-text search,
change tracking, relationship mapping, AI-generated dossiers, and advanced analytics.

## Installation

```toml
[dependencies]
vynco = "1.0"
```

For the synchronous (blocking) client:

```toml
[dependencies]
vynco = { version = "1.0", features = ["blocking"] }
```

## Quick Start

```rust
use vynco::{Client, CompanyListParams};

#[tokio::main]
async fn main() -> Result<(), vynco::VyncoError> {
    let client = Client::builder("vc_live_your_api_key")
        .build()?;

    // List companies with filtering
    let params = CompanyListParams {
        search: Some("Novartis".into()),
        canton: Some("BS".into()),
        ..Default::default()
    };
    let resp = client.companies().list(&params).await?;
    println!("Found {} companies", resp.data.total_count);
    println!("Credits used: {:?}", resp.meta.credits_used);

    // Get company by UID
    let company = client.companies().get("CHE-100.023.968").await?;
    println!("{}: {}", company.data.name, company.data.purpose);

    // Full-text search (FTS5)
    let results = client.companies().search(&vynco::CompanySearchRequest {
        query: "pharma".into(),
        limit: Some(10),
    }).await?;
    println!("Search returned {} results", results.data.len());

    // Check credit balance
    let balance = client.credits().balance().await?;
    println!("Balance: {} credits", balance.data.balance);

    Ok(())
}
```

### Blocking Client

```rust
use vynco::blocking::Client;

fn main() -> Result<(), vynco::VyncoError> {
    let client = Client::builder("vc_live_your_api_key")
        .build()?;

    let balance = client.credits().balance()?;
    println!("Balance: {} credits", balance.data.balance);

    Ok(())
}
```

## API Coverage

14 resource modules covering 52 endpoints:

| Resource | Methods | Credits |
|----------|---------|---------|
| `companies()` | `list`, `get`, `count`, `statistics`, `search`, `batch`, `compare` | 1-5 |
| `changes()` | `list`, `by_company`, `statistics`, `by_sogc`, `review`, `batch` | 0-2 |
| `persons()` | `list`, `get`, `roles`, `connections`, `board_members`, `network_stats` | 3-10 |
| `dossiers()` | `list`, `get`, `generate`, `statistics` | 0-100 |
| `relationships()` | `for_company`, `hierarchy` | 10 |
| `news()` | `for_company`, `recent` | 1-2 |
| `reports()` | `for_company` | 5 |
| `analytics()` | `cluster`, `anomalies`, `cohorts`, `cantons`, `auditors`, `rfm_segments`, `velocity` | 3-25 |
| `watches()` | `list`, `create`, `remove`, `notifications` | 0 |
| `api_keys()` | `list`, `create`, `revoke` | 0 |
| `credits()` | `balance`, `usage`, `history` | 0 |
| `billing()` | `create_checkout`, `create_portal` | 0 |
| `teams()` | `me`, `create`, `members`, `invite_member`, `update_member_role`, `remove_member`, `billing_summary` | 0 |
| `health()` | `check` | 0 |

## Response Metadata

Every response includes header metadata for credit tracking and rate limiting:

```rust
let resp = client.companies().get("CHE-100.023.968").await?;

println!("Request ID: {:?}", resp.meta.request_id);         // X-Request-Id
println!("Credits used: {:?}", resp.meta.credits_used);      // X-Credits-Used
println!("Credits remaining: {:?}", resp.meta.credits_remaining); // X-Credits-Remaining
println!("Rate limit: {:?}", resp.meta.rate_limit_limit);    // X-Rate-Limit-Limit
println!("Rate limit remaining: {:?}", resp.meta.rate_limit_remaining); // X-RateLimit-Remaining
println!("Rate limit reset: {:?}", resp.meta.rate_limit_reset); // X-RateLimit-Reset
println!("Data source: {:?}", resp.meta.data_source);        // X-Data-Source
```

## Configuration

```rust
use std::time::Duration;

let client = Client::builder("vc_live_your_api_key")
    .base_url("https://api.vynco.ch/api/v1")  // default
    .timeout(Duration::from_secs(60))           // default: 30s
    .max_retries(3)                             // default: 2
    .build()?;
```

The client automatically retries on HTTP 429 (rate limited) and 5xx (server error) with
exponential backoff (500ms x 2^attempt). It respects the `Retry-After` header when present.

## Error Handling

All API errors are mapped to typed variants:

```rust
use vynco::VyncoError;

match client.companies().get("CHE-000.000.000").await {
    Ok(resp) => println!("{}", resp.data.name),
    Err(VyncoError::Authentication(_)) => println!("Invalid API key"),
    Err(VyncoError::InsufficientCredits(_)) => println!("Top up credits"),
    Err(VyncoError::Forbidden(_)) => println!("Insufficient permissions"),
    Err(VyncoError::NotFound(body)) => println!("Not found: {}", body.detail),
    Err(VyncoError::Validation(body)) => println!("Bad request: {}", body.detail),
    Err(VyncoError::Conflict(_)) => println!("Resource conflict"),
    Err(VyncoError::RateLimit(_)) => println!("Rate limited, retry later"),
    Err(VyncoError::Server(_)) => println!("Server error"),
    Err(e) => eprintln!("Error: {e}"),
}
```

Error bodies follow [RFC 7807 Problem Details](https://tools.ietf.org/html/rfc7807) with
`error_type`, `title`, `detail`, `status`, and `message` fields.

## Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `rustls-tls` | Yes | Use rustls for TLS (no OpenSSL dependency) |
| `native-tls` | No | Use the platform's native TLS stack |
| `blocking` | No | Enable the synchronous blocking client |

## Minimum Supported Rust Version

Rust 1.83 or later.

## License

Apache-2.0
