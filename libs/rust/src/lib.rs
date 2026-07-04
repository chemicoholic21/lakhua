//! # Lakhua
//!
//! Fast, offline reverse geocoding for India using H3 spatial indexing.
//!
//! This crate provides reverse geocoding lookups for Indian locations using
//! pre-generated H3-indexed JSON datasets. It supports H3 resolutions 4
//! (state/region level) and 5 (city/district level) with automatic fallback
//! to coarser resolutions when no exact match is found.
//!
//! ## Quick Start
//!
//! ```no_run
//! use lakhua::geocode;
//!
//! let result = geocode(25.3510, 78.5610, &Default::default()).unwrap();
//! if let Some(location) = result {
//!     println!("{}, {}", location.location.city, location.location.state);
//! }
//! ```
//!
//! ## H3 Index Lookup
//!
//! ```no_run
//! use lakhua::geocode_h3;
//!
//! let result = geocode_h3("853d838bfffffff", &Default::default()).unwrap();
//! if let Some(location) = result {
//!     println!("{}, {}", location.location.city, location.location.state);
//! }
//! ```

pub mod config;
pub mod error;
pub mod types;

pub(crate) mod core;

pub use config::{DEFAULT_RESOLUTION, MAX_RESOLUTION, MIN_RESOLUTION, SUPPORTED_RESOLUTIONS};
pub use error::LakhuaError;
pub use types::{GeocodeOptions, GeocodeResult, LocationDetails};

/// Performs reverse geocoding for the given latitude and longitude.
///
/// Converts the coordinates to an H3 cell at the configured resolution
/// (default 5) and looks up the matching location in the dataset. If
/// `fallback` is enabled (default), it will try coarser resolutions
/// (down to resolution 4) when no exact match is found.
///
/// # Arguments
///
/// * `lat` - Latitude in degrees (-90.0 to 90.0)
/// * `lon` - Longitude in degrees (-180.0 to 180.0)
/// * `options` - Geocoding options (resolution, fallback, debug)
///
/// # Errors
///
/// Returns `LakhuaError::InvalidCoordinate` if coordinates are out of range
/// or non-finite.
///
/// # Examples
///
/// ```no_run
/// use lakhua::{geocode, GeocodeOptions};
///
/// let result = geocode(25.3510, 78.5610, &GeocodeOptions::default()).unwrap();
/// ```
pub fn geocode(
    lat: f64,
    lon: f64,
    options: &GeocodeOptions,
) -> Result<Option<GeocodeResult>, LakhuaError> {
    core::geocoder::geocode(lat, lon, options)
}

/// Performs reverse geocoding for the given H3 index string.
///
/// Looks up the H3 index in the dataset. If `fallback` is enabled and no
/// match is found at the index's resolution, it will try parent cells
/// at coarser resolutions down to resolution 4.
///
/// # Arguments
///
/// * `h3_index` - A valid H3 cell index string (e.g., "853d838bfffffff")
/// * `options` - Geocoding options (resolution, fallback, debug)
///
/// # Errors
///
/// Returns `LakhuaError::InvalidH3Index` if the H3 index is invalid.
///
/// # Examples
///
/// ```no_run
/// use lakhua::{geocode_h3, GeocodeOptions};
///
/// let result = geocode_h3("853d838bfffffff", &GeocodeOptions::default()).unwrap();
/// ```
pub fn geocode_h3(
    h3_index: &str,
    options: &GeocodeOptions,
) -> Result<Option<GeocodeResult>, LakhuaError> {
    core::geocoder::geocode_h3(h3_index, options)
}
