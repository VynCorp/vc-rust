use vynco::{
    Client, CompanyListParams, CompanySearchRequest, CreateApiKeyRequest, GenerateDossierRequest,
    VyncoError,
};

// ---------------------------------------------------------------------------
// Configuration
// ---------------------------------------------------------------------------

#[tokio::test]
async fn empty_api_key_returns_config_error() {
    let result = Client::builder("").build();
    assert!(result.is_err());
    match result.unwrap_err() {
        VyncoError::Config(msg) => assert!(msg.contains("empty")),
        other => panic!("expected Config error, got: {other}"),
    }
}

// ---------------------------------------------------------------------------
// Authentication
// ---------------------------------------------------------------------------

#[tokio::test]
async fn authorization_header_is_set() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/credits/balance")
        .match_header("Authorization", "Bearer vc_test_123")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{"balance":1000,"monthlyCredits":500,"usedThisMonth":50,"tier":"starter","overageRate":0.01}"#,
        )
        .create_async()
        .await;

    let client = Client::builder("vc_test_123")
        .base_url(server.url())
        .build()
        .unwrap();

    let resp = client.credits().balance().await.unwrap();
    assert_eq!(resp.data.balance, 1000);
    mock.assert_async().await;
}

// ---------------------------------------------------------------------------
// Error mapping
// ---------------------------------------------------------------------------

#[tokio::test]
async fn not_found_returns_not_found_error() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/companies/CHE-000.000.000")
        .with_status(404)
        .with_header("content-type", "application/json")
        .with_body(r#"{"type":"https://tools.ietf.org/html/rfc7231#section-6.5.4","title":"Not Found","detail":"Company not found","status":404}"#)
        .create_async()
        .await;

    let client = Client::builder("vc_test_key")
        .base_url(server.url())
        .build()
        .unwrap();

    let err = client.companies().get("CHE-000.000.000").await.unwrap_err();
    match err {
        VyncoError::NotFound(body) => {
            assert_eq!(body.detail, "Company not found");
            assert_eq!(body.title, "Not Found");
        }
        other => panic!("expected NotFound, got: {other}"),
    }
    mock.assert_async().await;
}

#[tokio::test]
async fn unauthorized_returns_authentication_error() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/credits/balance")
        .with_status(401)
        .with_header("content-type", "application/json")
        .with_body(r#"{"detail":"Invalid API key","status":401}"#)
        .create_async()
        .await;

    let client = Client::builder("vc_test_bad")
        .base_url(server.url())
        .build()
        .unwrap();

    let err = client.credits().balance().await.unwrap_err();
    match err {
        VyncoError::Authentication(body) => assert_eq!(body.detail, "Invalid API key"),
        other => panic!("expected Authentication, got: {other}"),
    }
    mock.assert_async().await;
}

#[tokio::test]
async fn rate_limit_returns_rate_limit_error() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/credits/balance")
        .with_status(429)
        .with_header("content-type", "application/json")
        .with_body(r#"{"detail":"Rate limit exceeded","status":429}"#)
        .create_async()
        .await;

    let client = Client::builder("vc_test_key")
        .base_url(server.url())
        .max_retries(0)
        .build()
        .unwrap();

    let err = client.credits().balance().await.unwrap_err();
    match err {
        VyncoError::RateLimit(body) => assert_eq!(body.detail, "Rate limit exceeded"),
        other => panic!("expected RateLimit, got: {other}"),
    }
    mock.assert_async().await;
}

#[tokio::test]
async fn server_error_returns_server_error() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/credits/balance")
        .with_status(500)
        .with_header("content-type", "application/json")
        .with_body(r#"{"detail":"Internal server error","status":500}"#)
        .create_async()
        .await;

    let client = Client::builder("vc_test_key")
        .base_url(server.url())
        .max_retries(0)
        .build()
        .unwrap();

    let err = client.credits().balance().await.unwrap_err();
    match err {
        VyncoError::Server(body) => assert_eq!(body.detail, "Internal server error"),
        other => panic!("expected Server, got: {other}"),
    }
    mock.assert_async().await;
}

