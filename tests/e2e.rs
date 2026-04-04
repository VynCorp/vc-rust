//! End-to-end tests against a live VynCo API.
//!
//! These tests call every SDK method against the real API.
//! They are `#[ignore]`d by default so `cargo test` skips them.
//!
//! ## Running
//!
//! ```bash
//! # Run all e2e tests (requires a funded API key):
//! VYNCO_API_KEY=vc_live_... cargo test --test e2e -- --ignored
//!
//! # Run only the free-tier (read-only) tests:
//! VYNCO_API_KEY=vc_live_... cargo test --test e2e free_ -- --ignored
//!
//! # Run a single test:
//! VYNCO_API_KEY=vc_live_... cargo test --test e2e free_companies_list -- --ignored
//! ```
//!
//! ## Tier gates
//!
//! Tests are prefixed by the minimum tier required:
//!
//! - `free_*`         — public / free-tier endpoints (no credits consumed)
//! - `starter_*`      — starter-tier endpoints (screening, watchlists, webhooks, notes, tags)
//! - `professional_*` — professional-tier endpoints (AI, dossiers, exports, analytics advanced)
//!
//! Write operations (POST/PUT/DELETE) clean up after themselves.
//!
//! ## Well-known UIDs
//!
//! Tests use Nestlé SA (`CHE-105.805.080`) as the default company because it is
//! a large, stable, public entity with rich data across all dimensions.

use std::time::Duration;
use vynco::*;

/// Well-known UID: Nestlé SA (VD, active, AG, enriched).
const NESTLE: &str = "CHE-105.805.080";

/// Build a client from the VYNCO_API_KEY environment variable.
/// Returns `None` if the key is not set, allowing tests to skip gracefully.
fn live_client() -> Option<Client> {
    let key = std::env::var("VYNCO_API_KEY").ok()?;
    if key.is_empty() {
        return None;
    }
    Some(
        Client::builder(&key)
            .timeout(Duration::from_secs(30))
            .max_retries(1)
            .build()
            .expect("failed to build client"),
    )
}

/// Helper: unwrap the client or skip the test.
macro_rules! client {
    () => {
        match live_client() {
            Some(c) => c,
            None => {
                eprintln!("VYNCO_API_KEY not set — skipping");
                return;
            }
        }
    };
}

// ===========================================================================
// Free tier — public / read-only endpoints
// ===========================================================================

#[tokio::test]
#[ignore]
async fn free_health_check() {
    let c = client!();
    let resp = c.health().check().await.unwrap();
    assert_eq!(resp.data.status, "healthy");
    assert!(!resp.data.version.is_empty());
}

// -- Companies ---------------------------------------------------------------

#[tokio::test]
#[ignore]
async fn free_companies_list() {
    let c = client!();
    let params = CompanyListParams {
        canton: Some("ZH".into()),
        page: Some(1),
        page_size: Some(5),
        ..Default::default()
    };
    let resp = c.companies().list(&params).await.unwrap();
    assert!(resp.data.total > 0);
    assert!(!resp.data.items.is_empty());
    let first = &resp.data.items[0];
    assert!(!first.uid.is_empty());
    assert!(!first.name.is_empty());
}

#[tokio::test]
#[ignore]
async fn free_companies_list_filters() {
    let c = client!();
    let params = CompanyListParams {
        legal_form: Some("AG".into()),
        capital_min: Some(1_000_000.0),
        sort_by: Some("shareCapital".into()),
        sort_desc: Some(true),
        page: Some(1),
        page_size: Some(3),
        ..Default::default()
    };
    let resp = c.companies().list(&params).await.unwrap();
    assert!(resp.data.total > 0);
    for co in &resp.data.items {
        assert_eq!(co.legal_form.as_deref(), Some("AG"));
        assert!(co.share_capital.unwrap_or(0.0) >= 1_000_000.0);
    }
}

#[tokio::test]
#[ignore]
async fn free_companies_get() {
    let c = client!();
    let resp = c.companies().get(NESTLE).await.unwrap();
    assert_eq!(resp.data.uid, NESTLE);
    assert!(resp.data.name.contains("Nestl"));
    assert!(resp.data.canton.is_some());
    assert!(resp.data.share_capital.is_some());
    // Verify enriched fields are present
    assert!(resp.data.legal_form.is_some());
    assert!(resp.data.founding_date.is_some() || resp.data.registration_date.is_some());
}

