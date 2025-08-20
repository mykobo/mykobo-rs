use mykobo_rs::business::{blacklisted_countries, whitelisted_countries};
use pretty_assertions::assert_eq;
#[test]
fn test_whitelisted_countries() {
    assert_eq!(whitelisted_countries().len(), 36);
    assert!(whitelisted_countries().contains_key("AR"));
    assert!(!whitelisted_countries().contains_key("AD")); // Andorra
}

#[test]
fn test_whitelisted_geo_zones() {
    assert_eq!(blacklisted_countries().len(), 63);
    assert!(blacklisted_countries().contains_key("RU"));
}
