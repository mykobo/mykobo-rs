use mykobo_rs::business::whitelisted_countries;
use pretty_assertions::assert_eq;
#[test]
fn test_whitelisted_countries() {
    assert_eq!(whitelisted_countries().len(), 35)
}
