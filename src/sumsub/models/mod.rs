pub mod request;
pub mod response;

// Re-export commonly used types
pub use request::{
    AccessTokenRequest, DocumentMetadata, InitiateVerificationRequest, NewApplicantRequest,
    NewDocumentRequest, ProfileData,
};
pub use response::{
    AccessTokenResponse, ApplicantResponse, ApplicantReview, InitiateVerificationResponse,
    NewDocumentResponse, ReviewResult,
};