#[tokio::test]
#[ignore]
async fn free_companies_get_full() {
    let c = client!();
    let resp = c.companies().get_full(NESTLE).await.unwrap();
    assert_eq!(resp.data.company.uid, NESTLE);
    // Nestlé should have persons and relationships
    assert!(!resp.data.persons.is_empty(), "expected persons for Nestlé");
}

#[tokio::test]
#[ignore]
async fn free_companies_count() {
    let c = client!();
    let resp = c.companies().count().await.unwrap();
    assert!(resp.data.count > 400_000, "expected 400k+ companies");
}

#[tokio::test]
#[ignore]
async fn free_companies_events() {
    let c = client!();
    let resp = c.companies().events(NESTLE, Some(5)).await.unwrap();
    // May be empty if no recent events, but should not error
    assert!(resp.data.count >= 0);
}

#[tokio::test]
#[ignore]
async fn free_companies_statistics() {
    let c = client!();
    let resp = c.companies().statistics().await.unwrap();
    assert!(resp.data.total > 0);
    assert!(!resp.data.by_canton.is_empty());
    assert!(!resp.data.by_legal_form.is_empty());
}

#[tokio::test]
#[ignore]
async fn free_companies_compare() {
    let c = client!();
    let req = CompareRequest {
        uids: vec![NESTLE.into(), "CHE-109.340.740".into()],
    };
    let resp = c.companies().compare(&req).await.unwrap();
    assert_eq!(resp.data.uids.len(), 2);
    assert!(!resp.data.dimensions.is_empty());
}

#[tokio::test]
#[ignore]
async fn free_companies_news() {
    let c = client!();
    let resp = c.companies().news(NESTLE).await.unwrap();
    // May be empty, but should deserialize
    let _ = resp.data;
}

#[tokio::test]
#[ignore]
async fn free_companies_reports() {
    let c = client!();
    let resp = c.companies().reports(NESTLE).await.unwrap();
    let _ = resp.data;
}

#[tokio::test]
#[ignore]
async fn free_companies_relationships() {
    let c = client!();
    let resp = c.companies().relationships(NESTLE).await.unwrap();
    let _ = resp.data;
}

#[tokio::test]
#[ignore]
async fn free_companies_hierarchy() {
    let c = client!();
    let resp = c.companies().hierarchy(NESTLE).await.unwrap();
    let _ = resp.data;
}

#[tokio::test]
#[ignore]
async fn free_companies_structure() {
    let c = client!();
    let resp = c.companies().structure(NESTLE).await.unwrap();
    let _ = resp.data;
}

#[tokio::test]
#[ignore]
async fn free_companies_classification() {
    let c = client!();
    let resp = c.companies().classification(NESTLE).await.unwrap();
    assert_eq!(resp.data.company_uid, NESTLE);
    assert!(!resp.data.method.is_empty());
}

#[tokio::test]
#[ignore]
async fn free_companies_fingerprint() {
    let c = client!();
    let resp = c.companies().fingerprint(NESTLE).await.unwrap();
    assert_eq!(resp.data.company_uid, NESTLE);
    assert!(!resp.data.canton.is_empty());
    assert!(resp.data.board_size > 0);
}

#[tokio::test]
#[ignore]
async fn free_companies_acquisitions() {
    let c = client!();
    let resp = c.companies().acquisitions(NESTLE).await.unwrap();
    let _ = resp.data;
}

#[tokio::test]
#[ignore]
async fn free_companies_nearby() {
    let c = client!();
    // Vevey coordinates (Nestlé HQ)
    let params = NearbyParams {
        lat: 46.464,
        lng: 6.841,
        radius_km: Some(2.0),
        limit: Some(5),
    };
    let resp = c.companies().nearby(&params).await.unwrap();
    assert!(!resp.data.is_empty());
    for co in &resp.data {
        assert!(co.distance <= 2.0);
    }
}

#[tokio::test]
#[ignore]
async fn free_companies_not_found() {
    let c = client!();
    let err = c.companies().get("CHE-000.000.000").await.unwrap_err();
    assert!(matches!(err, VyncoError::NotFound(_)));
}

// -- Auditors ----------------------------------------------------------------

#[tokio::test]
#[ignore]
async fn free_auditors_history() {
    let c = client!();
    let resp = c.auditors().history(NESTLE).await.unwrap();
    assert_eq!(resp.data.company_uid, NESTLE);
}

