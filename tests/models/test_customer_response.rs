use mykobo_rs::identity::IdentityServiceClient;
use pretty_assertions::assert_eq;
use serde::Serialize;
use std::env;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

use crate::read_file;

#[derive(Debug, Clone, Serialize)]
pub struct Credentials {
    pub access_key: String,
    pub secret_key: String,
}

#[tokio::test]
async fn test_customer_not_found_transformation() {
    // Start a background HTTP server on a random local port
    let mock_server = MockServer::start().await;

    env::set_var("IDENTITY_ACCESS_KEY", "TEST_ACCESS_KEY");
    env::set_var("IDENTITY_SECRET_KEY", "TEST_SECRET_KEY");
    env::set_var("IDENTITY_SERVICE_HOST", mock_server.uri());

    // Arrange the behaviour of the MockServer adding a Mock:
    Mock::given(method("POST"))
        .and(path("/authenticate"))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(read_file("tests/stubs/authenticate.json")),
        )
        // Mounting the mock on the mock server - it's now effective!
        .mount(&mock_server)
        .await;

    // Arrange the behaviour of the MockServer adding a Mock:
    Mock::given(method("GET"))
        .and(path(
            "/kyc/profile/urn:usrp:fb497b2fcbfa479991de4e8b0abecad6",
        ))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(read_file("tests/stubs/identity_data_kyc_pending.json")),
        )
        // Mounting the mock on the mock server - it's now effective!
        .mount(&mock_server)
        .await;

    let mut identity_service_client = IdentityServiceClient::new(3);

    let profile = identity_service_client
        .get_profile("urn:usrp:fb497b2fcbfa479991de4e8b0abecad6".to_string())
        .await;

    assert!(profile.is_ok());
    let profile = profile.unwrap();

    assert!(identity_service_client.token.is_some());
    assert_eq!(
        profile.id,
        "urn:usrp:fb497b2fcbfa479991de4e8b0abecad6".to_string()
    );
    assert!(profile.kyc_status.is_some())
}
