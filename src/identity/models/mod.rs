pub mod request;
pub mod response;

// Re-export commonly used types
pub use request::{Credentials, CustomerRequest, NewDocumentRequest, TokenCheckRequest, UpdateProfileRequest, RefreshToken, ValidateToken};
pub use response::{CustomerResponse, KycStatus, NewDocumentResponse, ServiceToken, TokenCheckResponse, TokenClaims, UserKycStatusResponse, KycDocumentResponse, KycApplicantReviewResponse, UserProfileResponse};