#[tokio::test]
#[ignore]
async fn free_auditors_tenures() {
    let c = client!();
    let params = AuditorTenureParams {
        min_years: Some(10.0),
        canton: Some("ZH".into()),
        page: Some(1),
        page_size: Some(5),
    };
    let resp = c.auditors().tenures(&params).await.unwrap();
    assert!(resp.data.total > 0);
    for t in &resp.data.items {
        assert!(t.tenure_years.unwrap_or(0.0) >= 10.0);
    }
}

// -- Dashboard ---------------------------------------------------------------

#[tokio::test]
#[ignore]
async fn free_dashboard() {
    let c = client!();
    let resp = c.dashboard().get().await.unwrap();
    assert!(resp.data.data.total_companies > 0);
    assert!(!resp.data.generated_at.is_empty());
}

// -- Changes -----------------------------------------------------------------

#[tokio::test]
#[ignore]
async fn free_changes_list() {
    let c = client!();
    let params = ChangeListParams {
        page: Some(1),
        page_size: Some(5),
        ..Default::default()
    };
    let resp = c.changes().list(&params).await.unwrap();
    assert!(resp.data.total > 0);
}

#[tokio::test]
#[ignore]
async fn free_changes_by_company() {
    let c = client!();
    let resp = c.changes().by_company(NESTLE).await.unwrap();
    let _ = resp.data;
}

#[tokio::test]
#[ignore]
async fn free_changes_statistics() {
    let c = client!();
    let resp = c.changes().statistics().await.unwrap();
    assert!(resp.data.total_changes > 0);
}

// -- Persons -----------------------------------------------------------------

#[tokio::test]
#[ignore]
async fn free_persons_board_members() {
    let c = client!();
    let resp = c.persons().board_members(NESTLE).await.unwrap();
    assert!(!resp.data.is_empty(), "Nestlé should have board members");
    for m in &resp.data {
        assert!(!m.role.is_empty());
    }
}

#[tokio::test]
#[ignore]
async fn free_persons_search() {
    let c = client!();
    let params = PersonSearchParams {
        q: Some("Mueller".into()),
        page: Some(1),
        page_size: Some(5),
    };
    let resp = c.persons().search(&params).await.unwrap();
    assert!(resp.data.total > 0);
    assert!(!resp.data.items.is_empty());
}

#[tokio::test]
#[ignore]
async fn free_persons_get() {
    let c = client!();
    // First find a person via search
    let params = PersonSearchParams {
        q: Some("Mueller".into()),
        page: Some(1),
        page_size: Some(1),
    };
    let search = c.persons().search(&params).await.unwrap();
    if search.data.items.is_empty() {
        eprintln!("no persons found — skipping detail test");
        return;
    }
    let id = &search.data.items[0].id;
    let resp = c.persons().get(id).await.unwrap();
    assert_eq!(&resp.data.id, id);
    assert!(!resp.data.full_name.is_empty());
}

// -- Analytics ---------------------------------------------------------------

#[tokio::test]
#[ignore]
async fn free_analytics_cantons() {
    let c = client!();
    let resp = c.analytics().cantons().await.unwrap();
    assert!(!resp.data.is_empty());
    // ZH should be present
    assert!(resp.data.iter().any(|d| d.canton == "ZH"));
}

#[tokio::test]
#[ignore]
async fn free_analytics_auditors() {
    let c = client!();
    let resp = c.analytics().auditors().await.unwrap();
    assert!(!resp.data.is_empty());
}

#[tokio::test]
#[ignore]
async fn free_analytics_cohorts() {
    let c = client!();
    let params = CohortParams {
        group_by: Some("canton".into()),
        metric: Some("count".into()),
    };
    let resp = c.analytics().cohorts(&params).await.unwrap();
    assert!(!resp.data.cohorts.is_empty());
}

#[tokio::test]
#[ignore]
async fn free_analytics_candidates() {
    let c = client!();
    let params = CandidateParams {
        canton: Some("ZH".into()),
        page: Some(1),
        page_size: Some(5),
        ..Default::default()
    };
    let resp = c.analytics().candidates(&params).await.unwrap();
    assert!(resp.data.total > 0);
}

// -- Graph -------------------------------------------------------------------

#[tokio::test]
#[ignore]
async fn free_graph_get() {
    let c = client!();
    let resp = c.graph().get(NESTLE).await.unwrap();
    assert!(!resp.data.nodes.is_empty());
    assert!(!resp.data.links.is_empty());
}

