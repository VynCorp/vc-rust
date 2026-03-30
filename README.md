# vynco

[![Crates.io](https://img.shields.io/crates/v/vynco.svg)](https://crates.io/crates/vynco)
[![Documentation](https://docs.rs/vynco/badge.svg)](https://docs.rs/vynco)
[![CI](https://github.com/VynCorp/vc-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/VynCorp/vc-rust/actions/workflows/ci.yml)
[![License: Apache-2.0](https://img.shields.io/crates/l/vynco.svg)](LICENSE)

Rust SDK for the [VynCo](https://vynco.ch) Swiss Corporate Intelligence API.

Access 500,000+ Swiss companies with event tracking, sanctions screening, AI-powered analysis,
watchlists, webhooks, and bulk data exports.

## Installation

```toml
[dependencies]
vynco = "2.0"
```

For the synchronous (blocking) client:

```toml
[dependencies]
vynco = { version = "2.0", features = ["blocking"] }
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
    println!("Found {} companies", resp.data.total);
    println!("Credits used: {:?}", resp.meta.credits_used);

    // Get company by UID
    let company = client.companies().get("CHE-105.805.080").await?;
    println!("{}: {:?}", company.data.name, company.data.legal_form);

    // Sanctions screening
    let screening = client.screening().screen(&vynco::ScreeningRequest {
        name: "Novartis AG".into(),
        uid: None,
        sources: None,
    }).await?;
    println!("Risk level: {}", screening.data.risk_level);

    // AI risk score
    let risk = client.ai().risk_score(&vynco::RiskScoreRequest {
        uid: "CHE-105.805.080".into(),
    }).await?;
    println!("Risk score: {}/100", risk.data.overall_score);

    // Credit balance
    let credits = client.credits().balance().await?;
    println!("Credits remaining: {}", credits.data.balance);

    // Team info
    let team = client.teams().me().await?;
    println!("Team: {} ({})", team.data.name, team.data.tier);

    Ok(())
}
```

### Blocking Client

```rust
use vynco::blocking::Client;

fn main() -> Result<(), vynco::VyncoError> {
    let client = Client::builder("vc_live_your_api_key")
        .build()?;

    let count = client.companies().count()?;
    println!("Companies: {}", count.data.count);

    Ok(())
}
```

## API Coverage

18 resource modules covering 69 endpoints:

| Resource | Methods |
|----------|---------|
| `health()` | `check` |
| `companies()` | `list`, `get`, `count`, `events`, `statistics`, `compare`, `news`, `reports`, `relationships`, `hierarchy`, `fingerprint`, `nearby` |
| `auditors()` | `history`, `tenures` |
| `dashboard()` | `get` |
| `screening()` | `screen` |
| `watchlists()` | `list`, `create`, `delete`, `companies`, `add_companies`, `remove_company`, `events` |
| `webhooks()` | `list`, `create`, `update`, `delete`, `test`, `deliveries` |
| `exports()` | `create`, `get`, `download` |
| `ai()` | `dossier`, `search`, `risk_score` |
| `api_keys()` | `list`, `create`, `revoke` |
| `credits()` | `balance`, `usage`, `history` |
| `billing()` | `create_checkout`, `create_portal` |
| `teams()` | `me`, `create`, `members`, `invite_member`, `update_member_role`, `remove_member`, `billing_summary` |
| `changes()` | `list`, `by_company`, `statistics` |
| `persons()` | `board_members` |
| `analytics()` | `statistics`, `cantons`, `auditors`, `cluster`, `anomalies`, `rfm_segments`, `cohorts`, `candidates` |
| `dossiers()` | `create`, `list`, `get`, `delete` |
| `graph()` | `get`, `export`, `analyze` |

## Response Metadata

Every response includes header metadata for credit tracking and rate limiting:

```rust
let resp = client.companies().get("CHE-105.805.080").await?;

println!("Request ID: {:?}", resp.meta.request_id);         // X-Request-Id
println!("Credits used: {:?}", resp.meta.credits_used);      // X-Credits-Used
println!("Credits remaining: {:?}", resp.meta.credits_remaining); // X-Credits-Remaining
println!("Rate limit: {:?}", resp.meta.rate_limit_limit);    // X-Rate-Limit-Limit
println!("Rate limit remaining: {:?}", resp.meta.rate_limit_remaining); // X-RateLimit-Remaining
println!("Rate limit reset: {:?}", resp.meta.rate_limit_reset); // X-RateLimit-Reset
println!("Data source: {:?}", resp.meta.data_source);        // X-Data-Source
```

## Example CLI

A full CLI example is included to demonstrate real-world SDK usage:

```bash
export VYNCO_API_KEY="vc_live_your_api_key"

cargo run --example vynco_cli -- health                            # API health check
cargo run --example vynco_cli -- companies --canton ZH --search "Novartis"  # List with filters
cargo run --example vynco_cli -- company CHE-105.805.649           # Lookup by UID
cargo run --example vynco_cli -- count                             # Count companies
cargo run --example vynco_cli -- events CHE-105.805.649 --limit 10 # Company events
cargo run --example vynco_cli -- screen "Test Corp"                # Sanctions screening
cargo run --example vynco_cli -- dashboard                         # Admin dashboard
cargo run --example vynco_cli -- auditors --min-years 10 --canton ZH  # Long-tenure auditors
cargo run --example vynco_cli -- risk CHE-105.805.649              # AI risk score
cargo run --example vynco_cli -- credits                           # Credit balance
cargo run --example vynco_cli -- team                              # Team info
cargo run --example vynco_cli -- changes --page 1 --page-size 10  # Recent SOGC changes
cargo run --example vynco_cli -- board-members CHE-105.805.649     # Board members
```

See [`examples/vynco_cli.rs`](examples/vynco_cli.rs) for the full source.

## Configuration

```rust
use std::time::Duration;

let client = Client::builder("vc_live_your_api_key")
    .base_url("https://api.vynco.ch")              // default
    .timeout(Duration::from_secs(60))               // default: 30s
    .max_retries(3)                                 // default: 2
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
    Err(VyncoError::NotFound(body)) => println!("Not found: {:?}", body.detail),
    Err(VyncoError::Validation(body)) => println!("Bad request: {:?}", body.detail),
    Err(VyncoError::Conflict(_)) => println!("Resource conflict"),
    Err(VyncoError::RateLimit(_)) => println!("Rate limited, retry later"),
    Err(VyncoError::Server(_)) => println!("Server error"),
    Err(e) => eprintln!("Error: {e}"),
}
```

Error bodies follow [RFC 7807 Problem Details](https://tools.ietf.org/html/rfc7807) with
`error_type`, `title`, `status`, `detail`, and `instance` fields.

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
