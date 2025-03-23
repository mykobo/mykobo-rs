use std::env;

use reqwest::Client;

use crate::{
    models::{
        request::sumsub::{
            AccessTokenRequest, InitiateVerificationRequest, NewApplicantRequest,
            NewDocumentRequest,
        },
        response::{
            sumsub::{
                AccessTokenResponse, ApplicantResponse, InitiateVerificationResponse,
                NewDocumentResponse,
            },
            ServiceError,
        },
    },
    util::parse_response,
};

pub struct SumsubClient {
    pub host: String,
    pub client: reqwest::Client,
}

impl Default for SumsubClient {
    fn default() -> Self {
        Self::new()
    }
}

impl SumsubClient {
    pub fn new() -> Self {
        Self {
            host: env::var("SUMSUB_HOST").expect("SUMSUB_HOST must be set"),
            client: Client::new(),
        }
    }

    // Generate an access token for a given profile.
    pub async fn get_access_token(
        &self,
        request: AccessTokenRequest,
    ) -> Result<AccessTokenResponse, ServiceError> {
        let url = format!("{}/access_token", self.host);
        let response = self.client.post(url).json(&request).send().await;

        parse_response::<AccessTokenResponse>(response).await
    }

    // Given a profile id, get the details from sumsub.
    pub async fn get_applicant(
        &self,
        profile_id: String,
    ) -> Result<ApplicantResponse, ServiceError> {
        let url = format!("{}/get_applicant/{}", self.host, profile_id);
        let response = self.client.get(url).send().await;

        parse_response::<ApplicantResponse>(response).await
    }

    pub async fn create_applicant(
        &self,
        applicant_request: NewApplicantRequest,
    ) -> Result<ApplicantResponse, ServiceError> {
        let url = format!("{}/create_applicant", self.host);
        let response = self.client.post(url).json(&applicant_request).send().await;

        parse_response::<ApplicantResponse>(response).await
    }

    pub async fn submit_document(
        &self,
        new_document_request: NewDocumentRequest,
    ) -> Result<NewDocumentResponse, ServiceError> {
        let url = format!("{}/add_document", self.host);
        let response = self
            .client
            .post(url)
            .json(&new_document_request)
            .send()
            .await;
        parse_response::<NewDocumentResponse>(response).await
    }

    pub async fn initiate_check(
        &self,
        initiate_verification_request: InitiateVerificationRequest,
    ) -> Result<InitiateVerificationResponse, ServiceError> {
        let url = format!("{}/initiate_verification", self.host);
        let response = self
            .client
            .post(url)
            .json(&initiate_verification_request)
            .send()
            .await;
        parse_response::<InitiateVerificationResponse>(response).await
    }
}
