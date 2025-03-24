use chrono::{NaiveDate, Utc};
use fake::{
    faker::{
        address::en::SecondaryAddress,
        finance::en::Bic,
        internet::en::FreeEmail,
        name::{en::FirstName, en::LastName},
        phone_number::en::PhoneNumber,
    },
    Fake,
};
use mykobo_rs::{
    identity::IdentityServiceClient,
    models::request::identity::{CustomerRequest, NewDocumentRequest, UpdateProfileRequest},
};
use pretty_assertions::assert_eq;
use serde::Serialize;
use std::{env, str::FromStr};
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
async fn test_customer_not_found() {
    let mock_server = MockServer::start().await;
    let wallet_server = MockServer::start().await;

    env::set_var("IDENTITY_ACCESS_KEY", "TEST_ACCESS_KEY");
    env::set_var("IDENTITY_SECRET_KEY", "TEST_SECRET_KEY");
    env::set_var("IDENTITY_SERVICE_HOST", mock_server.uri());
    env::set_var("WALLET_HOST", wallet_server.uri());

    Mock::given(method("POST"))
        .and(path("/authenticate"))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(read_file("tests/stubs/authenticate.json")),
        )
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path(
            "/kyc/profile/urn:usrp:fb497b2fcbfa479991de4e8b0abecad6",
        ))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(read_file("tests/stubs/identity_data_kyc_pending.json")),
        )
        .mount(&mock_server)
        .await;

    let mut identity_service_client = IdentityServiceClient::new(3);

    let profile = identity_service_client
        .get_kyc_status_with_profile("urn:usrp:fb497b2fcbfa479991de4e8b0abecad6", None)
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

#[tokio::test]
async fn test_get_profile() {
    let mock_server = MockServer::start().await;
    let wallet_server = MockServer::start().await;

    env::set_var("IDENTITY_ACCESS_KEY", "TEST_ACCESS_KEY");
    env::set_var("IDENTITY_SECRET_KEY", "TEST_SECRET_KEY");
    env::set_var("IDENTITY_SERVICE_HOST", mock_server.uri());
    env::set_var("WALLET_HOST", wallet_server.uri());

    Mock::given(method("POST"))
        .and(path("/authenticate"))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(read_file("tests/stubs/authenticate.json")),
        )
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path(
            "/user/profile/urn:usrp:acc7f99158f4419bb57613b38b68d494",
        ))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(read_file("tests/stubs/get_profile.json")),
        )
        .mount(&mock_server)
        .await;

    let mut identity_service_client = IdentityServiceClient::new(3);

    let profile = identity_service_client
        .get_profile_by_id("urn:usrp:acc7f99158f4419bb57613b38b68d494", None)
        .await;

    assert!(profile.is_ok());
    let profile = profile.unwrap();

    assert_eq!(
        profile.id,
        "urn:usrp:acc7f99158f4419bb57613b38b68d494".to_string()
    );

    assert_eq!(profile.kyc_status.review_status, "pending")
}

#[tokio::test]
async fn test_create_customer() {
    let mock_server = MockServer::start().await;
    let wallet_server = MockServer::start().await;

    env::set_var("IDENTITY_ACCESS_KEY", "TEST_ACCESS_KEY");
    env::set_var("IDENTITY_SECRET_KEY", "TEST_SECRET_KEY");
    env::set_var("IDENTITY_SERVICE_HOST", mock_server.uri());
    env::set_var("WALLET_HOST", wallet_server.uri());

    Mock::given(method("POST"))
        .and(path("/authenticate"))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(read_file("tests/stubs/authenticate.json")),
        )
        .mount(&mock_server)
        .await;

    Mock::given(method("POST"))
        .and(path("/user/profile/new"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(read_file("tests/stubs/new_customer_response.json")),
        )
        .mount(&mock_server)
        .await;

    let mut identity_service_client = IdentityServiceClient::new(3);
    let request = CustomerRequest {
        id: Some("urn:usrp:fb497b2fcbfa479991de4e8b0abecad6".to_string()),
        account: Some("GCGRZQ2OZWQVUWSRAFXSNL3N2KF4IVDOONNFBRP2G3622JJYCUYBCQE6".to_string()),
        first_name: Some(FirstName().fake()),
        last_name: Some(LastName().fake()),
        email_address: FreeEmail().fake(),
        additional_name: None,
        address: SecondaryAddress().fake(),
        mobile_number: PhoneNumber().fake(),
        birth_date: Some(NaiveDate::from_str("1990-01-01").unwrap()),
        birth_country_code: Some("UK".to_string()),
        bank_account_number: Some(Bic().fake()),
        tax_id: None,
        tax_id_name: None,
        credential_id: Some("urn:svcp:fb497b2fcbfa479991de4e8b0abecad6".to_string()),
    };

    let new_customer = identity_service_client.new_profile(request.clone()).await;
    assert!(new_customer.is_ok());
    let new_customer = new_customer.unwrap();
    assert_eq!(new_customer.first_name, "Test".to_string());
}

