# vynco

Rust SDK for the [VynCo](https://vynco.ch) Swiss Corporate Intelligence API.

## Installation

Add to your `Cargo.toml`:

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

    // List companies
    let params = CompanyListParams {
        search: Some("Novartis".into()),
        canton: Some("BS".into()),
        ..Default::default()
    };
    let resp = client.companies().list(&params).await?;
    println!("Found {} companies", resp.data.total_count);
    println!("Credits used: {:?}", resp.meta.credits_used);

    // Get company details
    let company = client.companies().get("CHE-100.023.968").await?;
    println!("{}: {}", company.data.name, company.data.purpose);

    // Full-text search
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

## Blocking Usage

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

## Resources

| Resource | Methods |
|----------|---------|
| `companies()` | `list`, `get`, `count`, `statistics`, `search`, `batch`, `compare` |
| `changes()` | `list`, `by_company`, `statistics`, `by_sogc`, `review`, `batch` |
| `persons()` | `list`, `get`, `roles`, `connections`, `board_members`, `network_stats` |
| `dossiers()` | `list`, `get`, `generate`, `statistics` |
| `relationships()` | `for_company`, `hierarchy` |
| `news()` | `for_company`, `recent` |
| `reports()` | `for_company` |
| `analytics()` | `cluster`, `anomalies`, `cohorts`, `cantons`, `auditors`, `rfm_segments`, `velocity` |
| `watches()` | `list`, `create`, `remove`, `notifications` |
| `api_keys()` | `list`, `create`, `revoke` |
| `credits()` | `balance`, `usage`, `history` |
| `billing()` | `create_checkout`, `create_portal` |
| `teams()` | `me`, `create`, `members`, `invite_member`, `update_member_role`, `remove_member`, `billing_summary` |
| `health()` | `check` |

## Response Metadata

Every response includes header metadata:

```rust
let resp = client.companies().get("CHE-100.023.968").await?;

println!("Request ID: {:?}", resp.meta.request_id);
println!("Credits used: {:?}", resp.meta.credits_used);
println!("Credits remaining: {:?}", resp.meta.credits_remaining);
println!("Rate limit: {:?}", resp.meta.rate_limit_limit);
println!("Rate limit remaining: {:?}", resp.meta.rate_limit_remaining);
println!("Rate limit reset: {:?}", resp.meta.rate_limit_reset);
println!("Data source: {:?}", resp.meta.data_source);
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

## Error Handling

```rust
use vynco::VyncoError;

match client.companies().get("CHE-000.000.000").await {
    Ok(resp) => println!("{}", resp.data.name),
    Err(VyncoError::NotFound(body)) => println!("Not found: {}", body.detail),
    Err(VyncoError::RateLimit(_)) => println!("Rate limited, try again later"),
    Err(VyncoError::InsufficientCredits(_)) => println!("Top up credits"),
    Err(VyncoError::Authentication(_)) => println!("Check your API key"),
    Err(VyncoError::Conflict(_)) => println!("Resource conflict"),
    Err(e) => eprintln!("Error: {e}"),
}
```

## License

Apache-2.0
