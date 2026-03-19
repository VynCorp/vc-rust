//! # VynCo CLI Example
//!
//! A command-line tool demonstrating the VynCo Rust SDK.
//!
//! ## Usage
//!
//! ```bash
//! export VYNCO_API_KEY="vc_live_your_api_key"
//!
//! # Check API health
//! cargo run --example vynco_cli -- health
//!
//! # View credit balance
//! cargo run --example vynco_cli -- credits
//!
//! # Show your team info
//! cargo run --example vynco_cli -- team
//!
//! # List companies (with optional filters)
//! cargo run --example vynco_cli -- companies --canton ZH --search "Novartis"
//!
//! # Full-text search across all companies
//! cargo run --example vynco_cli -- search "pharmaceutical manufacturing"
//!
//! # Get a specific company by UID
//! cargo run --example vynco_cli -- company CHE-105.805.649
//!
//! # Count companies (with optional filters)
//! cargo run --example vynco_cli -- count --canton ZH
//!
//! # View company statistics
//! cargo run --example vynco_cli -- stats
//!
//! # List recent changes
//! cargo run --example vynco_cli -- changes
//! ```

use clap::{Parser, Subcommand};
use vynco::{Client, CompanyCountParams, CompanyListParams, CompanySearchRequest, VyncoError};

#[derive(Parser)]
#[command(name = "vynco")]
#[command(about = "VynCo Swiss Corporate Intelligence CLI — example for the vynco Rust SDK")]
struct Cli {
    /// API key (overrides VYNCO_API_KEY env var)
    #[arg(long, env = "VYNCO_API_KEY")]
    api_key: String,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Check API health status
    Health,

    /// Show credit balance and usage
    Credits,

    /// Show your team information
    Team,

    /// List companies with optional filters
    Companies {
        /// Filter by Swiss canton (e.g. ZH, BE, GE)
        #[arg(long)]
        canton: Option<String>,

        /// Filter by name (substring match)
        #[arg(long)]
        search: Option<String>,

        /// Filter by status (e.g. Active, Dissolved)
        #[arg(long)]
        status: Option<String>,

        /// Page number (default: 1)
        #[arg(long, default_value = "1")]
        page: u32,

        /// Page size (default: 10)
        #[arg(long, default_value = "10")]
        page_size: u32,
    },

    /// Full-text search across all companies
    Search {
        /// Search query
        query: String,

        /// Maximum results to return
        #[arg(long, default_value = "10")]
        limit: u32,
    },

    /// Get a specific company by UID (e.g. CHE-105.805.649)
    Company {
        /// Company UID
        uid: String,
    },

    /// Count companies (with optional filters)
    Count {
        /// Filter by canton
        #[arg(long)]
        canton: Option<String>,

        /// Filter by status
        #[arg(long)]
        status: Option<String>,
    },

    /// Show company database statistics
    Stats,

    /// List recent company changes
    Changes {
        /// Page number
        #[arg(long, default_value = "1")]
        page: u32,

        /// Page size
        #[arg(long, default_value = "10")]
        page_size: u32,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let client = match Client::builder(&cli.api_key).build() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to create client: {e}");
            std::process::exit(1);
        }
    };

    if let Err(e) = run(client, cli.command).await {
        eprintln!("\nError: {e}");
        match &e {
            VyncoError::Authentication(_) => {
                eprintln!("Hint: Check your VYNCO_API_KEY or pass --api-key");
            }
            VyncoError::InsufficientCredits(_) => {
                eprintln!("Hint: Top up your credits at https://vynco.ch");
            }
            VyncoError::RateLimit(_) => {
                eprintln!("Hint: You've hit the rate limit — wait a moment and retry");
            }
            _ => {}
        }
        std::process::exit(1);
    }
}