#[tokio::test]
#[ignore]
async fn free_graph_export() {
    let c = client!();
    let file = c.graph().export(NESTLE, "graphml").await.unwrap();
    assert!(!file.bytes.is_empty());
}

// -- Credits -----------------------------------------------------------------

#[tokio::test]
#[ignore]
async fn free_credits_balance() {
    let c = client!();
    let resp = c.credits().balance().await.unwrap();
    assert!(!resp.data.tier.is_empty());
}

#[tokio::test]
#[ignore]
async fn free_credits_usage() {
    let c = client!();
    let resp = c.credits().usage(None).await.unwrap();
    let _ = resp.data.total;
}

#[tokio::test]
#[ignore]
async fn free_credits_history() {
    let c = client!();
    let resp = c.credits().history(Some(5), Some(0)).await.unwrap();
    let _ = resp.data;
}

// -- Teams -------------------------------------------------------------------

#[tokio::test]
#[ignore]
async fn free_teams_me() {
    let c = client!();
    let resp = c.teams().me().await.unwrap();
    assert!(!resp.data.name.is_empty());
}

#[tokio::test]
#[ignore]
async fn free_teams_members() {
    let c = client!();
    let resp = c.teams().members().await.unwrap();
    // At least the current user
    assert!(!resp.data.is_empty());
}

#[tokio::test]
#[ignore]
async fn free_teams_billing_summary() {
    let c = client!();
    let resp = c.teams().billing_summary().await.unwrap();
    assert!(!resp.data.tier.is_empty());
}

// -- API Keys ----------------------------------------------------------------

#[tokio::test]
#[ignore]
async fn free_api_keys_list() {
    let c = client!();
    let resp = c.api_keys().list().await.unwrap();
    // Should have at least the key we're using
    assert!(!resp.data.is_empty());
}

// -- Response metadata -------------------------------------------------------

#[tokio::test]
#[ignore]
async fn free_response_metadata() {
    let c = client!();
    let resp = c.companies().count().await.unwrap();
    assert!(resp.meta.request_id.is_some(), "expected X-Request-Id");
}

// ===========================================================================
// Starter tier — screening, watchlists, webhooks, notes, tags
// ===========================================================================

#[tokio::test]
#[ignore]
async fn starter_screening() {
    let c = client!();
    let req = ScreeningRequest {
        name: "Nestlé SA".into(),
        uid: Some(NESTLE.into()),
        sources: None,
    };
    let resp = c.screening().screen(&req).await.unwrap();
    assert!(!resp.data.risk_level.is_empty());
    assert!(!resp.data.sources_checked.is_empty());
}

// -- Watchlist CRUD ----------------------------------------------------------

#[tokio::test]
#[ignore]
async fn starter_watchlists_crud() {
    let c = client!();

    // Create
    let wl = c
        .watchlists()
        .create(&CreateWatchlistRequest {
            name: "e2e-test-watchlist".into(),
            description: Some("automated test".into()),
        })
        .await
        .unwrap();
    let wl_id = wl.data.id.clone();
    assert_eq!(wl.data.name, "e2e-test-watchlist");

    // List
    let list = c.watchlists().list().await.unwrap();
    assert!(list.data.iter().any(|w| w.id == wl_id));

    // Add companies
    let add = c
        .watchlists()
        .add_companies(
            &wl_id,
            &AddCompaniesRequest {
                uids: vec![NESTLE.into()],
            },
        )
        .await
        .unwrap();
    assert!(add.data.added >= 1);

    // List companies
    let cos = c.watchlists().companies(&wl_id).await.unwrap();
    assert!(cos.data.uids.contains(&NESTLE.to_string()));

    // Events
    let _events = c.watchlists().events(&wl_id, Some(5)).await.unwrap();

    // Remove company
    c.watchlists().remove_company(&wl_id, NESTLE).await.unwrap();

    // Delete watchlist
    c.watchlists().delete(&wl_id).await.unwrap();
}

// -- Webhook CRUD ------------------------------------------------------------

