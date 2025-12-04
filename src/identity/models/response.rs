use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
    pub aud: String,
    pub scope: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KycStatus {
    pub review_status: String,
    pub received_at: Option<NaiveDateTime>,
}

impl Default for KycStatus {
    fn default() -> Self {
        KycStatus {
            review_status: "pending".to_string(),
            received_at: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfileResponse {
    #[serde(default)]
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email_address: String,
    pub additional_name: Option<String>,
    pub address_line_1: Option<String>,
    pub address_line_2: Option<String>,
    pub mobile_number: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub birth_country_code: Option<String>,
    pub id_country_code: Option<String>,
    pub bank_account_number: Option<String>,
    pub bank_number: Option<String>,
    pub tax_id: Option<String>,
    pub tax_id_name: Option<String>,
    pub credential_id: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub suspended_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct KycDocumentResponse {
    pub id: String,
    pub profile_id: String,
    pub document_type: String,
    pub document_sub_type: Option<String>,
    pub document_status: String,
    pub document_path: Option<String>,
    pub reject_reason: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
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
    pub received_at: NaiveDateTime,
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
    pub address_line_1: Option<String>,
    pub address_line_2: Option<String>,
    pub mobile_number: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub birth_country_code: Option<String>,
    pub id_country_code: Option<String>,
    pub bank_account_number: Option<String>,
    pub bank_number: Option<String>,
    pub tax_id: Option<String>,
    pub tax_id_name: Option<String>,
    pub created_at: NaiveDateTime,
    pub kyc_status: Option<KycApplicantReviewResponse>,
    pub kyc_documents: Vec<KycDocumentResponse>,
}

#[derive(Debug, Deserialize)]
pub struct CustomerResponse {
    #[serde(default)]
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email_address: String,
    pub additional_name: Option<String>,
    pub address_line_1: Option<String>,
    pub address_line_2: Option<String>,
    pub mobile_number: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub birth_country_code: Option<String>,
    pub id_country_code: Option<String>,
    pub bank_account_number: Option<String>,
    pub bank_number: Option<String>,
    pub tax_id: Option<String>,
    pub tax_id_name: Option<String>,
    pub credential_id: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub suspended_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub otp_verified: Option<bool>,
    pub kyc_status: KycStatus,
}

#[derive(Debug, Deserialize)]
pub struct NewDocumentResponse {
    pub profile_id: String,
    pub document_type: String,
    pub document_status: Option<String>,
    pub document_path: Option<String>,
    pub created_at: String,
    pub updated_at: Option<String>,
    pub document_sub_type: Option<String>,
    pub reject_reason: Option<String>,
    pub id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GroupedIndicator {
    pub score: f64,
    pub breakdown: std::collections::HashMap<String, f64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RiskScoreBreakdown {
    pub total_score: f64,
    pub verification: std::collections::HashMap<String, f64>,
    pub source_of_funds: Option<GroupedIndicator>,
    pub country_risk_jurisdiction: Option<GroupedIndicator>,
    pub expected_volume: Option<GroupedIndicator>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserRiskScoreHistory {
    pub id: String,
    pub profile_id: String,
    pub score: f64,
    pub created_at: NaiveDateTime,
}
#[derive(Debug, Deserialize)]
pub struct UserRiskProfileResponse {
    pub risk_score: f64,
    pub break_down: RiskScoreBreakdown,
    pub latest_score_history: Option<UserRiskScoreHistory>,
}
