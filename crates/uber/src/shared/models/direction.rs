//! Directions API types and polyline decoding utilities.

use serde::{Deserialize, Serialize};

use super::location::LatLng;

/// Route details returned by the Directions API.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectionDetails {
    /// Human-readable distance string (e.g. "12.5 km").
    pub distance_text: String,
    /// Distance in metres.
    pub distance_value_meters: u64,
    /// Human-readable duration string (e.g. "25 mins").
    pub duration_text: String,
    /// Duration in seconds.
    pub duration_value_seconds: u64,
    /// Google-encoded polyline representing the route geometry.
    pub encoded_polyline: String,
}

/// Decodes a [Google encoded polyline](https://developers.google.com/maps/documentation/utilities/polylinealgorithm)
/// string into a vector of [`LatLng`] points.
///
/// # Examples
///
/// ```
/// use shared::models::direction::decode_polyline;
///
/// let points = decode_polyline("_p~iF~ps|U_ulLnnqC_mqNvxq`@");
/// assert_eq!(points.len(), 3);
/// ```
pub fn decode_polyline(encoded: &str) -> Vec<LatLng> {
    let mut points = Vec::new();
    let mut lat: i64 = 0;
    let mut lng: i64 = 0;
    let bytes = encoded.as_bytes();
    let len = bytes.len();
    let mut index = 0;

    while index < len {
        // Decode latitude delta
        let mut shift = 0u32;
        let mut result: i64 = 0;
        loop {
            let byte = (bytes[index] as i64) - 63;
            index += 1;
            result |= (byte & 0x1F) << shift;
            shift += 5;
            if byte < 0x20 {
                break;
            }
        }
        let d_lat = if (result & 1) != 0 {
            !(result >> 1)
        } else {
            result >> 1
        };
        lat += d_lat;

        // Decode longitude delta
        shift = 0;
        result = 0;
        loop {
            let byte = (bytes[index] as i64) - 63;
            index += 1;
            result |= (byte & 0x1F) << shift;
            shift += 5;
            if byte < 0x20 {
                break;
            }
        }
        let d_lng = if (result & 1) != 0 {
            !(result >> 1)
        } else {
            result >> 1
        };
        lng += d_lng;

        points.push(LatLng {
            lat: lat as f64 / 1e5,
            lng: lng as f64 / 1e5,
        });
    }

    points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_known_polyline() {
        // Example from Google's documentation
        let points = decode_polyline("_p~iF~ps|U_ulLnnqC_mqNvxq`@");
        assert_eq!(points.len(), 3);

        let eps = 1e-5;
        assert!((points[0].lat - 38.5).abs() < eps);
        assert!((points[0].lng - (-120.2)).abs() < eps);

        assert!((points[1].lat - 40.7).abs() < eps);
        assert!((points[1].lng - (-120.95)).abs() < eps);

        assert!((points[2].lat - 43.252).abs() < eps);
        assert!((points[2].lng - (-126.453)).abs() < eps);
    }

    #[test]
    fn decode_empty_polyline() {
        let points = decode_polyline("");
        assert!(points.is_empty());
    }
}
