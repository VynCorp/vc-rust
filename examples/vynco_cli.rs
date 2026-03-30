//! # VynCo CLI Example
//!
//! A command-line tool demonstrating the VynCo Rust SDK v2.
//!
//! ## Usage
//!
//! ```bash
//! export VYNCO_API_KEY="vc_live_your_api_key"
//!
//! # Check API health
//! cargo run --example vynco_cli -- health
//!
//! # List companies (with optional filters)
//! cargo run --example vynco_cli -- companies --canton ZH --search "Novartis"
//!
//! # Get a specific company by UID
//! cargo run --example vynco_cli -- company CHE-105.805.649
//!
//! # Count companies
//! cargo run --example vynco_cli -- count
//!
//! # Show events for a company
//! cargo run --example vynco_cli -- events CHE-105.805.649 --limit 10
//!
//! # Screen a company name
//! cargo run --example vynco_cli -- screen "Test Corp"
//!
//! # Show dashboard
//! cargo run --example vynco_cli -- dashboard
//!
//! # List long-tenure auditors
//! cargo run --example vynco_cli -- auditors --min-years 10 --canton ZH
//!
//! # AI risk score for a company
//! cargo run --example vynco_cli -- risk CHE-105.805.649
//! ```

use clap::{Parser, Subcommand};
use vynco::{AuditorTenureParams, Client, CompanyListParams, VyncoError};

