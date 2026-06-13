pub mod request;
pub mod response;

// Re-export commonly used types
pub use request::RegisterWalletRequest;
pub use response::{UserWallet, WalletData, WalletProfile};
