use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Deserialize)]
pub struct ServiceToken {
    pub subject_id: String,
    pub token: String,
    pub refresh_token: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TokenCheckResponse {
    pub authorised: bool,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
    pub aud: String,
    pub scope: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfileResponse {
    #[serde(default)]
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email_address: String,
    pub additional_name: Option<String>,
    pub address: Option<String>,
    pub mobile_number: Option<String>,
    pub birth_date: Option<chrono::NaiveDate>,
    pub birth_country_code: Option<String>,
    pub bank_account_number: Option<String>,
    pub tax_id: Option<String>,
    pub tax_id_name: Option<String>,
    pub credential_id: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub suspended_at: Option<chrono::NaiveDateTime>,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct KycDocumentResponse {
    pub id: i32,
    pub profile_id: String,
    pub document_type: String,
    pub document_sub_type: Option<String>,
    pub document_status: String,
    pub document_data: Option<String>,
    pub reject_reason: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct KycApplicantReviewResponse {
    pub id: String,
    pub profile_id: Option<String>,
    pub review_status: String,
    pub webhook_type: String,
    pub applicant_id: String,
    pub correlation_id: Option<String>,
    pub review_result: Option<String>,
    pub received_at: chrono::NaiveDateTime,
    pub level_name: Option<String>,
    pub admin_comment: Option<String>,
    pub user_comment: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct UserKycStatusResponse {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email_address: String,
    pub additional_name: Option<String>,
    pub address: Option<String>,
    pub mobile_number: Option<String>,
    pub birth_date: Option<chrono::NaiveDate>,
    pub birth_country_code: Option<String>,
    pub bank_account_number: Option<String>,
    pub tax_id: Option<String>,
    pub tax_id_name: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub kyc_status: Option<KycApplicantReviewResponse>,
    pub kyc_documents: Vec<KycDocumentResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MykoboStatusCode {
    NotFound,
    BadRequest,
    Unauthorised,
    DependencyFailed,
}

impl From<StatusCode> for MykoboStatusCode {
    fn from(status: StatusCode) -> Self {
        match status {
            StatusCode::NOT_FOUND => MykoboStatusCode::NotFound,
            StatusCode::BAD_REQUEST => MykoboStatusCode::BadRequest,
            StatusCode::UNAUTHORIZED => MykoboStatusCode::Unauthorised,
            _ => MykoboStatusCode::DependencyFailed,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceError {
    pub error: Option<String>,
    pub message: Option<String>,
    pub status: MykoboStatusCode,
}

impl Display for ServiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = self
            .error
            .clone()
            .unwrap_or("Unknown error - U".to_string());
        write!(f, "{}", message)
    }
}

impl From<reqwest::Error> for ServiceError {
    fn from(error: reqwest::Error) -> Self {
        let r_message = if error.is_connect() || error.is_timeout() {
            Some("Connection error".to_string())
        } else if error.is_request() {
            Some("Bad Request".to_string())
        } else {
            Some("An unknown error occurred".to_string())
        };

        ServiceError {
            error: r_message.clone().unwrap().to_string().into(),
            message: r_message,
            status: MykoboStatusCode::from(error.status().unwrap_or(StatusCode::BAD_REQUEST)),
        }
    }
}

#[derive(Deserialize)]
pub struct WalletProfile {
    pub profile_id: String,
}