#[tokio::test]
async fn insufficient_credits_returns_error() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/dossiers/CHE-100.000.000/generate")
        .with_status(402)
        .with_header("content-type", "application/json")
        .with_body(r#"{"detail":"Insufficient credits","status":402}"#)
        .create_async()
        .await;

    let client = Client::builder("vc_test_key")
        .base_url(server.url())
        .build()
        .unwrap();

    let req = GenerateDossierRequest {
        dossier_type: Some("comprehensive".into()),
    };
    let err = client
        .dossiers()
        .generate("CHE-100.000.000", &req)
        .await
        .unwrap_err();
    match err {
        VyncoError::InsufficientCredits(body) => {
            assert_eq!(body.detail, "Insufficient credits")
        }
        other => panic!("expected InsufficientCredits, got: {other}"),
    }
    mock.assert_async().await;
}

#[tokio::test]
async fn conflict_returns_conflict_error() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/teams/me/members")
        .with_status(409)
        .with_header("content-type", "application/json")
        .with_body(r#"{"detail":"Already a member","status":409}"#)
        .create_async()
        .await;

    let client = Client::builder("vc_test_key")
        .base_url(server.url())
        .build()
        .unwrap();

    let req = vynco::InviteMemberRequest {
        email: "test@example.com".into(),
        role: "Member".into(),
        name: None,
        invited_by: None,
    };
    let err = client.teams().invite_member(&req).await.unwrap_err();
    match err {
        VyncoError::Conflict(body) => assert_eq!(body.detail, "Already a member"),
        other => panic!("expected Conflict, got: {other}"),
    }
    mock.assert_async().await;
}

// ---------------------------------------------------------------------------
// Company endpoints
// ---------------------------------------------------------------------------

#[tokio::test]
async fn company_list_parses_paginated_response() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/companies")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("search".into(), "Novartis".into()),
            mockito::Matcher::UrlEncoded("canton".into(), "BS".into()),
        ]))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_header("X-Request-Id", "req-abc-123")
        .with_header("X-Credits-Used", "1")
        .with_header("X-Credits-Remaining", "499")
        .with_header("X-Rate-Limit-Limit", "60")
        .with_header(
            "X-Data-Source",
            "Zefix / Federal Commercial Registry Office (EHRA)",
        )
        .with_body(
            r#"{
                "items": [
                    {
                        "uid": "CHE-100.023.968",
                        "name": "Novartis AG",
                        "canton": "BS",
                        "legalForm": "AG",
                        "status": "Active",
                        "address": "Basel, Switzerland",
                        "purpose": "Pharmaceutical company",
                        "auditorCategory": "State-supervised",
                        "createdAt": "1996-12-20T00:00:00Z",
                        "updatedAt": "2026-01-15T10:30:00Z"
                    }
                ],
                "totalCount": 1,
                "page": 1,
                "pageSize": 25
            }"#,
        )
        .create_async()
        .await;

    let client = Client::builder("vc_test_key")
        .base_url(server.url())
        .build()
        .unwrap();

    let params = CompanyListParams {
        search: Some("Novartis".into()),
        canton: Some("BS".into()),
        ..Default::default()
    };

    let resp = client.companies().list(&params).await.unwrap();

    // Verify body
    assert_eq!(resp.data.total_count, 1);
    assert_eq!(resp.data.items.len(), 1);
    assert_eq!(resp.data.items[0].uid, "CHE-100.023.968");
    assert_eq!(resp.data.items[0].name, "Novartis AG");
    assert_eq!(resp.data.items[0].canton, "BS");
    assert_eq!(resp.data.items[0].legal_form, "AG");
    assert_eq!(resp.data.items[0].address, "Basel, Switzerland");
    assert_eq!(resp.data.items[0].auditor_category, "State-supervised");

    // Verify metadata from headers
    assert_eq!(resp.meta.request_id.as_deref(), Some("req-abc-123"));
    assert_eq!(resp.meta.credits_used, Some(1));
    assert_eq!(resp.meta.credits_remaining, Some(499));
    assert_eq!(resp.meta.rate_limit_limit, Some(60));
    assert!(resp.meta.data_source.as_deref().unwrap().contains("Zefix"));

    mock.assert_async().await;
}