#[tokio::test]
async fn test_update_customer() {
    let mock_server = MockServer::start().await;
    let wallet_server = MockServer::start().await;

    env::set_var("IDENTITY_ACCESS_KEY", "TEST_ACCESS_KEY");
    env::set_var("IDENTITY_SECRET_KEY", "TEST_SECRET_KEY");
    env::set_var("IDENTITY_SERVICE_HOST", mock_server.uri());
    env::set_var("WALLET_HOST", wallet_server.uri());

    Mock::given(method("POST"))
        .and(path("/authenticate"))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(read_file("tests/stubs/authenticate.json")),
        )
        .mount(&mock_server)
        .await;

    Mock::given(method("PATCH"))
        .and(path("/user/profile/update"))
        .respond_with(
            ResponseTemplate::new(202)
                .set_body_string(read_file("tests/stubs/new_customer_response.json")),
        )
        .mount(&mock_server)
        .await;

    let request = UpdateProfileRequest {
        bank_account_number: Some("GB29NWBK60161331926819".to_string()),
        tax_id: None,
        tax_id_name: None,
        suspended_at: None,
        deleted_at: None,
    };

    let mut identity_service_client = IdentityServiceClient::new(3);

    let updated_customer = identity_service_client
        .update_profile(request.clone())
        .await;

    assert!(updated_customer.is_ok());
    let updated_customer = updated_customer.unwrap();
    assert_eq!(
        updated_customer.bank_account_number,
        Some("GB29NWBK60161331926819".to_string())
    );
}

#[tokio::test]
async fn test_new_document() {
    let mock_server = MockServer::start().await;
    let wallet_server = MockServer::start().await;

    env::set_var("IDENTITY_ACCESS_KEY", "TEST_ACCESS_KEY");
    env::set_var("IDENTITY_SECRET_KEY", "TEST_SECRET_KEY");
    env::set_var("IDENTITY_SERVICE_HOST", mock_server.uri());
    env::set_var("WALLET_HOST", wallet_server.uri());

    Mock::given(method("POST"))
        .and(path("/authenticate"))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(read_file("tests/stubs/authenticate.json")),
        )
        .mount(&mock_server)
        .await;

    Mock::given(method("PUT"))
        .and(path("/kyc/documents"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(read_file("tests/stubs/new_document_response.json")),
        )
        .mount(&mock_server)
        .await;
    let mut identity_service_client = IdentityServiceClient::new(3);

    let request = NewDocumentRequest {
        profile_id: "urn:usrp:fb497b2fcbfa479991de4e8b0abecad6".to_string(),
        document_type: "photo_id_front".to_string(),
        document_sub_type: Some("passport".to_string()),
        document_path: Some(
            "urn:usrp:fb497b2fcbfa479991de4e8b0abecad6/photo_id_front.png".to_string(),
        ),
        document_status: "pending".to_string(),
        created_at: Utc::now().naive_utc(),
        updated_at: None,
    };
    let new_document = identity_service_client.new_document(request.clone()).await;
    print!("{:?}", new_document);
    assert!(new_document.is_ok());
    let new_document = new_document.unwrap();
    assert_eq!(new_document.document_type, "photo_id_front".to_string());
    assert_eq!(
        new_document.profile_id,
        "urn:usrp:5028f1bf5a2e4fddb51f8d6f93a6b35f".to_string()
    )
}
