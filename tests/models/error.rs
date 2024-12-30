use mykobo_rs::models::response::{MykoboStatusCode, ServiceError};

use crate::read_file;

#[test]
fn test_error_deserialise() {
    let content = read_file("tests/stubs/wallet_not_found.json");
    let service_error: ServiceError = serde_json::from_str(content.as_str()).unwrap();
    assert_eq!(service_error.error, Some("Not Found".to_string()));
    assert_eq!(service_error.message, Some("The requested wallet [GCGRZQ2OZWQVUWSRAFXSNL3N2KF4IVDOONNFBRP2G3622JJYCUYBCQE7] optional memo [wrongmemo] could not be found".to_string()));
    assert_eq!(service_error.status, MykoboStatusCode::DependencyFailed);
}
