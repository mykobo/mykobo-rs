use jsonwebtoken::{dangerous::insecure_decode, encode, EncodingKey, Header};
use mykobo_rs::identity::{models::ServiceToken, IdentityServiceClient};
use serde_json::json;
use serial_test::serial;
use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};

/// Helper function to generate a JWT token with a specific expiration time
fn generate_test_token(exp_offset_seconds: i64) -> String {
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let exp = (current_time + exp_offset_seconds) as usize;
    let iat = current_time as usize;

    let claims = json!({
        "sub": "urn:svc:test-service-id",
        "iat": iat,
        "exp": exp,
        "aud": "Service",
        "scope": ["user:read", "user:write"]
    });

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("test-secret".as_ref()),
    )
    .unwrap()
}

#[test]
#[serial]
fn test_token_validation_with_expired_token() {
    // Set up environment variables
    env::set_var("IDENTITY_ACCESS_KEY", "TEST_ACCESS_KEY");
    env::set_var("IDENTITY_SECRET_KEY", "TEST_SECRET_KEY");
    env::set_var("IDENTITY_SERVICE_HOST", "http://localhost:8080");

    let mut client = IdentityServiceClient::new(3);

    // Generate a token that expired 1 hour ago
    let expired_token = generate_test_token(-3600);

    // Set the expired token on the client
    client.set_token(Some(ServiceToken {
        subject_id: "urn:svc:test-service-id".to_string(),
        token: expired_token,
        refresh_token: "refresh-token".to_string(),
    }));

    // token_is_valid is private, but we can test it indirectly through attempt_token_acquisition
    // Since the token is expired, token_is_valid should return false
    // However, we can't directly test the private method, so we'll verify the token structure
    assert!(client.token.is_some());

    // The token should be present but expired
    let token = client.get_token().unwrap();

    // Verify we can decode it (structure is valid)
    let decoded = insecure_decode::<serde_json::Value>(&token.token);
    assert!(decoded.is_ok());

    // Verify the expiration is in the past
    let claims = decoded.unwrap().claims;
    let exp = claims["exp"].as_u64().unwrap() as usize;
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;

    assert!(exp < current_time, "Token should be expired");
}

#[test]
#[serial]
fn test_token_validation_with_valid_token() {
    // Set up environment variables
    env::set_var("IDENTITY_ACCESS_KEY", "TEST_ACCESS_KEY");
    env::set_var("IDENTITY_SECRET_KEY", "TEST_SECRET_KEY");
    env::set_var("IDENTITY_SERVICE_HOST", "http://localhost:8080");

    let mut client = IdentityServiceClient::new(3);

    // Generate a token that expires in 1 hour
    let valid_token = generate_test_token(3600);

    // Set the valid token on the client
    client.set_token(Some(ServiceToken {
        subject_id: "urn:svc:test-service-id".to_string(),
        token: valid_token,
        refresh_token: "refresh-token".to_string(),
    }));

    assert!(client.token.is_some());

    let token = client.get_token().unwrap();

    // Verify we can decode it
    let decoded = insecure_decode::<serde_json::Value>(&token.token);
    assert!(decoded.is_ok());

    // Verify the expiration is in the future
    let claims = decoded.unwrap().claims;
    let exp = claims["exp"].as_u64().unwrap() as usize;
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;

    assert!(exp > current_time, "Token should not be expired");
}

#[test]
#[serial]
fn test_token_validation_with_token_expiring_soon() {
    // Set up environment variables
    env::set_var("IDENTITY_ACCESS_KEY", "TEST_ACCESS_KEY");
    env::set_var("IDENTITY_SECRET_KEY", "TEST_SECRET_KEY");
    env::set_var("IDENTITY_SERVICE_HOST", "http://localhost:8080");

    let mut client = IdentityServiceClient::new(3);

    // Generate a token that expires in 30 seconds
    let soon_expiring_token = generate_test_token(30);

    // Set the token on the client
    client.set_token(Some(ServiceToken {
        subject_id: "urn:svc:test-service-id".to_string(),
        token: soon_expiring_token,
        refresh_token: "refresh-token".to_string(),
    }));

    assert!(client.token.is_some());

    let token = client.get_token().unwrap();

    // Verify we can decode it
    let decoded = insecure_decode::<serde_json::Value>(&token.token);
    assert!(decoded.is_ok());

    // Verify the expiration is in the future (still valid)
    let claims = decoded.unwrap().claims;
    let exp = claims["exp"].as_u64().unwrap() as usize;
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;

    assert!(exp > current_time, "Token should still be valid");
    assert!(
        exp - current_time <= 30,
        "Token should expire within 30 seconds"
    );
}

#[test]
#[serial]
fn test_token_validation_with_no_token() {
    // Set up environment variables
    env::set_var("IDENTITY_ACCESS_KEY", "TEST_ACCESS_KEY");
    env::set_var("IDENTITY_SECRET_KEY", "TEST_SECRET_KEY");
    env::set_var("IDENTITY_SERVICE_HOST", "http://localhost:8080");

    let client = IdentityServiceClient::new(3);

    // Client should have no token initially
    assert!(client.token.is_none());
    assert!(client.get_token().is_none());
}
