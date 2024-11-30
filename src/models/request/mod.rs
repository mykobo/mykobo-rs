use chrono::{NaiveDate, NaiveDateTime};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct RefreshToken {
    pub refresh_token: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TokenCheckRequest {
    pub token: String,
    pub scope: Option<String>,
    pub subject: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ValidateToken {
    pub token: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Credentials {
    pub access_key: String,
    pub secret_key: String,
}

impl Credentials {
    pub fn new(access_key: String, secret_key: String) -> Self {
        Self {
            access_key,
            secret_key,
        }
    }
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct CustomerRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_id: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
pub struct NewDocumentRequest {
    pub profile_id: String,
    pub document_type: String,
    pub document_status: String,
    pub document_data: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub document_sub_type: Option<String>,
}
