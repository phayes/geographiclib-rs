//! A subset of [geographiclib](https://geographiclib.sourceforge.io/) implemented in Rust.
//!
//! # Examples
//!
//! ```rust
//! // Determine the point 10000 km NE of JFK - the "direct" geodesic calculation.
//! use geographiclib_rs::{Geodesic, DirectGeodesic};
//!
//! let g = Geodesic::wgs84();
//! let jfk_lat = 40.64;
//! let jfk_lon = -73.78;
//! let northeast_azimuth = 45.0;
//!
//! let (lat, lon, az) = g.direct(jfk_lat, jfk_lon, northeast_azimuth, 10e6);
//!
//! use approx::assert_relative_eq;
//! assert_relative_eq!(lat, 32.621100463725796);
//! assert_relative_eq!(lon, 49.052487092959836);
//! assert_relative_eq!(az,  140.4059858768007);
//! ```
//!
//! ```rust
//! // Determine the distance between two points - the "inverse" geodesic calculation.
//! use geographiclib_rs::{Geodesic, InverseGeodesic};
//!
//! let g = Geodesic::wgs84();
//! let p1 = (34.095925, -118.2884237);
//! let p2 = (59.4323439, 24.7341649);
//! let s12: f64 = g.inverse(p1.0, p1.1, p2.0, p2.1);
//!
//! use approx::assert_relative_eq;
//! assert_relative_eq!(s12, 9094718.72751138);
//! ```
//!
//! ```rust
//! // Determine the perimeter and area of a polygon.
//! use geographiclib_rs::{Geodesic, PolygonArea, Winding};
//!
//! let g = Geodesic::wgs84();
//! let mut pa = PolygonArea::new(&g, Winding::CounterClockwise);
//! pa.add_point(0.0, 0.0);
//! pa.add_point(0.0, 1.0);
//! pa.add_point(1.0, 1.0);
//! pa.add_point(1.0, 0.0);
//!
//! let (perimeter_m, area_m_squared, num_points) = pa.compute(false);
//!
//! use approx::assert_relative_eq;
//! assert_relative_eq!(perimeter_m, 443770.91724830196);
//! assert_relative_eq!(area_m_squared, 12308778361.469452);
//! assert_eq!(num_points, 4);
//! ```
//!
//! ```rust
//! // Determine the distance between rovers Pathfinder and Curiosity on Mars
//! use geographiclib_rs::{Geodesic, InverseGeodesic};
//!
//! let mars = Geodesic::new(3396190.0, 1.0 / 169.8944472);
//! let pathfinder = (19.26, 326.75);
//! let curiosity = (-4.765700445, 137.39820983);
//! let distance_m: f64 = mars.inverse(curiosity.0, curiosity.1, pathfinder.0, pathfinder.1);
//!
//! assert_eq!(distance_m.round(), 9639113.0);
//! ```
//!
//! # Features
//!
//! 1. `accurate`: Enabled by default. Use the [`accurate`](https://docs.rs/accurate/latest/accurate/) crate to provide high accuracy polygon areas and perimeters in `PolygonArea`. Can be disabled for better performance or when `PolygonArea` is not being used.

mod geodesic;
pub use geodesic::{DirectGeodesic, Geodesic, InverseGeodesic};

pub mod geodesiccapability;
pub use geodesiccapability as capability;

mod geodesicline;
mod geomath;
mod polygonarea;
pub use polygonarea::PolygonArea;
pub use polygonarea::Winding;

#[macro_use]
extern crate lazy_static;
