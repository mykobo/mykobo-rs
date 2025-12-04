use crate::read_file;
use mykobo_rs::identity::models::{CustomerResponse, ServiceToken, UserKycStatusResponse};
use pretty_assertions::assert_eq;
use mykobo_rs::identity::models::response::UserRiskProfileResponse;

#[test]
fn test_deserialise_new_customer_response() {
    let content = read_file("tests/stubs/new_customer_response.json");
    let customer = serde_json::from_str(&content);

    assert!(customer.is_ok());
    let customer: CustomerResponse = customer.unwrap();
    assert_eq!(
        customer.id,
        "urn:usrp:5028f1bf5a2e4fddb51f8d6f93a6b35f".to_string()
    );
    assert_eq!(customer.kyc_status.review_status, "pending".to_string());
    assert_eq!(
        customer.bank_account_number,
        Some("GB29NWBK60161331926819".to_string())
    );
    assert_eq!(customer.email_address, "test7@gmail.com".to_string());
    assert_eq!(customer.first_name, "Test".to_string());
    assert_eq!(customer.last_name, "Test".to_string());
    assert_eq!(
        customer.credential_id,
        Some("urn:usr:4aaa23c86d914dbdb9f2f22cd1870e33".to_string()),
    )
}

#[test]
fn test_deserialise_authenticate_response() {
    let content = read_file("tests/stubs/authenticate.json");
    let token = serde_json::from_str(&content);

    assert!(token.is_ok());
    let token: ServiceToken = token.unwrap();
    assert_eq!(
        token.subject_id,
        "urn:svc:94fc474ee7144ed181855d63f0a2bcad".to_string()
    );

    assert_eq!(token.token,
        "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ1cm46c3ZjOjk0ZmM0NzRlZTcxNDRlZDE4MTg1NWQ2M2YwYTJiY2FkIiwianRpIjoidXJuOnRrbjoyZWU0N2ZjYTI2OTg0Y2MyODZjZDgyNDQ5NzVkNGVhYSIsImlzcyI6ImxvY2FsaG9zdCIsImlhdCI6MTczMjEyMDk4NCwiZXhwIjoxNzMyNzI1Nzg0LCJhdWQiOiJTZXJ2aWNlIiwic2NvcGUiOlsidXNlcjpyZWFkIiwidXNlcjp3cml0ZSIsInVzZXI6YWRtaW4iLCJzZXJ2aWNlOnJlYWQiLCJ0b2tlbjpyZWFkIiwiYnVzaW5lc3M6cmVhZCIsImJ1c2luZXNzOnJlYWQiLCJ3YWxsZXQ6cmVhZCIsIndhbGxldDp3cml0ZSIsInRyYW5zYWN0aW9uOnJlYWQiLCJ0cmFuc2FjdGlvbjp3cml0ZSIsInRyYW5zYWN0aW9uOmFkbWluIl19.t7Tl4kOf8emvPhUL4tTsBau9F3LEPFsnUAX7c2uxMVA".to_string())
}

#[test]
fn test_deserialise_identity_response() {
    let pending_response = read_file("tests/stubs/identity_data_kyc_pending.json");
    let pending = serde_json::from_str(&pending_response);
    assert!(pending.is_ok());
    let pending: UserKycStatusResponse = pending.unwrap();
    assert!(pending
        .kyc_status
        .is_some_and(|k| k.review_status == *"init"));
    // ---- Completed KYC response
    let completed_response = read_file("tests/stubs/identity_data_kyc_completed.json");
    let completed = serde_json::from_str(&completed_response);
    assert!(completed.is_ok());
    let completed: UserKycStatusResponse = completed.unwrap();
    assert!(completed.kyc_status.is_some_and(|k| {
        k.review_status == *"completed" && k.review_result == Some("GREEN".to_string())
    }))
}

#[test]
fn test_deserialise_identity_risk_response() {
    let risk_score_file = read_file("tests/stubs/identity_user_risk_score.json");
    let user_risk_score = serde_json::from_str(&risk_score_file);
    assert!(user_risk_score.is_ok());
    let profile: UserRiskProfileResponse = user_risk_score.unwrap();
    assert!(profile
        .latest_score_history
        .is_some_and(|k| k.score == 10.5));
}