

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
    clippy::panicking_unwrap
)]
#![allow(clippy::module_name_repetitions)]

//The end goal is to have no panics in the library
// #![cfg_attr(
//     not(test),
//     forbid(
//         clippy::panic,
//         clippy::todo,
//         clippy::unimplemented,
//         clippy::expect_used
//     )
// )]

#[cfg(feature = "value-union")]
pub mod value;
#[cfg(feature = "units")]
pub mod units;
#[cfg(feature = "time")]
pub mod time;
#[cfg(feature = "logging")]
pub mod logging;
#[cfg(feature = "structure")]
pub mod structure;
#[cfg(feature = "hal")]
pub mod hal;