pub mod request;
pub mod response;

// Re-export commonly used types
pub use request::{
    Credentials, CustomerRequest, NewDocumentRequest, PatchScopesRequest, RefreshToken,
    TokenCheckRequest, UpdateProfileRequest, UpdateServiceProfileRequest, ValidateToken,
};
pub use response::{
    CredentialsResponse, CustomerResponse, KycApplicantReviewResponse, KycDocumentResponse,
    KycStatus, NewDocumentResponse, PaginatedServicesResponse, ServiceResponse, ServiceToken,
    TokenCheckResponse, TokenClaims, UserKycStatusResponse, UserProfileResponse,
};