async fn run(client: Client, command: Command) -> Result<(), VyncoError> {
    match command {
        Command::Health => {
            let resp = client.health().check().await?;
            println!("API Status:  {}", resp.data.status);
            println!("Uptime:      {}", resp.data.uptime);
            if !resp.data.checks.is_empty() {
                println!("\nHealth checks:");
                for check in &resp.data.checks {
                    println!(
                        "  {:<16} {:<10} {}ms  {}",
                        check.name, check.status, check.duration_ms, check.message
                    );
                }
            }
            print_meta(&resp.meta);
        }

        Command::Credits => {
            let resp = client.credits().balance().await?;
            let b = &resp.data;
            println!("Tier:              {}", b.tier);
            println!("Balance:           {} credits", b.balance);
            println!("Monthly allowance: {} credits", b.monthly_credits);
            println!("Used this month:   {} credits", b.used_this_month);
            println!("Overage rate:      {:.2}/credit", b.overage_rate);
            print_meta(&resp.meta);
        }

        Command::Team => {
            let resp = client.teams().me().await?;
            let t = &resp.data;
            println!("Team:       {} ({})", t.name, t.slug);
            println!("ID:         {}", t.id);
            println!("Tier:       {}", t.tier);
            println!(
                "Credits:    {} (monthly: {})",
                t.credit_balance, t.monthly_credits
            );
            println!("Created:    {}", t.created_at);
            print_meta(&resp.meta);
        }

        Command::Companies {
            canton,
            search,
            status,
            page,
            page_size,
        } => {
            let params = CompanyListParams {
                page: Some(page),
                page_size: Some(page_size),
                canton,
                search,
                status,
                ..Default::default()
            };
            let resp = client.companies().list(&params).await?;
            println!(
                "Companies: page {}/{} ({} total)\n",
                resp.data.page,
                (resp.data.total_count as f64 / resp.data.page_size as f64).ceil() as u64,
                resp.data.total_count,
            );
            for c in &resp.data.items {
                println!(
                    "  {:<18} {:<45} {:<6} {:<12} {}",
                    c.uid, c.name, c.canton, c.legal_form, c.status
                );
            }
            print_meta(&resp.meta);
        }

        Command::Search { query, limit } => {
            let req = CompanySearchRequest {
                query: query.clone(),
                limit: Some(limit),
            };
            let resp = client.companies().search(&req).await?;
            println!("Search \"{}\" — {} results:\n", query, resp.data.len());
            for c in &resp.data {
                println!("  {:<18} {:<45} {} ({})", c.uid, c.name, c.canton, c.status);
                if !c.purpose.is_empty() {
                    let purpose = if c.purpose.len() > 100 {
                        format!("{}...", &c.purpose[..100])
                    } else {
                        c.purpose.clone()
                    };
                    println!("                    {}", purpose);
                }
                println!();
            }
            print_meta(&resp.meta);
        }

        Command::Company { uid } => {
            let resp = client.companies().get(&uid).await?;
            let c = &resp.data;
            println!("UID:         {}", c.uid);
            println!("Name:        {}", c.name);
            println!("Legal form:  {}", c.legal_form);
            println!("Status:      {}", c.status);
            println!("Canton:      {}", c.canton);
            println!("Address:     {}", c.address);
            println!("Auditor cat: {}", c.auditor_category);
            if !c.purpose.is_empty() {
                println!("Purpose:     {}", c.purpose);
            }
            if let Some(ref d) = c.created_at {
                println!("Created:     {}", d);
            }
            if let Some(ref d) = c.updated_at {
                println!("Updated:     {}", d);
            }
            print_meta(&resp.meta);
        }

        Command::Count { canton, status } => {
            let params = CompanyCountParams {
                canton: canton.clone(),
                status: status.clone(),
                ..Default::default()
            };
            let resp = client.companies().count(&params).await?;
            let filter = match (&canton, &status) {
                (Some(c), Some(s)) => format!(" (canton={c}, status={s})"),
                (Some(c), None) => format!(" (canton={c})"),
                (None, Some(s)) => format!(" (status={s})"),
                (None, None) => String::new(),
            };
            println!("Company count{filter}: {}", resp.data.count);
            print_meta(&resp.meta);
        }

        Command::Stats => {
            let resp = client.companies().statistics().await?;
            let s = &resp.data;
            println!("Total companies:    {}", s.total_count);
            println!("Enriched:           {}", s.enriched_count);
            if !s.canton_counts.is_empty() {
                println!("\nBy canton:");
                let mut cantons: Vec<_> = s.canton_counts.iter().collect();
                cantons.sort_by(|a, b| b.1.cmp(a.1));
                for (canton, count) in cantons.iter().take(10) {
                    println!("  {:<6} {:>8}", canton, count);
                }
                if cantons.len() > 10 {
                    println!("  ... and {} more cantons", cantons.len() - 10);
                }
            }
            print_meta(&resp.meta);
        }

        Command::Changes { page, page_size } => {
            let params = vynco::ChangeListParams {
                page: Some(page),
                page_size: Some(page_size),
                ..Default::default()
            };
            let resp = client.changes().list(&params).await?;
            println!(
                "Recent changes: page {}/{} ({} total)\n",
                resp.data.page,
                (resp.data.total_count as f64 / resp.data.page_size as f64).ceil() as u64,
                resp.data.total_count,
            );
            for ch in &resp.data.items {
                println!(
                    "  {} {:<18} {:<30} {}",
                    &ch.detected_at[..10.min(ch.detected_at.len())],
                    ch.company_uid,
                    ch.company_name,
                    ch.change_type,
                );
                if let (Some(old), Some(new)) = (&ch.old_value, &ch.new_value) {
                    println!("    {} → {}", old, new);
                }
            }
            print_meta(&resp.meta);
        }
    }

    Ok(())
}

fn print_meta(meta: &vynco::ResponseMeta) {
    println!("\n--- Response metadata ---");
    if let Some(ref id) = meta.request_id {
        println!("Request ID:          {id}");
    }
    if let Some(used) = meta.credits_used {
        println!("Credits used:        {used}");
    }
    if let Some(remaining) = meta.credits_remaining {
        println!("Credits remaining:   {remaining}");
    }
    if let Some(limit) = meta.rate_limit_limit {
        println!("Rate limit:          {limit}/min");
    }
    if let Some(remaining) = meta.rate_limit_remaining {
        println!("Rate limit remaining:{remaining}");
    }
    if let Some(ref source) = meta.data_source {
        println!("Data source:         {source}");
    }
}
