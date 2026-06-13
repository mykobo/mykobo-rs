use std::collections::HashMap;

pub fn whitelisted_countries() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("AU", "Australia");
    map.insert("AR", "Argentina");
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

pub fn whitelisted_countries_risk() -> HashMap<&'static str, (&'static str, &'static str)> {
    let mut map = HashMap::new();
    map.insert("AU", ("Australia", "LOW"));
    map.insert("AR", ("Argentina", "LOW"));
    map.insert("AT", ("Austria", "LOW"));
    map.insert("BE", ("Belgium", "LOW"));
    map.insert("BG", ("Bulgaria", "MEDIUM"));
    map.insert("CA", ("Canada", "LOW"));
    map.insert("HR", ("Croatia", "MEDIUM"));
    map.insert("CZ", ("Czech Republic", "MEDIUM"));
    map.insert("DK", ("Denmark", "LOW"));
    map.insert("EE", ("Estonia", "LOW"));
    map.insert("FI", ("Finland", "LOW"));
    map.insert("FR", ("France", "LOW"));
    map.insert("DE", ("Germany", "LOW"));
    map.insert("GR", ("Greece", "LOW"));
    map.insert("GI", ("Gibraltar", "LOW"));
    map.insert("HU", ("Hungary", "MEDIUM"));
    map.insert("IS", ("Iceland", "LOW"));
    map.insert("IE", ("Ireland", "LOW"));
    map.insert("IT", ("Italy", "LOW"));
    map.insert("LV", ("Latvia", "LOW"));
    map.insert("LI", ("Liechtenstein", "LOW"));
    map.insert("LT", ("Lithuania", "LOW"));
    map.insert("LU", ("Luxembourg", "LOW"));
    map.insert("ME", ("Montenegro", "LOW"));
    map.insert("NL", ("Netherlands", "LOW"));
    map.insert("NZ", ("New Zealand", "LOW"));
    map.insert("NO", ("Norway", "LOW"));
    map.insert("PL", ("Poland", "LOW"));
    map.insert("PT", ("Portugal", "LOW"));
    map.insert("RO", ("Romania", "MEDIUM"));
    map.insert("SK", ("Slovakia", "MEDIUM"));
    map.insert("SI", ("Slovenia", "MEDIUM"));
    map.insert("ES", ("Spain", "LOW"));
    map.insert("SE", ("Sweden", "LOW"));
    map.insert("CH", ("Switzerland", "LOW"));
    map.insert("US", ("United States", "LOW"));
    map
}

pub fn get_risk_jurisdiction(country_code: &str) -> Option<String> {
    whitelisted_countries_risk()
        .get(country_code)
        .map(|(_, risk)| risk.to_string())
}

pub fn blacklisted_countries() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("AF", "Afghanistan");
    map.insert("AL", "Albania");
    map.insert("AS", "American Samoa");
    map.insert("AO", "Angola");
    map.insert("AM", "Armenia");
    map.insert("BB", "Barbados");
    map.insert("BY", "Belarus");
    map.insert("BF", "Burkina Faso");
    map.insert("KH", "Cambodia");
    map.insert("KY", "Cayman Islands");
    map.insert("CF", "Central African Republic");
    map.insert("TD", "Chad");
    map.insert("KM", "Comoros");
    map.insert("CN", "China");
    map.insert("CU", "Cuba");
    map.insert("KP", "North Korea");
    map.insert("CD", "Democratic Republic of the Congo");
    map.insert("GQ", "Equatorial Guinea");
    map.insert("FJ", "Fiji");
    map.insert("GA", "Gabon");
    map.insert("GU", "Guam");
    map.insert("GW", "Guinea-Bissau");
    map.insert("HT", "Haiti");
    map.insert("HK", "Hong Kong");
    map.insert("IN", "India");
    map.insert("IR", "Iran");
    map.insert("IQ", "Iraq");
    map.insert("IL", "Israel");
    map.insert("JM", "Jamaica");
    map.insert("JO", "Jordan");
    map.insert("KZ", "Kazakhstan");
    map.insert("KG", "Kyrgyzstan");
    map.insert("LA", "Laos");
    map.insert("LB", "Lebanon");
    map.insert("LY", "Libya");
    map.insert("ML", "Mali");
    map.insert("MX", "Mexico");
    map.insert("MA", "Morocco");
    map.insert("MZ", "Mozambique");
    map.insert("MM", "Myanmar");
    map.insert("NI", "Nicaragua");
    map.insert("PK", "Pakistan");
    map.insert("PS", "Palestine");
    map.insert("PA", "Panama");
    map.insert("PH", "Philippines");
    map.insert("RU", "Russia");
    map.insert("WS", "Samoa");
    map.insert("SN", "Senegal");
    map.insert("SO", "Somalia");
    map.insert("SS", "South Sudan");
    map.insert("SD", "Sudan");
    map.insert("SY", "Syria");
    map.insert("TT", "Trinidad and Tobago");
    map.insert("TR", "Turkey");
    map.insert("UG", "Uganda");
    map.insert("UA", "Ukraine");
    map.insert("VI", "U.S. Virgin Islands");
    map.insert("UZ", "Uzbekistan");
    map.insert("VU", "Vanuatu");
    map.insert("YE", "Yemen");
    map.insert("GB", "United Kingdom");
    map.insert("VE", "Venezuela");
    map.insert("ZW", "Zimbabwe");
    // Special regions (Crimea, Donetsk, etc.) can be handled separately if needed
    map
}
