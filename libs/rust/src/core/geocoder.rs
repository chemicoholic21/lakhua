use h3o::{CellIndex, LatLng, Resolution};
use log::debug;

use crate::config::{clamp_resolution, MIN_RESOLUTION};
use crate::core::data_loader;
use crate::error::LakhuaError;
use crate::types::{GeocodeOptions, GeocodeResult, LocationDetails};

fn resolution_to_h3(res: u8) -> Result<Resolution, LakhuaError> {
    match res {
        0 => Ok(Resolution::Zero),
        1 => Ok(Resolution::One),
        2 => Ok(Resolution::Two),
        3 => Ok(Resolution::Three),
        4 => Ok(Resolution::Four),
        5 => Ok(Resolution::Five),
        6 => Ok(Resolution::Six),
        7 => Ok(Resolution::Seven),
        8 => Ok(Resolution::Eight),
        9 => Ok(Resolution::Nine),
        10 => Ok(Resolution::Ten),
        11 => Ok(Resolution::Eleven),
        12 => Ok(Resolution::Twelve),
        13 => Ok(Resolution::Thirteen),
        14 => Ok(Resolution::Fourteen),
        15 => Ok(Resolution::Fifteen),
        _ => Err(LakhuaError::H3(format!(
            "Invalid resolution: {}",
            res
        ))),
    }
}

fn build_result(
    location: &LocationDetails,
    h3_index: &str,
    resolution: u8,
) -> GeocodeResult {
    GeocodeResult {
        location: location.clone(),
        matched_h3: h3_index.to_string(),
        matched_resolution: resolution,
    }
}

pub fn geocode_h3(
    h3_index: &str,
    options: &GeocodeOptions,
) -> Result<Option<GeocodeResult>, LakhuaError> {
    data_loader::load_all_stores_once()?;

    let cell = h3_index
        .parse::<CellIndex>()
        .map_err(|e| LakhuaError::InvalidH3Index(e.to_string()))?;

    let input_resolution = cell.resolution() as u8;
    let start_resolution = clamp_resolution(input_resolution);

    if options.debug {
        debug!(
            "geocode_h3: h3_index={}, input_resolution={}, start_resolution={}",
            h3_index, input_resolution, start_resolution
        );
    }

    let end_resolution = if options.fallback { MIN_RESOLUTION } else { start_resolution };

    for resolution in (end_resolution..=start_resolution).rev() {
        let candidate = if resolution == input_resolution {
            h3_index.to_string()
        } else {
            let h3_resolution = resolution_to_h3(resolution)?;
            match cell.parent(h3_resolution) {
                Some(parent) => parent.to_string(),
                None => continue,
            }
        };

        if let Some(store) = data_loader::get_store(resolution) {
            if let Some(location) = store.get(&candidate) {
                if options.debug {
                    debug!("Match found at resolution {}: {}", resolution, candidate);
                }
                return Ok(Some(build_result(location, &candidate, resolution)));
            }
        }
    }

    if options.debug {
        debug!("No match found for h3_index={}", h3_index);
    }

    Ok(None)
}

