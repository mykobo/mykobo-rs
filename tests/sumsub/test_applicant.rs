use std::env;

use crate::read_file;
use mykobo_rs::{
    models::request::sumsub::{
        DocumentMetadata, InitiateVerificationRequest, NewApplicantRequest, NewDocumentRequest,
        ProfileData,
    },
    sumsub::SumsubClient,
};
use pretty_assertions::assert_eq;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

#[tokio::test]
async fn test_new_applicant() {
    let sumsub_server = MockServer::start().await;

    env::set_var("SUMSUB_HOST", sumsub_server.uri());

    Mock::given(method("POST"))
        .and(path("/create_applicant"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(read_file("tests/stubs/sumsub_applicant_success.json")),
        )
        .mount(&sumsub_server)
        .await;

    let sumsub_client = SumsubClient::new();
    let profile_data = ProfileData {
        first_name: "Kobi".to_string(),
        last_name: "Codie".to_string(),
        email: "kobi@mykobo.co".to_string(),
    };

    let applicant_request = NewApplicantRequest {
        external_user_id: "urn:usrp:bfac71ae055e4c4dbedc0e3c7b56b79e".to_string(),
        level_name: "sep6-kyc-level".to_string(),
        profile: profile_data,
    };

    let new_applicant = sumsub_client.create_applicant(applicant_request).await;

    assert!(new_applicant.is_ok());
    let applicant = new_applicant.unwrap();
    assert_eq!(
        applicant.external_user_id,
        "urn:usrp:bfac71ae055e4c4dbedc0e3c7b56b79e".to_string()
    );
    assert_eq!(applicant.id, "676c51021125c521e0b6704f".to_string())
}

#[tokio::test]
async fn test_get_applicant() {
    let sumsub_server = MockServer::start().await;

    env::set_var("SUMSUB_HOST", sumsub_server.uri());

    Mock::given(method("GET"))
        .and(path(
            "/get_applicant/urn:usrp:bfac71ae055e4c4dbedc0e3c7b56b79e",
        ))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(read_file("tests/stubs/sumsub_applicant_success.json")),
        )
        .mount(&sumsub_server)
        .await;

    let sumsub_client = SumsubClient::new();

    let existing_applicant = sumsub_client
        .get_applicant("urn:usrp:bfac71ae055e4c4dbedc0e3c7b56b79e".to_string())
        .await;

    assert!(existing_applicant.is_ok());
    let applicant = existing_applicant.unwrap();
    assert_eq!(
        applicant.external_user_id,
        "urn:usrp:bfac71ae055e4c4dbedc0e3c7b56b79e".to_string()
    );
    assert_eq!(applicant.id, "676c51021125c521e0b6704f".to_string())
}

#[tokio::test]
async fn test_submit_document() {
    let sumsub_server = MockServer::start().await;

    env::set_var("SUMSUB_HOST", sumsub_server.uri());

    Mock::given(method("POST"))
        .and(path("/add_document"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(read_file("tests/stubs/sumsub_add_document_success.json")),
        )
        .mount(&sumsub_server)
        .await;

    let sumsub_client = SumsubClient::new();

    let document_request = NewDocumentRequest {
        metadata: DocumentMetadata {
            id_doc_type: "ID_CARD".to_string(),
            id_doc_sub_type: Some("FRONT_SIDE".to_string()),
            country: Some("DEU".to_string()),
        },
        file_path: "urn:usrp:bfac71ae055e4c4dbedc0e3c7b56b79e/photo_id_front.png".to_string(),
        applicant_id: "676c51021125c521e0b6704f".to_string(),
    };

    let new_document = sumsub_client.submit_document(document_request).await;

    println!("{:?}", new_document);
    assert!(new_document.is_ok());
    let document = new_document.unwrap();
    assert_eq!(document.doc_id, "439993055".to_string());
    assert_eq!(document.id_doc_type, "ID_CARD".to_string());
    assert_eq!(document.id_doc_sub_type, Some("FRONT_SIDE".to_string()));
    assert_eq!(document.country, Some("DEU".to_string()));
}

#[tokio::test]
async fn test_initiate_check_success() {
    let sumsub_server = MockServer::start().await;

    env::set_var("SUMSUB_HOST", sumsub_server.uri());

    Mock::given(method("POST"))
        .and(path("/initiate_verification"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(read_file("tests/stubs/sumsub_initiate_check_success.json")),
        )
        .mount(&sumsub_server)
        .await;

    let sumsub_client = SumsubClient::new();

    let init_check_request = InitiateVerificationRequest {
        applicant_id: "676c51021125c521e0b6704f".to_string(),
        reason: "Documents uploaded via API".to_string(),
    };

    let init_check_response = sumsub_client.initiate_check(init_check_request).await;
    assert!(init_check_response.is_ok());
    assert!(init_check_response.unwrap().ok == 1);
}
