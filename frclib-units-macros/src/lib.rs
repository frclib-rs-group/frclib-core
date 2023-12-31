use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

mod unit;
mod conversion;
mod dimensions;
mod family;

/// Defines a unit as a transparent newtype around the supplied primitive type.
/// Implements various conversion traits for the unit and some mathematical operations.
/// Depending on features will also add nalgebra complex field and serde support.
#[proc_macro]
pub fn unit(input: TokenStream) -> TokenStream {
    unit::unit(TokenStream2::from(input)).into()
}

/// Implements traits to convert between units using the supplied function.
/// The unit must be defined with the `unit` macro.
#[proc_macro]
pub fn unit_conversion(input: TokenStream) -> TokenStream {
    conversion::conversion(TokenStream2::from(input)).into()
}

/// Implements math operations for the unit to allow dimensional analysis.
/// If division is used in macro will only implement division while multiplication will implement both.
#[proc_macro]
pub fn unit_dimensional_analysis(input: TokenStream) -> TokenStream {
    dimensions::dimensions(TokenStream2::from(input)).into()
}

/// Defines a trait for a family of units.
/// Every unit in the family must be defined with the `unit` macro and be convertible to each other.
#[proc_macro]
pub fn unit_family(input: TokenStream) -> TokenStream {
    family::family(TokenStream2::from(input)).into()
}