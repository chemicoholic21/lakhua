pub const MIN_RESOLUTION: u8 = 4;
pub const MAX_RESOLUTION: u8 = 5;
pub const DEFAULT_RESOLUTION: u8 = MAX_RESOLUTION;
pub const SUPPORTED_RESOLUTIONS: [u8; 2] = [4, 5];

pub const DATA_FILE_PREFIX: &str = "reverse_geo_";

pub fn clamp_resolution(resolution: u8) -> u8 {
    resolution.clamp(MIN_RESOLUTION, MAX_RESOLUTION)
}