#[tokio::test]
#[ignore]
async fn starter_webhooks_crud() {
    let c = client!();

    // Create
    let created = c
        .webhooks()
        .create(&CreateWebhookRequest {
            url: "https://httpbin.org/post".into(),
            description: Some("e2e test webhook".into()),
            event_filters: None,
            company_filters: None,
        })
        .await
        .unwrap();
    let wh_id = created.data.webhook.id.clone();
    assert!(!created.data.signing_secret.is_empty());

    // List
    let list = c.webhooks().list().await.unwrap();
    assert!(list.data.iter().any(|w| w.id == wh_id));

    // Update
    let updated = c
        .webhooks()
        .update(
            &wh_id,
            &UpdateWebhookRequest {
                description: Some("updated description".into()),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(updated.data.description, "updated description");

    // Deliveries
    let _deliveries = c.webhooks().deliveries(&wh_id, Some(5)).await.unwrap();

    // Test delivery
    let test_resp = c.webhooks().test(&wh_id).await.unwrap();
    let _ = test_resp.data.success;

    // Delete
    c.webhooks().delete(&wh_id).await.unwrap();
}

// -- Notes CRUD --------------------------------------------------------------

#[tokio::test]
#[ignore]
async fn starter_notes_crud() {
    let c = client!();

    // Create
    let note = c
        .companies()
        .create_note(
            NESTLE,
            &CreateNoteRequest {
                content: "e2e test note".into(),
                note_type: Some("note".into()),
                rating: Some(3),
                is_private: Some(true),
            },
        )
        .await
        .unwrap();
    let note_id = note.data.id.clone();
    assert_eq!(note.data.content, "e2e test note");
    assert!(note.data.is_private);

    // List
    let notes = c.companies().notes(NESTLE).await.unwrap();
    assert!(notes.data.iter().any(|n| n.id == note_id));

    // Update
    let updated = c
        .companies()
        .update_note(
            NESTLE,
            &note_id,
            &UpdateNoteRequest {
                content: Some("updated e2e note".into()),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(updated.data.content, "updated e2e note");

    // Delete
    c.companies().delete_note(NESTLE, &note_id).await.unwrap();
}

// -- Tags CRUD ---------------------------------------------------------------

#[tokio::test]
#[ignore]
async fn starter_tags_crud() {
    let c = client!();

    // Create
    let tag = c
        .companies()
        .create_tag(
            NESTLE,
            &CreateTagRequest {
                tag_name: "e2e-test-tag".into(),
                color: Some("#00FF00".into()),
            },
        )
        .await
        .unwrap();
    let tag_id = tag.data.id.clone();
    assert_eq!(tag.data.tag_name, "e2e-test-tag");

    // List company tags
    let tags = c.companies().tags(NESTLE).await.unwrap();
    assert!(tags.data.iter().any(|t| t.id == tag_id));

    // List all user tags
    let all = c.companies().all_tags().await.unwrap();
    assert!(all.data.iter().any(|t| t.tag_name == "e2e-test-tag"));

    // Delete
    c.companies().delete_tag(NESTLE, &tag_id).await.unwrap();
}

// -- Starter analytics -------------------------------------------------------

#[tokio::test]
#[ignore]
async fn starter_analytics_cluster() {
    let c = client!();
    let req = ClusterRequest {
        algorithm: "kmeans".into(),
        k: Some(3),
    };
    let resp = c.analytics().cluster(&req).await.unwrap();
    assert!(!resp.data.clusters.is_empty());
}

#[tokio::test]
#[ignore]
async fn starter_analytics_anomalies() {
    let c = client!();
    let req = AnomalyRequest {
        algorithm: "zscore".into(),
        threshold: Some(2.0),
    };
    let resp = c.analytics().anomalies(&req).await.unwrap();
    assert!(resp.data.total_scanned > 0);
}

#[tokio::test]
#[ignore]
async fn starter_analytics_rfm_segments() {
    let c = client!();
    let resp = c.analytics().rfm_segments().await.unwrap();
    assert!(!resp.data.segments.is_empty());
}

// ===========================================================================
// Professional tier — AI, dossiers, exports, network analysis
// (these consume credits)
// ===========================================================================

#[tokio::test]
#[ignore]
async fn professional_ai_dossier() {
    let c = client!();
    let req = DossierRequest {
        uid: NESTLE.into(),
        depth: Some("summary".into()),
    };
    let resp = c.ai().dossier(&req).await.unwrap();
    assert_eq!(resp.data.uid, NESTLE);
    assert!(!resp.data.dossier.is_empty());
}

#[tokio::test]
#[ignore]
async fn professional_ai_search() {
    let c = client!();
    let req = AiSearchRequest {
        query: "large food companies in Vaud".into(),
    };
    let resp = c.ai().search(&req).await.unwrap();
    assert!(!resp.data.explanation.is_empty());
}

#[tokio::test]
#[ignore]
async fn professional_ai_risk_score() {
    let c = client!();
    let req = RiskScoreRequest { uid: NESTLE.into() };
    let resp = c.ai().risk_score(&req).await.unwrap();
    assert_eq!(resp.data.uid, NESTLE);
    assert!(!resp.data.risk_level.is_empty());
    assert!(!resp.data.breakdown.is_empty());
}

// -- Managed dossiers --------------------------------------------------------

#[tokio::test]
#[ignore]
async fn professional_dossiers_crud() {
    let c = client!();

    // Create
    let dossier = c
        .dossiers()
        .create(&CreateDossierRequest {
            uid: NESTLE.into(),
            level: Some("summary".into()),
        })
        .await
        .unwrap();
    let dos_id = dossier.data.id.clone();
    assert_eq!(dossier.data.company_uid, NESTLE);

    // Get
    let fetched = c.dossiers().get(&dos_id).await.unwrap();
    assert_eq!(fetched.data.id, dos_id);

    // List
    let list = c.dossiers().list().await.unwrap();
    assert!(list.data.iter().any(|d| d.id == dos_id));

    // Delete
    c.dossiers().delete(&dos_id).await.unwrap();
}

#[tokio::test]
#[ignore]
async fn professional_dossiers_generate() {
    let c = client!();
    let resp = c.dossiers().generate(NESTLE).await.unwrap();
    assert_eq!(resp.data.company_uid, NESTLE);
    assert!(!resp.data.content.is_empty());

    // Clean up
    c.dossiers().delete(&resp.data.id).await.unwrap();
}

// -- Exports -----------------------------------------------------------------

#[tokio::test]
#[ignore]
async fn professional_exports_lifecycle() {
    let c = client!();

    // Create
    let job = c
        .exports()
        .create(&CreateExportRequest {
            format: Some("csv".into()),
            canton: Some("AI".into()), // small canton for speed
            max_rows: Some(10),
            ..Default::default()
        })
        .await
        .unwrap();
    assert!(!job.data.id.is_empty());

    // Poll until done (max ~30s)
    let export_id = job.data.id.clone();
    for _ in 0..10 {
        tokio::time::sleep(Duration::from_secs(3)).await;
        let status = c.exports().get(&export_id).await.unwrap();
        if status.data.job.status == "completed" || status.data.job.status == "ready" {
            break;
        }
        if status.data.job.status == "failed" {
            panic!("export failed: {:?}", status.data.job.error_message);
        }
    }

    // Download
    let file = c.exports().download(&export_id).await.unwrap();
    assert!(!file.bytes.is_empty());
}

#[tokio::test]
#[ignore]
async fn professional_companies_export_excel() {
    let c = client!();
    let req = ExcelExportRequest {
        uids: Some(vec![NESTLE.into()]),
        ..Default::default()
    };
    let file = c.companies().export_excel(&req).await.unwrap();
    assert!(!file.bytes.is_empty());
    assert!(file.content_type.contains("csv") || file.content_type.contains("excel"));
}

// -- Network analysis --------------------------------------------------------

#[tokio::test]
#[ignore]
async fn professional_graph_analyze() {
    let c = client!();
    let req = NetworkAnalysisRequest {
        uids: vec![NESTLE.into(), "CHE-109.340.740".into()],
        overlay: "persons".into(),
    };
    let resp = c.graph().analyze(&req).await.unwrap();
    assert!(!resp.data.nodes.is_empty());
}

// ===========================================================================
// API key CRUD (careful — creates/revokes real keys)
// ===========================================================================

#[tokio::test]
#[ignore]
async fn starter_api_keys_crud() {
    let c = client!();

    // Create
    let created = c
        .api_keys()
        .create(&CreateApiKeyRequest {
            name: Some("e2e-test-key".into()),
            environment: Some("test".into()),
            scopes: None,
        })
        .await
        .unwrap();
    let key_id = created.data.id.clone();
    assert!(!created.data.key.is_empty());

    // List
    let list = c.api_keys().list().await.unwrap();
    assert!(list.data.iter().any(|k| k.id == key_id));

    // Revoke
    c.api_keys().revoke(&key_id).await.unwrap();
}
