//! # frclib-core
//! 
//! This crate contains the core functionality for the rust FRC ecosystem.
//! Whenever a crate that `frclib` depends on needs to use an frc feature it will use this crate to prevent circular dependencies.
//! 
//! This library goes as far as it can to be panic free,
//! the only function that can panic is [HAL Initialization](crate::hal::HAL).
//! 
// ## Modules
// 
// ### [Value](crate::value)
// 
// This module contains the [``FrcValue``](crate::value::FrcValue) type which is used to represent values in various frc protocols.
// Variants:
// - [Void](crate::value::FrcValue::Void)
// - [Boolean](crate::value::FrcValue::Boolean)
// - [Float](crate::value::FrcValue::Float)
// - [Double](crate::value::FrcValue::Double)
// - [String](crate::value::FrcValue::String)
// - [Boolean Array](crate::value::FrcValue::BooleanArray)
// - [Float Array](crate::value::FrcValue::FloatArray)
// - [Double Array](crate::value::FrcValue::DoubleArray)
// - [String Array](crate::value::FrcValue::StringArray)
// - [Raw](crate::value::FrcValue::Raw)
// - [Struct](crate::value::FrcValue::Struct)
// 
// ### [Units](crate::units)
// 
// This module contains a variety of units and unit families with macros for user defined units.
// Units:
// - [Time](crate::units::time)
// - [Angle](crate::units::angle)
// - [Angular Velocity](crate::units::angular_velocity)
// - [Angular Acceleration](crate::units::angular_acceleration)
// - [Distance](crate::units::distance)
// - [Linear Velocity](crate::units::linear_velocity)
// - [Mass](crate::units::mass)
// - [Temperature](crate::units::temperature)
// - [Energy](crate::units::energy)
// - [Moment of Inertia](crate::units::moment_of_inertia)
// - [Torque](crate::units::torque)
// - [Data](crate::units::data)
// - [Data Rate](crate::units::data_rate)
// 

#![deny(clippy::all, clippy::cargo, clippy::pedantic, clippy::nursery)]
#![deny(
    missing_copy_implementations,
    single_use_lifetimes,
    variant_size_differences,
    arithmetic_overflow,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_results,
    unused_lifetimes,
    unused_unsafe,
    unused_tuple_struct_fields,
    useless_ptr_null_checks,
    cenum_impl_drop_cast,
    while_true,
    unused_features,
    absolute_paths_not_starting_with_crate,
    unused_allocation,
    unreachable_code,
    unused_comparisons,
    unused_parens,
    asm_sub_register,
    break_with_label_and_loop,
    bindings_with_variant_name,
    anonymous_parameters,
    clippy::unwrap_used,
    clippy::panicking_unwrap,
    missing_abi,
    missing_fragment_specifier,
    clippy::missing_safety_doc,
    clippy::missing_asserts_for_indexing,
    clippy::missing_assert_message,
    clippy::possible_missing_comma
)]
#![allow(clippy::module_name_repetitions, clippy::option_if_let_else)]
#![cfg_attr(
    not(test),
    forbid(
        clippy::panic,
        clippy::todo,
        clippy::unimplemented,
        clippy::expect_used
    )
)]
#![cfg_attr(
    not(test),
    warn(
        missing_docs
    )
)]

#[cfg(feature = "value-union")]
pub mod value;
#[cfg(feature = "units")]
pub mod units;
#[cfg(feature = "time")]
pub mod time;
#[cfg(feature = "structure")]
pub mod structure;
#[cfg(feature = "hal")]
pub mod hal;