use std::env;

use mykobo_rs::{models::response::auth::ServiceToken, wallets::WalletServiceClient};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

use crate::read_file;

#[tokio::test]
async fn test_get_customer_with_account_id() {
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
            "/user/wallet/GCGRZQ2OZWQVUWSRAFXSNL3N2KF4IVDOONNFBRP2G3622JJYCUYBCQE6",
        ))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(read_file("tests/stubs/new_wallet_success.json")),
        )
        .mount(&wallet_server)
        .await;

    let mut wallet_service_client = WalletServiceClient::new(3);
    let wallet_customer = wallet_service_client
        .get_customer(
            Some(ServiceToken {
                subject_id: "SUBJECT_ID".to_string(),
                token: "TOKEN".to_string(),
                refresh_token: "REFRESH_TOKEN".to_string(),
            }),
            "GCGRZQ2OZWQVUWSRAFXSNL3N2KF4IVDOONNFBRP2G3622JJYCUYBCQE6",
            None,
        )
        .await;

    assert!(wallet_customer.is_ok());
    let wa = wallet_customer.unwrap();
    assert_eq!(
        wa.profile_id,
        "urn:usrp:6bd0201b36f64008bd94209aef8d2b15".to_string()
    )
}

#[tokio::test]
async fn test_get_customer_not_found() {
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
            "/user/wallet/GCGRZQ2OZWQVUWSRAFXSNL3N2KF4IVDOONNFBRP2G3622JJYCUYBCQE6",
        ))
        .respond_with(
            ResponseTemplate::new(404)
                .set_body_string(read_file("tests/stubs/wallet_not_found.json")),
        )
        .mount(&wallet_server)
        .await;

    let mut wallet_service_client = WalletServiceClient::new(3);
    let wallet_customer = wallet_service_client
        .get_customer(
            Some(ServiceToken {
                subject_id: "SUBJECT_ID".to_string(),
                token: "TOKEN".to_string(),
                refresh_token: "REFRESH_TOKEN".to_string(),
            }),
            "GCGRZQ2OZWQVUWSRAFXSNL3N2KF4IVDOONNFBRP2G3622JJYCUYBCQE6",
            None,
        )
        .await;
    print!("{:?}", wallet_customer);
    assert!(wallet_customer.is_err());
}