pub fn geocode(
    lat: f64,
    lon: f64,
    options: &GeocodeOptions,
) -> Result<Option<GeocodeResult>, LakhuaError> {
    if !lat.is_finite() || !lon.is_finite() {
        return Err(LakhuaError::InvalidCoordinate(lat, lon));
    }
    if !(-90.0..=90.0).contains(&lat) || !(-180.0..=180.0).contains(&lon) {
        return Err(LakhuaError::InvalidCoordinate(lat, lon));
    }

    data_loader::load_all_stores_once()?;

    let resolution = clamp_resolution(options.resolution);
    let h3_resolution = resolution_to_h3(resolution)?;

    let coord = LatLng::new(lat, lon)
        .map_err(|_| LakhuaError::InvalidCoordinate(lat, lon))?;

    let cell = coord.to_cell(h3_resolution);
    let h3_index = cell.to_string();

    if options.debug {
        debug!(
            "geocode: lat={}, lon={}, resolution={}, h3_index={}",
            lat, lon, resolution, h3_index
        );
    }

    geocode_h3(&h3_index, options)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::data_loader;
    use std::collections::HashMap;
    use std::sync::{LazyLock, Mutex};

    static TEST_MUTEX: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

    fn lock() -> std::sync::MutexGuard<'static, ()> {
        TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner())
    }

    fn test_options() -> GeocodeOptions {
        GeocodeOptions {
            resolution: 5,
            fallback: true,
            debug: false,
        }
    }

    fn make_location(city: &str) -> LocationDetails {
        LocationDetails {
            city: city.to_string(),
            state: "Test State".to_string(),
            district: None,
            pincode: None,
        }
    }

    #[test]
    fn test_geocode_h3_exact_match() {
        let _lock = lock();
        data_loader::clear_store_cache();

        let mut store5 = HashMap::new();
        store5.insert("853d838bfffffff".to_string(), make_location("Orchha"));

        let mut stores = HashMap::new();
        stores.insert(5u8, store5);
        data_loader::set_stores_for_testing(stores);

        let result = geocode_h3("853d838bfffffff", &test_options())
            .unwrap()
            .unwrap();

        assert_eq!(result.location.city, "Orchha");
        assert_eq!(result.matched_h3, "853d838bfffffff");
        assert_eq!(result.matched_resolution, 5);
    }

    #[test]
    fn test_geocode_h3_fallback_to_resolution_4() {
        let _lock = lock();
        data_loader::clear_store_cache();

        let mut store4 = HashMap::new();
        store4.insert("843d839ffffffff".to_string(), make_location("OrchhaRegion"));

        let mut store5 = HashMap::new();
        store5.insert("853d838bfffffff".to_string(), make_location("Orchha"));

        let mut stores = HashMap::new();
        stores.insert(4u8, store4);
        stores.insert(5u8, store5);
        data_loader::set_stores_for_testing(stores);

        let opts = GeocodeOptions {
            resolution: 5,
            fallback: true,
            debug: false,
        };

        let result = geocode_h3("853d838bfffffff", &opts).unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn test_geocode_h3_no_match() {
        let _lock = lock();
        data_loader::clear_store_cache();

        let mut store5 = HashMap::new();
        store5.insert(
            "851fb467fffffff".to_string(),
            make_location("Unrelated"),
        );

        let mut stores = HashMap::new();
        stores.insert(5u8, store5);
        stores.insert(4u8, HashMap::new());
        data_loader::set_stores_for_testing(stores);

        let result = geocode_h3("853d838bfffffff", &test_options()).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_geocode_invalid_h3() {
        let _lock = lock();
        data_loader::clear_store_cache();

        let mut stores = HashMap::new();
        stores.insert(5u8, HashMap::new());
        stores.insert(4u8, HashMap::new());
        data_loader::set_stores_for_testing(stores);

        let result = geocode_h3("invalid_index", &test_options());
        assert!(result.is_err());
    }

    #[test]
    fn test_geocode_out_of_range_coordinate() {
        let _lock = lock();
        data_loader::clear_store_cache();

        let mut stores = HashMap::new();
        stores.insert(5u8, HashMap::new());
        stores.insert(4u8, HashMap::new());
        data_loader::set_stores_for_testing(stores);

        let result = geocode(91.0, 181.0, &test_options());
        assert!(result.is_err());
    }

    #[test]
    fn test_geocode_nan_coordinate() {
        let _lock = lock();
        data_loader::clear_store_cache();

        let mut stores = HashMap::new();
        stores.insert(5u8, HashMap::new());
        stores.insert(4u8, HashMap::new());
        data_loader::set_stores_for_testing(stores);

        let result = geocode(f64::NAN, 77.0, &test_options());
        assert!(result.is_err());
    }
}
