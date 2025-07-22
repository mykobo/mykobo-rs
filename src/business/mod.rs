use std::collections::HashMap;

pub fn whitelisted_countries() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("AU", "Australia");
    map.insert("AT", "Austria");
    map.insert("BE", "Belgium");
    map.insert("BG", "Bulgaria");
    map.insert("CA", "Canada");
    map.insert("HR", "Croatia");
    map.insert("CZ", "Czech Republic");
    map.insert("DK", "Denmark");
    map.insert("EE", "Estonia");
    map.insert("FI", "Finland");
    map.insert("FR", "France");
    map.insert("DE", "Germany");
    map.insert("GR", "Greece");
    map.insert("GI", "Gibraltar");
    map.insert("HU", "Hungary");
    map.insert("IS", "Iceland");
    map.insert("IE", "Ireland");
    map.insert("IT", "Italy");
    map.insert("LV", "Latvia");
    map.insert("LI", "Liechtenstein");
    map.insert("LT", "Lithuania");
    map.insert("LU", "Luxembourg");
    map.insert("ME", "Montenegro");
    map.insert("NL", "Netherlands");
    map.insert("NZ", "New Zealand");
    map.insert("NO", "Norway");
    map.insert("PL", "Poland");
    map.insert("PT", "Portugal");
    map.insert("RO", "Romania");
    map.insert("SK", "Slovakia");
    map.insert("SI", "Slovenia");
    map.insert("ES", "Spain");
    map.insert("SE", "Sweden");
    map.insert("CH", "Switzerland");
    map.insert("US", "United States");
    map
}
