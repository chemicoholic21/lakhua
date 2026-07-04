use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LocationDetails {
    pub city: String,
    pub state: String,
    pub district: Option<String>,
    pub pincode: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GeocodeResult {
    #[serde(flatten)]
    pub location: LocationDetails,
    pub matched_h3: String,
    pub matched_resolution: u8,
}

#[derive(Debug, Clone)]
pub struct GeocodeOptions {
    pub resolution: u8,
    pub fallback: bool,
    pub debug: bool,
}

impl Default for GeocodeOptions {
    fn default() -> Self {
        Self {
            resolution: 5,
            fallback: true,
            debug: false,
        }
    }
}