#[derive(Parser)]
#[command(name = "vynco")]
#[command(about = "VynCo Swiss Corporate Intelligence CLI — example for the vynco Rust SDK v2")]
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

    /// List companies with optional filters
    Companies {
        /// Filter by Swiss canton (e.g. ZH, BE, GE)
        #[arg(long)]
        canton: Option<String>,

        /// Filter by name (substring match)
        #[arg(long)]
        search: Option<String>,

        /// Page number (default: 1)
        #[arg(long, default_value = "1")]
        page: i64,

        /// Page size (default: 10)
        #[arg(long, default_value = "10")]
        page_size: i64,
    },

    /// Get a specific company by UID (e.g. CHE-105.805.649)
    Company {
        /// Company UID
        uid: String,
    },

    /// Count companies in the database
    Count,

    /// Show events for a company
    Events {
        /// Company UID
        uid: String,

        /// Maximum number of events to return
        #[arg(long)]
        limit: Option<u32>,
    },

    /// Screen a company name against sanctions lists
    Screen {
        /// Company name to screen
        name: String,

        /// Optional company UID for more precise matching
        #[arg(long)]
        uid: Option<String>,
    },

    /// Show admin dashboard
    Dashboard,

    /// List long-tenure auditors
    Auditors {
        /// Minimum tenure in years (e.g. 10)
        #[arg(long)]
        min_years: Option<f64>,

        /// Filter by Swiss canton
        #[arg(long)]
        canton: Option<String>,
    },

    /// AI risk score for a company
    Risk {
        /// Company UID
        uid: String,
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
            println!("Database:    {}", resp.data.database);
            println!("Redis:       {}", resp.data.redis);
            println!("Version:     {}", resp.data.version);
            print_meta(&resp.meta);
        }

        Command::Companies {
            canton,
            search,
            page,
            page_size,
        } => {
            let params = CompanyListParams {
                page: Some(page),
                page_size: Some(page_size),
                canton,
                search,
                ..Default::default()
            };
            let resp = client.companies().list(&params).await?;
            let total_pages = if resp.data.page_size > 0 {
                (resp.data.total as f64 / resp.data.page_size as f64).ceil() as i64
            } else {
                1
            };
            println!(
                "Companies: page {}/{} ({} total)\n",
                resp.data.page, total_pages, resp.data.total,
            );
            for c in &resp.data.items {
                println!(
                    "  {:<18} {:<45} {:<6} {}",
                    c.uid,
                    c.name,
                    c.canton.as_deref().unwrap_or("-"),
                    c.status.as_deref().unwrap_or("-"),
                );
            }
            print_meta(&resp.meta);
        }

        Command::Company { uid } => {
            let resp = client.companies().get(&uid).await?;
            let c = &resp.data;
            println!("UID:         {}", c.uid);
            println!("Name:        {}", c.name);
            println!("Legal form:  {}", c.legal_form.as_deref().unwrap_or("-"));
            println!("Status:      {}", c.status.as_deref().unwrap_or("-"));
            println!("Canton:      {}", c.canton.as_deref().unwrap_or("-"));
            if let Some(ref cap) = c.share_capital {
                println!("Share cap:   {:.2}", cap);
            }
            if let Some(ref cat) = c.auditor_category {
                println!("Auditor cat: {}", cat);
            }
            if let Some(ref d) = c.updated_at {
                println!("Updated:     {}", d);
            }
            print_meta(&resp.meta);
        }

        Command::Count => {
            let resp = client.companies().count().await?;
            println!("Company count: {}", resp.data.count);
            print_meta(&resp.meta);
        }

        Command::Events { uid, limit } => {
            let resp = client.companies().events(&uid, limit).await?;
            println!("Events for {}: {} total\n", uid, resp.data.count);
            for evt in &resp.data.events {
                println!(
                    "  [{:<10}] {:<30} {}",
                    evt.category, evt.ce_type, evt.summary,
                );
            }
            print_meta(&resp.meta);
        }

        Command::Screen { name, uid } => {
            let req = vynco::ScreeningRequest {
                name: name.clone(),
                uid,
                sources: None,
            };
            let resp = client.screening().screen(&req).await?;
            println!("Screening: \"{}\"", name);
            println!("Risk level: {}", resp.data.risk_level);
            println!("Hits:       {}", resp.data.hit_count);
            println!("Sources:    {}", resp.data.sources_checked.join(", "));
            if !resp.data.hits.is_empty() {
                println!("\nMatches:");
                for hit in &resp.data.hits {
                    println!(
                        "  {} (score: {:.2}) — {} [{}]",
                        hit.matched_name,
                        hit.score,
                        hit.source,
                        hit.datasets.join(", "),
                    );
                }
            }
            print_meta(&resp.meta);
        }

        Command::Dashboard => {
            let resp = client.dashboard().get().await?;
            let d = &resp.data.data;
            println!("Total companies: {}", d.total_companies);
            println!("With canton:     {}", d.with_canton);
            println!("With status:     {}", d.with_status);
            println!("With legal form: {}", d.with_legal_form);
            println!("With capital:    {}", d.with_capital);
            println!("Completeness:    {:.1}%", d.completeness_pct);

            let t = &resp.data.auditor_tenures;
            println!("\nAuditor tenures:");
            println!("  Total:          {}", t.total_tenures);
            println!("  Long (7+yr):    {}", t.long_tenures_7plus);
            println!("  Avg years:      {:.1}", t.avg_tenure_years);
            println!("  Max years:      {:.1}", t.max_tenure_years);
            print_meta(&resp.meta);
        }

        Command::Auditors { min_years, canton } => {
            let params = AuditorTenureParams {
                min_years,
                canton,
                page: Some(1),
                page_size: Some(20),
            };
            let resp = client.auditors().tenures(&params).await?;
            let total_pages = if resp.data.page_size > 0 {
                (resp.data.total as f64 / resp.data.page_size as f64).ceil() as i64
            } else {
                1
            };
            println!(
                "Auditor tenures: page {}/{} ({} total)\n",
                resp.data.page, total_pages, resp.data.total,
            );
            for t in &resp.data.items {
                println!(
                    "  {:<18} {:<35} {:<25} {:.1} yr",
                    t.company_uid,
                    t.company_name,
                    t.auditor_name,
                    t.tenure_years.unwrap_or(0.0),
                );
            }
            print_meta(&resp.meta);
        }

        Command::Risk { uid } => {
            let req = vynco::RiskScoreRequest { uid: uid.clone() };
            let resp = client.ai().risk_score(&req).await?;
            let r = &resp.data;
            println!("Risk score for {} ({})", r.uid, r.company_name);
            println!("Overall:     {}/100", r.overall_score);
            println!("Risk level:  {}", r.risk_level);
            if !r.breakdown.is_empty() {
                println!("\nBreakdown:");
                for f in &r.breakdown {
                    println!(
                        "  {:<20} score: {:<4} weight: {:.2}  {}",
                        f.factor, f.score, f.weight, f.description,
                    );
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
