use lakhua::{geocode, geocode_h3, GeocodeOptions};

#[test]
fn test_geocode_h3_exact_match_res5() {
    let options = GeocodeOptions::default();
    let result = geocode_h3("853d838bfffffff", &options).unwrap();

    if let Some(loc) = result {
        assert!(!loc.location.city.is_empty());
        assert!(!loc.location.state.is_empty());
        assert_eq!(loc.matched_h3, "853d838bfffffff");
        assert_eq!(loc.matched_resolution, 5);
    }
}

#[test]
fn test_geocode_h3_exact_match_res4() {
    let options = GeocodeOptions::default();
    let result = geocode_h3("843d839ffffffff", &options).unwrap();

    if let Some(loc) = result {
        assert!(!loc.location.city.is_empty());
        assert!(!loc.location.state.is_empty());
        assert_eq!(loc.matched_h3, "843d839ffffffff");
        assert_eq!(loc.matched_resolution, 4);
    }
}

#[test]
fn test_geocode_h3_fallback_res5_to_res4() {
    let options = GeocodeOptions {
        resolution: 5,
        fallback: true,
        debug: false,
    };

    let result = geocode_h3("853d838bfffffff", &options).unwrap();
    if let Some(loc) = result {
        assert!(loc.matched_resolution == 4 || loc.matched_resolution == 5);
    }
}

#[test]
fn test_geocode_h3_no_fallback() {
    let options = GeocodeOptions {
        resolution: 5,
        fallback: false,
        debug: false,
    };

    let result = geocode_h3("853d838bfffffff", &options).unwrap();
    if let Some(loc) = result {
        assert_eq!(loc.matched_resolution, 5);
    }
}

#[test]
fn test_geocode_h3_invalid_index() {
    let options = GeocodeOptions::default();
    let result = geocode_h3("invalid_h3_index", &options);
    assert!(result.is_err());
}

#[test]
fn test_geocode_coordinates_delhi() {
    let options = GeocodeOptions::default();
    let result = geocode(28.6139, 77.2090, &options).unwrap();
    assert!(result.is_some());
    let loc = result.unwrap();
    assert!(!loc.location.city.is_empty());
    assert!(!loc.location.state.is_empty());
}

#[test]
fn test_geocode_coordinates_mumbai() {
    let options = GeocodeOptions::default();
    let result = geocode(19.0760, 72.8777, &options).unwrap();
    assert!(result.is_some());
    let loc = result.unwrap();
    assert!(!loc.location.city.is_empty());
}

#[test]
fn test_geocode_coordinates_out_of_range() {
    let options = GeocodeOptions::default();
    assert!(geocode(91.0, 77.0, &options).is_err());
    assert!(geocode(-91.0, 77.0, &options).is_err());
    assert!(geocode(28.0, 181.0, &options).is_err());
    assert!(geocode(28.0, -181.0, &options).is_err());
}

#[test]
fn test_geocode_coordinates_nan() {
    let options = GeocodeOptions::default();
    assert!(geocode(f64::NAN, 77.0, &options).is_err());
    assert!(geocode(28.0, f64::NAN, &options).is_err());
    assert!(geocode(f64::INFINITY, 77.0, &options).is_err());
}

#[test]
fn test_geocode_options_defaults() {
    let opts = GeocodeOptions::default();
    assert_eq!(opts.resolution, 5);
    assert!(opts.fallback);
    assert!(!opts.debug);
}

#[test]
fn test_geocode_result_structure() {
    let options = GeocodeOptions::default();
    let result = geocode_h3("853d838bfffffff", &options).unwrap();

    if let Some(loc) = result {
        assert!(loc.location.city.len() > 0);
        assert!(loc.location.state.len() > 0);
        assert!(loc.matched_h3.len() > 0);
        assert!(loc.matched_resolution >= 4 && loc.matched_resolution <= 5);
    }
}

#[test]
fn test_geocode_ocean_returns_none() {
    let options = GeocodeOptions::default();
    let result = geocode(0.0, 0.0, &options).unwrap();
    assert!(result.is_none());
}

#[test]
fn test_resolution_clamping() {
    let opts_low = GeocodeOptions {
        resolution: 1,
        fallback: false,
        debug: false,
    };
    let result = geocode(28.6139, 77.2090, &opts_low);
    assert!(result.is_ok());

    let opts_high = GeocodeOptions {
        resolution: 15,
        fallback: false,
        debug: false,
    };
    let result = geocode(28.6139, 77.2090, &opts_high);
    assert!(result.is_ok());
}
