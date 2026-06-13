use crate::read_file;
use mykobo_rs::payment_intent::models::{HealthResponse, ReferenceResponse};
use pretty_assertions::assert_eq;

#[test]
fn test_deserialise_health_response() {
    let content = read_file("tests/payment_intent/fixtures/health_response.json");
    let result = serde_json::from_str::<HealthResponse>(&content);

    assert!(result.is_ok());
    let health = result.unwrap();
    assert_eq!(health.status, "Ok");
    assert_eq!(health.message, "Payment Intent Service is running");
}

#[test]
fn test_deserialise_reference_response() {
    let content = read_file("tests/payment_intent/fixtures/create_reference_response.json");
    let result = serde_json::from_str::<ReferenceResponse>(&content);

    assert!(result.is_ok());
    let ref_resp = result.unwrap();
    assert_eq!(ref_resp.id, "urn:dpref:a1b2c3d4e5f6");
    assert_eq!(ref_resp.profile_id, "urn:usrp:test-user");
    assert_eq!(ref_resp.reference, "MYK-P-ABC12345");
    assert!(ref_resp.is_active);
    assert_eq!(ref_resp.wallet_address, "GABCDEFGHIJKLMNOPQRSTUVWXYZ123456");
    assert_eq!(ref_resp.client_domain, Some("example.com".to_string()));
    assert_eq!(ref_resp.created_at, "2026-06-13 12:00:00");
}

#[test]
fn test_deserialise_reference_response_no_client_domain() {
    let content = r#"{
        "id": "urn:dpref:a1b2c3d4e5f6",
        "profile_id": "urn:usrp:test-user",
        "reference": "MYK-P-ABC12345",
        "is_active": true,
        "wallet_address": "GABCDEFGHIJKLMNOPQRSTUVWXYZ123456",
        "created_at": "2026-06-13 12:00:00"
    }"#;
    let result = serde_json::from_str::<ReferenceResponse>(&content);

    assert!(result.is_ok());
    let ref_resp = result.unwrap();
    assert_eq!(ref_resp.client_domain, None);
}

#[test]
fn test_deserialise_references_list() {
    let content = read_file("tests/payment_intent/fixtures/get_references_by_user_response.json");
    let result = serde_json::from_str::<Vec<ReferenceResponse>>(&content);

    assert!(result.is_ok());
    let refs = result.unwrap();
    assert_eq!(refs.len(), 2);

    let first = &refs[0];
    assert_eq!(first.reference, "MYK-P-ABC12345");
    assert!(first.is_active);
    assert!(first.client_domain.is_none());

    let second = &refs[1];
    assert_eq!(second.reference, "MYK-P-XYZ98765");
    assert!(!second.is_active);
    assert_eq!(second.wallet_address, "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb1");
    assert_eq!(second.client_domain, Some("other.example".to_string()));
}

#[test]
fn test_serialise_create_reference_request() {
    use mykobo_rs::payment_intent::models::CreateReferenceRequest;

    let req = CreateReferenceRequest {
        profile_id: "urn:usrp:test-user".to_string(),
        wallet_address: "GABCDEFGHIJKLMNOPQRSTUVWXYZ123456".to_string(),
        client_domain: Some("example.com".to_string()),
    };

    let json = serde_json::to_string(&req).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["profile_id"], "urn:usrp:test-user");
    assert_eq!(parsed["wallet_address"], "GABCDEFGHIJKLMNOPQRSTUVWXYZ123456");
    assert_eq!(parsed["client_domain"], "example.com");
}

#[test]
fn test_serialise_create_reference_request_no_client_domain() {
    use mykobo_rs::payment_intent::models::CreateReferenceRequest;

    let req = CreateReferenceRequest {
        profile_id: "urn:usrp:test-user".to_string(),
        wallet_address: "GABCDEFGHIJKLMNOPQRSTUVWXYZ123456".to_string(),
        client_domain: None,
    };

    let json = serde_json::to_string(&req).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert!(parsed.get("client_domain").is_none() || parsed["client_domain"].is_null());
}
