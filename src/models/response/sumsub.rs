use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessTokenResponse {
    pub token: String,
    pub user_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewDocumentResponse {
    pub id_doc_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_doc_sub_type: Option<String>,
    pub country: String,
    pub doc_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicantReview {
    pub level_name: String,
    pub review_status: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicantResponse {
    pub id: String,
    pub external_user_id: String,
    pub inspection_id: String,
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review: Option<ApplicantReview>,
}
