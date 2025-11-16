pub mod request;
pub mod response;

// Re-export commonly used types
pub use request::{
    Credentials, CustomerRequest, NewDocumentRequest, RefreshToken, TokenCheckRequest,
    UpdateProfileRequest, ValidateToken,
};
pub use response::{
    CustomerResponse, KycApplicantReviewResponse, KycDocumentResponse, KycStatus,
    NewDocumentResponse, ServiceToken, TokenCheckResponse, TokenClaims, UserKycStatusResponse,
    UserProfileResponse,
};
