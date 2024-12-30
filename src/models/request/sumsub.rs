use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessTokenRequest {
    // this is our profile id. For sumsub it would be external to them.
    pub external_user_id: String,
    // level name is the KYC level for which to derive this token from. Usually the SEP6 level, if it's non-interactive
    pub level_name: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileData {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewApplicantRequest {
    pub external_user_id: String,
    pub level_name: String,
    pub profile: ProfileData,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentMetadata {
    pub id_doc_type: String,
    pub id_doc_sub_type: String,
    pub country: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewDocumentRequest {
    pub metadata: DocumentMetadata,
    pub file_path: String,
    pub applicant_id: String,
}
