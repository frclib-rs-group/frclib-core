//! A custom unit system for frclib rust.
//! 
//! This system is user extensible and defines many common units for use in FRC.
//! The system is designed in which library/vendor functions will take `impl #FAMILY_TYPE` as their argument type,
//! then store it as SI units in their internal strcture. This allows for the user to use any unit type they want,
//! and the library will handle the conversion to SI units.
//! 
//! Due to the use of `impl #FAMILY_TYPE` as the argument type any const functions must be implemented on the unit type itself.
//! It is highly recommended to use the SI unit as the base unit for the type.
//! 
//! # Example
//! ```
//! use frclib_core::units::time::{Second, Time};
//! 
//! #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
//! pub enum DebounceType {
//!     Rising,
//!     Falling,
//!     Both,
//! }
//! 
//! #[derive(Debug, Clone, Copy)]
//! pub struct Debouncer {
//!     debounce_time: Second,
//!     previous_time: Second,
//!     debounce_type: DebounceType,
//!     base_value: bool,
//! }
//! 
//! impl Debouncer {
//!    #[must_use]
//!   pub fn new(debounce_time: impl Time, debounce_type: DebounceType, base_value: bool) -> Self {
//!         Self {
//!            debounce_time: debounce_time.into(),
//!            previous_time: Second::new(0.0),
//!            debounce_type,
//!            base_value,
//!         }
//!   }
//! }









/// Angle units
pub mod angle;
/// Angular acceleration units
pub mod angular_acceleration;
/// Angular velocity units
pub mod angular_velocity;
/// Data units
pub mod data;
/// Data rate units
pub mod data_rate;
/// Distance units
pub mod distance;
/// Energy units
pub mod energy;
/// Force units
pub mod linear_velocity;
/// Linear acceleration units
pub mod mass;
/// Linear velocity units
pub mod moment_of_inertia;
/// Mass units
pub mod temperature;
/// Moment of inertia units
pub mod time;
/// Temperature units
pub mod torque;

#[cfg(test)]
#[doc(hidden)]
mod test;
#[doc(hidden)]
mod macros;