#[tokio::test]
async fn company_get_by_uid() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/companies/CHE-100.023.968")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
                "uid": "CHE-100.023.968",
                "name": "Novartis AG",
                "canton": "BS",
                "legalForm": "AG",
                "status": "Active",
                "address": "Basel, Switzerland",
                "purpose": "Pharmaceutical company",
                "auditorCategory": "State-supervised",
                "createdAt": "1996-12-20T00:00:00Z",
                "updatedAt": "2026-01-15T10:30:00Z"
            }"#,
        )
        .create_async()
        .await;

    let client = Client::builder("vc_test_key")
        .base_url(server.url())
        .build()
        .unwrap();

    let resp = client.companies().get("CHE-100.023.968").await.unwrap();
    assert_eq!(resp.data.name, "Novartis AG");
    assert_eq!(resp.data.status, "Active");
    assert_eq!(resp.data.address, "Basel, Switzerland");
    mock.assert_async().await;
}

#[tokio::test]
async fn company_search_fts() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/companies/search")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"[
                {
                    "uid": "CHE-100.023.968",
                    "name": "Novartis AG",
                    "canton": "BS",
                    "legalForm": "AG",
                    "status": "Active",
                    "address": "",
                    "purpose": "",
                    "auditorCategory": "",
                    "createdAt": null,
                    "updatedAt": null
                }
            ]"#,
        )
        .create_async()
        .await;

    let client = Client::builder("vc_test_key")
        .base_url(server.url())
        .build()
        .unwrap();

    let req = CompanySearchRequest {
        query: "Novartis".into(),
        limit: Some(10),
    };
    let resp = client.companies().search(&req).await.unwrap();
    assert_eq!(resp.data.len(), 1);
    assert_eq!(resp.data[0].uid, "CHE-100.023.968");
    mock.assert_async().await;
}

// ---------------------------------------------------------------------------
// Changes
// ---------------------------------------------------------------------------

#[tokio::test]
async fn changes_list_parses_response() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/changes")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
                "items": [
                    {
                        "id": "change-001",
                        "companyUid": "CHE-100.023.968",
                        "companyName": "Novartis AG",
                        "changeType": "NameChange",
                        "fieldName": "name",
                        "oldValue": "Old Name",
                        "newValue": "New Name",
                        "sogcId": "HR01-1234",
                        "isReviewed": false,
                        "isFlagged": true,
                        "detectedAt": "2026-03-15T10:00:00Z"
                    }
                ],
                "totalCount": 1,
                "page": 1,
                "pageSize": 25
            }"#,
        )
        .create_async()
        .await;

    let client = Client::builder("vc_test_key")
        .base_url(server.url())
        .build()
        .unwrap();

    let params = vynco::ChangeListParams::default();
    let resp = client.changes().list(&params).await.unwrap();
    assert_eq!(resp.data.total_count, 1);
    assert_eq!(resp.data.items[0].company_name, "Novartis AG");
    assert_eq!(resp.data.items[0].sogc_id.as_deref(), Some("HR01-1234"));
    assert!(resp.data.items[0].is_flagged);
    assert!(!resp.data.items[0].is_reviewed);
    mock.assert_async().await;
}

// ---------------------------------------------------------------------------
// Credit balance
// ---------------------------------------------------------------------------

#[tokio::test]
async fn credit_balance_parses_response() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/credits/balance")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
                "balance": 4500,
                "monthlyCredits": 5000,
                "usedThisMonth": 500,
                "tier": "professional",
                "overageRate": 0.005
            }"#,
        )
        .create_async()
        .await;

    let client = Client::builder("vc_test_key")
        .base_url(server.url())
        .build()
        .unwrap();

    let resp = client.credits().balance().await.unwrap();
    assert_eq!(resp.data.balance, 4500);
    assert_eq!(resp.data.monthly_credits, 5000);
    assert_eq!(resp.data.used_this_month, 500);
    assert_eq!(resp.data.tier, "professional");
    mock.assert_async().await;
}

// ---------------------------------------------------------------------------
// API key creation
// ---------------------------------------------------------------------------

#[tokio::test]
async fn api_key_creation_returns_secret() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/api-keys")
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
                "id": "key_abc123",
                "name": "CI Pipeline",
                "prefix": "vc_live_",
                "isTestKey": false,
                "createdAt": "2026-03-17T12:00:00Z",
                "lastUsedAt": null,
                "key": "vc_live_abcdefghijklmnop1234567890ABCDEF"
            }"#,
        )
        .create_async()
        .await;

    let client = Client::builder("vc_test_key")
        .base_url(server.url())
        .build()
        .unwrap();

    let req = CreateApiKeyRequest {
        name: "CI Pipeline".into(),
        is_test_key: Some(false),
    };

    let resp = client.api_keys().create(&req).await.unwrap();
    assert_eq!(resp.data.id, "key_abc123");
    assert_eq!(resp.data.name, "CI Pipeline");
    assert!(resp.data.key.starts_with("vc_live_"));
    mock.assert_async().await;
}

// ---------------------------------------------------------------------------
// Health check
// ---------------------------------------------------------------------------

#[tokio::test]
async fn health_check_parses_response() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/health")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
                "status": "Healthy",
                "uptime": "3d 14h 22m",
                "checks": [
                    {
                        "name": "database",
                        "status": "Healthy",
                        "durationMs": 12,
                        "message": ""
                    }
                ]
            }"#,
        )
        .create_async()
        .await;

    let client = Client::builder("vc_test_key")
        .base_url(server.url())
        .build()
        .unwrap();

    let resp = client.health().check().await.unwrap();
    assert_eq!(resp.data.status, "Healthy");
    assert_eq!(resp.data.checks.len(), 1);
    assert_eq!(resp.data.checks[0].name, "database");
    mock.assert_async().await;
}

// ---------------------------------------------------------------------------
// Response metadata from headers (including new rate limit fields)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn response_meta_parsed_from_headers() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/teams/me")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_header("X-Request-Id", "req-xyz-789")
        .with_header("X-Credits-Used", "0")
        .with_header("X-Credits-Remaining", "10000")
        .with_header("X-Rate-Limit-Limit", "300")
        .with_header("X-RateLimit-Remaining", "295")
        .with_header("X-RateLimit-Reset", "1742310000")
        .with_header(
            "X-Data-Source",
            "Zefix / Federal Commercial Registry Office (EHRA)",
        )
        .with_body(
            r#"{
                "id": "team_001",
                "name": "Acme Corp",
                "slug": "acme-corp",
                "tier": "enterprise",
                "creditBalance": 10000,
                "monthlyCredits": 10000,
                "overageRate": 0.002,
                "createdAt": "2025-06-01T00:00:00Z",
                "updatedAt": "2026-03-01T00:00:00Z"
            }"#,
        )
        .create_async()
        .await;

    let client = Client::builder("vc_test_key")
        .base_url(server.url())
        .build()
        .unwrap();

    let resp = client.teams().me().await.unwrap();

    assert_eq!(resp.meta.request_id.as_deref(), Some("req-xyz-789"));
    assert_eq!(resp.meta.credits_used, Some(0));
    assert_eq!(resp.meta.credits_remaining, Some(10000));
    assert_eq!(resp.meta.rate_limit_limit, Some(300));
    assert_eq!(resp.meta.rate_limit_remaining, Some(295));
    assert_eq!(resp.meta.rate_limit_reset, Some(1742310000));
    assert!(resp.meta.data_source.as_deref().unwrap().contains("Zefix"));
    assert_eq!(resp.data.name, "Acme Corp");
    assert_eq!(resp.data.tier, "enterprise");
    mock.assert_async().await;
}
