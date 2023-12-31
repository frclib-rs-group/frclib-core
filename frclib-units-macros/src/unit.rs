use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn unit(input: TokenStream) -> TokenStream {
    let mut output = TokenStream::new();
    //get an ident and a type from the token stream
    //filter out puncts and commas
    let mut iter = input.into_iter().filter(|token| {
        !matches!(
            token,
            proc_macro2::TokenTree::Punct(_) | proc_macro2::TokenTree::Group(_)
        )
    });
    let struct_name =
        syn::parse2::<syn::Ident>(iter.next().expect("could not find first ident").into())
            .expect("could not parse first ident as an ident");
    let r#type = syn::parse2::<syn::Ident>(iter.next().expect("could not find second type").into())
        .expect("could not parse second type");

    //create a new struct with the given name and type
    let struct_item = quote! {
        #[forbid(non_camel_case_types)]
        #[repr(transparent)]
        pub struct #struct_name(pub #r#type);
    };

    //impl clone, copy, debug and display for the struct
    let impl_basic_block = quote! {
        #[allow(clippy::expl_impl_clone_on_copy)]
        impl Clone for #struct_name {
            fn clone(&self) -> Self {
                *self
            }
        }
        #[allow(clippy::expl_impl_clone_on_copy)]
        impl Copy for #struct_name {}
        impl std::fmt::Debug for #struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}({})", stringify!(#struct_name), self.0)
            }
        }
        impl std::fmt::Display for #struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}({})", stringify!(#struct_name), self.0)
            }
        }
        impl std::hash::Hash for #struct_name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                std::hash::Hash::hash(&(self.0 as f64).to_bits(), state)
            }
        }
        impl #struct_name {
            #[inline(always)]
            pub const fn new(value: #r#type) -> Self {
                Self(value)
            }
            #[inline(always)]
            pub const fn value(&self) -> #r#type {
                self.0
            }
        }
    };

    //implement math operators for the struct
    let impl_math_block = quote! {
        impl std::ops::Add for #struct_name {
            type Output = Self;
            #[inline(always)]
            fn add(self, rhs: Self) -> Self::Output {
                Self(self.0 + rhs.0)
            }
        }
        #[cfg(feature = "units-ref-ops")]
        impl std::ops::Add<&#struct_name> for #struct_name {
            type Output = Self;
            #[inline(always)]
            fn add(self, rhs: &#struct_name) -> Self::Output {
                Self(self.0 + rhs.0)
            }
        }
        #[cfg(feature = "units-ref-ops")]
        impl std::ops::Add<#struct_name> for &#struct_name {
            type Output = #struct_name;
            #[inline(always)]
            fn add(self, rhs: #struct_name) -> Self::Output {
                #struct_name(self.0 + rhs.0)
            }
        }
        #[cfg(feature = "units-ref-ops")]
        impl std::ops::Add<&#struct_name> for &#struct_name {
            type Output = #struct_name;
            #[inline(always)]
            fn add(self, rhs: &#struct_name) -> Self::Output {
                #struct_name(self.0 + rhs.0)
            }
        }
        impl std::ops::AddAssign for #struct_name {
            #[inline(always)]
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0;
            }
        }
        impl std::ops::Sub for #struct_name {
            type Output = Self;
            #[inline(always)]
            fn sub(self, rhs: Self) -> Self::Output {
                Self(self.0 - rhs.0)
            }
        }
        #[cfg(feature = "units-ref-ops")]
        impl std::ops::Sub<&#struct_name> for #struct_name {
            type Output = Self;
            #[inline(always)]
            fn sub(self, rhs: &#struct_name) -> Self::Output {
                Self(self.0 - rhs.0)
            }
        }
        #[cfg(feature = "units-ref-ops")]
        impl std::ops::Sub<#struct_name> for &#struct_name {
            type Output = #struct_name;
            #[inline(always)]
            fn sub(self, rhs: #struct_name) -> Self::Output {
                #struct_name(self.0 - rhs.0)
            }
        }
        #[cfg(feature = "units-ref-ops")]
        impl std::ops::Sub<&#struct_name> for &#struct_name {
            type Output = #struct_name;
            #[inline(always)]
            fn sub(self, rhs: &#struct_name) -> Self::Output {
                #struct_name(self.0 - rhs.0)
            }
        }
        impl std::ops::SubAssign for #struct_name {
            #[inline(always)]
            fn sub_assign(&mut self, rhs: Self) {
                self.0 -= rhs.0;
            }
        }
        impl std::ops::Mul for #struct_name {
            type Output = Self;
            #[inline(always)]
            fn mul(self, rhs: Self) -> Self::Output {
                Self(self.0 * rhs.0)
            }
        }
        #[cfg(feature = "units-ref-ops")]
        impl std::ops::Mul<&#struct_name> for #struct_name {
            type Output = Self;
            #[inline(always)]
            fn mul(self, rhs: &#struct_name) -> Self::Output {
                Self(self.0 * rhs.0)
            }
        }
        #[cfg(feature = "units-ref-ops")]
        impl std::ops::Mul<#struct_name> for &#struct_name {
            type Output = #struct_name;
            #[inline(always)]
            fn mul(self, rhs: #struct_name) -> Self::Output {
                #struct_name(self.0 * rhs.0)
            }
        }
        #[cfg(feature = "units-ref-ops")]
        impl std::ops::Mul<&#struct_name> for &#struct_name {
            type Output = #struct_name;
            #[inline(always)]
            fn mul(self, rhs: &#struct_name) -> Self::Output {
                #struct_name(self.0 * rhs.0)
            }
        }
        impl std::ops::MulAssign for #struct_name {
            #[inline(always)]
            fn mul_assign(&mut self, rhs: Self) {
                self.0 *= rhs.0;
            }
        }
        impl std::ops::Div for #struct_name {
            type Output = Self;
            #[inline(always)]
            fn div(self, rhs: Self) -> Self::Output {
                Self(self.0 / rhs.0)
            }
        }
        #[cfg(feature = "units-ref-ops")]
        impl std::ops::Div<&#struct_name> for #struct_name {
            type Output = Self;
            #[inline(always)]
            fn div(self, rhs: &#struct_name) -> Self::Output {
                Self(self.0 / rhs.0)
            }
        }
        #[cfg(feature = "units-ref-ops")]
        impl std::ops::Div<#struct_name> for &#struct_name {
            type Output = #struct_name;
            #[inline(always)]
            fn div(self, rhs: #struct_name) -> Self::Output {
                #struct_name(self.0 / rhs.0)
            }
        }
        #[cfg(feature = "units-ref-ops")]
        impl std::ops::Div<&#struct_name> for &#struct_name {
            type Output = #struct_name;
            #[inline(always)]
            fn div(self, rhs: &#struct_name) -> Self::Output {
                #struct_name(self.0 / rhs.0)
            }
        }
        impl std::ops::DivAssign for #struct_name {
            #[inline(always)]
            fn div_assign(&mut self, rhs: Self) {
                self.0 /= rhs.0;
            }
        }
        impl std::ops::Rem for #struct_name {
            type Output = Self;
            #[inline(always)]
            fn rem(self, rhs: Self) -> Self::Output {
                Self(self.0 % rhs.0)
            }
        }
        #[cfg(feature = "units-ref-ops")]
        impl std::ops::Rem<&#struct_name> for #struct_name {
            type Output = Self;
            #[inline(always)]
            fn rem(self, rhs: &#struct_name) -> Self::Output {
                Self(self.0 % rhs.0)
            }
        }
        #[cfg(feature = "units-ref-ops")]
        impl std::ops::Rem<#struct_name> for &#struct_name {
            type Output = #struct_name;
            #[inline(always)]
            fn rem(self, rhs: #struct_name) -> Self::Output {
                #struct_name(self.0 % rhs.0)
            }
        }
        #[cfg(feature = "units-ref-ops")]
        impl std::ops::Rem<&#struct_name> for &#struct_name {
            type Output = #struct_name;
            #[inline(always)]
            fn rem(self, rhs: &#struct_name) -> Self::Output {
                #struct_name(self.0 % rhs.0)
            }
        }
        impl std::ops::RemAssign for #struct_name {
            #[inline(always)]
            fn rem_assign(&mut self, rhs: Self) {
                self.0 %= rhs.0;
            }
        }
        impl #struct_name {
            #[inline(always)]
            pub fn square(&self) -> Self {
                Self(self.0 * self.0)
            }
            #[inline(always)]
            pub fn cube(&self) -> Self {
                Self(self.0 * self.0 * self.0)
            }
            #[inline(always)]
            pub fn map(&self, f: impl FnOnce(#r#type) -> #r#type) -> Self {
                Self(f(self.0))
            }
        }
    };

    //implement num traits for the struct
    let impl_num_traits_block = quote! {
        impl num::Zero for #struct_name {
            fn zero() -> Self {
                Self(#r#type::zero())
            }
            fn is_zero(&self) -> bool {
                self.0.is_zero()
            }
        }
        impl num::One for #struct_name {
            fn one() -> Self {
                Self(#r#type::one())
            }
            fn is_one(&self) -> bool {
                self.0.is_one()
            }
        }
        impl num::Num for #struct_name {
            type FromStrRadixErr = <#r#type as num::Num>::FromStrRadixErr;
            fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
                Ok(Self(#r#type::from_str_radix(str, radix)?))
            }
        }
        impl num::ToPrimitive for #struct_name {
            fn to_i64(&self) -> Option<i64> {
                self.0.to_i64()
            }
            fn to_u64(&self) -> Option<u64> {
                self.0.to_u64()
            }
        }
        impl num::FromPrimitive for #struct_name {
            fn from_i64(n: i64) -> Option<Self> {
                Some(Self(#r#type::from_i64(n)?))
            }
            fn from_u64(n: u64) -> Option<Self> {
                Some(Self(#r#type::from_u64(n)?))
            }
            fn from_f64(n: f64) -> Option<Self> {
                Some(Self(#r#type::from_f64(n)?))
            }
        }
    };

    //implement into and from for its type
    let impl_into_from_block = quote! {
        impl From<#struct_name> for #r#type {
            #[inline(always)]
            fn from(value: #struct_name) -> #r#type {
                value.0
            }
        }
        #[cfg(feature = "units-ref-ops")]
        impl From<&#struct_name> for #r#type {
            #[inline(always)]
            fn from(value: &#struct_name) -> #r#type {
                value.0
            }
        }
        impl From<f64> for #struct_name {
            #[inline(always)]
            fn from(value: f64) -> Self {
                Self(value as #r#type)
            }
        }
        #[cfg(feature = "units-ref-ops")]
        impl From<&f64> for #struct_name {
            #[inline(always)]
            fn from(value: &f64) -> Self {
                Self(*value as #r#type)
            }
        }
        impl From<f32> for #struct_name {
            #[inline(always)]
            fn from(value: f32) -> Self {
                Self(value as #r#type)
            }
        }
        #[cfg(feature = "units-ref-ops")]
        impl From<&f32> for #struct_name {
            #[inline(always)]
            fn from(value: &f32) -> Self {
                Self(*value as #r#type)
            }
        }
        impl From<u64> for #struct_name {
            #[inline(always)]
            fn from(value: u64) -> Self {
                Self(value as #r#type)
            }
        }
        #[cfg(feature = "units-ref-ops")]
        impl From<&u64> for #struct_name {
            #[inline(always)]
            fn from(value: &u64) -> Self {
                Self(*value as #r#type)
            }
        }
        impl From<u32> for #struct_name {
            #[inline(always)]
            fn from(value: u32) -> Self {
                Self(value as #r#type)
            }
        }
        #[cfg(feature = "units-ref-ops")]
        impl From<&u32> for #struct_name {
            #[inline(always)]
            fn from(value: &u32) -> Self {
                Self(*value as #r#type)
            }
        }
        impl From<u16> for #struct_name {
            #[inline(always)]
            fn from(value: u16) -> Self {
                Self(value as #r#type)
            }
        }
        #[cfg(feature = "units-ref-ops")]
        impl From<&u16> for #struct_name {
            #[inline(always)]
            fn from(value: &u16) -> Self {
                Self(*value as #r#type)
            }
        }
        impl From<u8> for #struct_name {
            #[inline(always)]
            fn from(value: u8) -> Self {
                Self(value as #r#type)
            }
        }
        #[cfg(feature = "units-ref-ops")]
        impl From<&u8> for #struct_name {
            #[inline(always)]
            fn from(value: &u8) -> Self {
                Self(*value as #r#type)
            }
        }
        impl From<i64> for #struct_name {
            #[inline(always)]
            fn from(value: i64) -> Self {
                Self(value as #r#type)
            }
        }
        #[cfg(feature = "units-ref-ops")]
        impl From<&i64> for #struct_name {
            #[inline(always)]
            fn from(value: &i64) -> Self {
                Self(*value as #r#type)
            }
        }
        impl From<i32> for #struct_name {
            #[inline(always)]
            fn from(value: i32) -> Self {
                Self(value as #r#type)
            }
        }
        #[cfg(feature = "units-ref-ops")]
        impl From<&i32> for #struct_name {
            #[inline(always)]
            fn from(value: &i32) -> Self {
                Self(*value as #r#type)
            }
        }
        impl From<i16> for #struct_name {
            #[inline(always)]
            fn from(value: i16) -> Self {
                Self(value as #r#type)
            }
        }
        #[cfg(feature = "units-ref-ops")]
        impl From<&i16> for #struct_name {
            #[inline(always)]
            fn from(value: &i16) -> Self {
                Self(*value as #r#type)
            }
        }
        impl From<i8> for #struct_name {
            #[inline(always)]
            fn from(value: i8) -> Self {
                Self(value as #r#type)
            }
        }
        #[cfg(feature = "units-ref-ops")]
        impl From<&i8> for #struct_name {
            #[inline(always)]
            fn from(value: &i8) -> Self {
                Self(*value as #r#type)
            }
        }
    };

    //implement serde for the struct
    let impl_serde_block = quote! {
        #[cfg(feature = "units-extra")]
        impl serde::Serialize for #struct_name {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                self.0.serialize(serializer)
            }
        }
        #[cfg(feature = "units-extra")]
        impl<'de> serde::Deserialize<'de> for #struct_name {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                #r#type::deserialize(deserializer).map(|value| Self(value))
            }
        }
    };

    //implement partial eq and partial ord for the struct
    let impl_partial_eq_block = quote! {
        impl std::cmp::PartialEq for #struct_name {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }
        impl std::cmp::PartialOrd for #struct_name {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                self.0.partial_cmp(&other.0)
            }
        }
    };

    let impl_negative_block = quote! {
        impl std::ops::Neg for #struct_name {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self(-self.0)
            }
        }
    };

    let impl_simd_block = quote! {
        #[cfg(feature = "units-extra")]
        impl nalgebra::SimdValue for #struct_name {
            type Element = #struct_name;
            type SimdBool = bool;

            #[inline]
            fn lanes() -> usize {
                1
            }
            #[inline]
            fn splat(val: Self::Element) -> Self {
                val
            }
            #[inline]
            fn extract(&self, _: usize) -> Self::Element {
                *self
            }
            #[inline]
            unsafe fn extract_unchecked(&self, _: usize) -> Self::Element {
                *self
            }
            #[inline]
            fn replace(&mut self, _: usize, val: Self::Element) {
                self.0 = val.0
            }
            #[inline]
            unsafe fn replace_unchecked(&mut self, _: usize, val: Self::Element) {
                self.0 = val.0
            }
            #[inline]
            fn select(self, cond: Self::SimdBool, other: Self) -> Self {
                if cond {
                    self
                } else {
                    other
                }
            }
            #[inline]
            fn map_lanes(self, f: impl Fn(Self::Element) -> Self::Element) -> Self
                where
                    Self: Clone, {
                f(self)
            }
            #[inline]
            fn zip_map_lanes(
                    self,
                    b: Self,
                    f: impl Fn(Self::Element, Self::Element) -> Self::Element,
                ) -> Self
                where
                    Self: Clone, {
                f(self, b)
            }
        }
        #[cfg(feature = "units-extra")]
        impl nalgebra::Field for #struct_name {}
        #[cfg(feature = "units-extra")]
        impl simba::scalar::SubsetOf<#struct_name> for #struct_name {
            #[inline]
            fn is_in_subset(_element: &Self) -> bool {true}
            fn to_superset(&self) -> #struct_name {*self}
            fn from_superset(element: &#struct_name) -> Option<Self> {Some(*element)}
            fn from_superset_unchecked(element: &#struct_name) -> Self {*element}
        }
        #[cfg(feature = "units-extra")]
        impl simba::scalar::SubsetOf<#struct_name> for f64 {
            #[inline]
            fn is_in_subset(_element: &#struct_name) -> bool {true}
            fn to_superset(&self) -> #struct_name {#struct_name::new(*self)}
            fn from_superset(element: &#struct_name) -> Option<Self> {Some(element.0 as f64)}
            fn from_superset_unchecked(element: &#struct_name) -> Self {element.0 as f64}
        }
        #[cfg(feature = "units-extra")]
        impl nalgebra::ComplexField for #struct_name {
            type RealField = #r#type;
            #[inline]
            fn is_finite(&self) -> bool {self.0.is_finite()}
            #[inline]
            fn try_sqrt(self) -> Option<Self> {Some(#struct_name::new(self.0.sqrt()))}
            #[inline]
            fn abs(self) -> Self::RealField {
                nalgebra::ComplexField::abs(#r#type::from(self.0))
            }
            #[inline]
            fn acos(self) -> Self {
                #struct_name::new(nalgebra::ComplexField::acos(#r#type::from(self.0)))
            }
            #[inline]
            fn acosh(self) -> Self {
                #struct_name::new(nalgebra::ComplexField::acosh(#r#type::from(self.0)))
            }
            #[inline]
            fn asin(self) -> Self {
                #struct_name::new(nalgebra::ComplexField::asin(#r#type::from(self.0)))
            }
            #[inline]
            fn asinh(self) -> Self {
                #struct_name::new(nalgebra::ComplexField::asinh(#r#type::from(self.0)))
            }
            #[inline]
            fn atan(self) -> Self {
                #struct_name::new(nalgebra::ComplexField::atan(#r#type::from(self.0)))
            }
            #[inline]
            fn atanh(self) -> Self {
                #struct_name::new(nalgebra::ComplexField::atanh(#r#type::from(self.0)))
            }
            #[inline]
            fn cos(self) -> Self {
                #struct_name::new(nalgebra::ComplexField::cos(#r#type::from(self.0)))
            }
            #[inline]
            fn cosh(self) -> Self {
                #struct_name::new(nalgebra::ComplexField::cosh(#r#type::from(self.0)))
            }
            #[inline]
            fn exp(self) -> Self {
                #struct_name::new(nalgebra::ComplexField::exp(#r#type::from(self.0)))
            }
            #[inline]
            fn ln(self) -> Self {
                #struct_name::new(nalgebra::ComplexField::ln(#r#type::from(self.0)))
            }
            #[inline]
            fn log(self, base: #r#type) -> Self {
                #struct_name::new(nalgebra::ComplexField::log(#r#type::from(self.0), base))
            }
            #[inline]
            fn powf(self, n: Self::RealField) -> Self {
                #struct_name::new(nalgebra::ComplexField::powf(#r#type::from(self.0), n))
            }
            #[inline]
            fn powi(self, n: i32) -> Self {
                #struct_name::new(nalgebra::ComplexField::powi(#r#type::from(self.0), n))
            }
            #[inline]
            fn recip(self) -> Self {
                #struct_name::new(nalgebra::ComplexField::recip(#r#type::from(self.0)))
            }
            #[inline]
            fn sin(self) -> Self {
                #struct_name::new(nalgebra::ComplexField::sin(#r#type::from(self.0)))
            }
            #[inline]
            fn sinh(self) -> Self {
                #struct_name::new(nalgebra::ComplexField::sinh(#r#type::from(self.0)))
            }
            #[inline]
            fn sqrt(self) -> Self {
                #struct_name::new(nalgebra::ComplexField::sqrt(#r#type::from(self.0)))
            }
            #[inline]
            fn tan(self) -> Self {
                #struct_name::new(nalgebra::ComplexField::tan(#r#type::from(self.0)))
            }
            #[inline]
            fn tanh(self) -> Self {
                #struct_name::new(nalgebra::ComplexField::tanh(#r#type::from(self.0)))
            }
            #[inline]
            fn argument(self) -> Self::RealField {
                nalgebra::ComplexField::argument(#r#type::from(self.0))
            }
            #[inline]
            fn modulus(self) -> Self::RealField {
                nalgebra::ComplexField::modulus(#r#type::from(self.0))
            }
            #[inline]
            fn to_exp(self) -> (Self::RealField, Self) {
                let (r, theta) = nalgebra::ComplexField::to_exp(#r#type::from(self.0));
                (r, #struct_name::new(theta))
            }
            #[inline]
            fn cbrt(self) -> Self {
                #struct_name::new(nalgebra::ComplexField::cbrt(#r#type::from(self.0)))
            }
            #[inline]
            fn hypot(self, other: Self) -> Self::RealField {
                nalgebra::ComplexField::hypot(#r#type::from(self.0), #r#type::from(other.0))
            }
            #[inline]
            fn ceil(self) -> Self {
                #struct_name::new(nalgebra::ComplexField::ceil(#r#type::from(self.0)))
            }
            #[inline]
            fn floor(self) -> Self {
                #struct_name::new(nalgebra::ComplexField::floor(#r#type::from(self.0)))
            }
            #[inline]
            fn round(self) -> Self {
                #struct_name::new(nalgebra::ComplexField::round(#r#type::from(self.0)))
            }
            #[inline]
            fn trunc(self) -> Self {
                #struct_name::new(nalgebra::ComplexField::trunc(#r#type::from(self.0)))
            }
            #[inline]
            fn conjugate(self) -> Self {
                #struct_name::new(nalgebra::ComplexField::conjugate(#r#type::from(self.0)))
            }
            #[inline]
            fn cosc(self) -> Self {
                #struct_name::new(nalgebra::ComplexField::cosc(#r#type::from(self.0)))
            }
            #[inline]
            fn sinhc(self) -> Self {
                #struct_name::new(nalgebra::ComplexField::sinhc(#r#type::from(self.0)))
            }
            #[inline]
            fn signum(self) -> Self {
                #struct_name::new(nalgebra::ComplexField::signum(#r#type::from(self.0)))
            }
            #[inline]
            fn coshc(self) -> Self {
                #struct_name::new(nalgebra::ComplexField::coshc(#r#type::from(self.0)))
            }
            #[inline]
            fn exp2(self) -> Self {
                #struct_name::new(nalgebra::ComplexField::exp2(#r#type::from(self.0)))
            }
            #[inline]
            fn exp_m1(self) -> Self {
                #struct_name::new(nalgebra::ComplexField::exp_m1(#r#type::from(self.0)))
            }
            #[inline]
            fn ln_1p(self) -> Self {
                #struct_name::new(nalgebra::ComplexField::ln_1p(#r#type::from(self.0)))
            }
            #[inline]
            fn log10(self) -> Self {
                #struct_name::new(nalgebra::ComplexField::log10(#r#type::from(self.0)))
            }
            #[inline]
            fn fract(self) -> Self {
                #struct_name::new(nalgebra::ComplexField::fract(#r#type::from(self.0)))
            }
            #[inline]
            fn from_real(re: Self::RealField) -> Self {
                #struct_name::new(nalgebra::ComplexField::from_real(re))
            }
            #[inline]
            fn imaginary(self) -> Self::RealField {
                nalgebra::ComplexField::imaginary(#r#type::from(self.0))
            }
            #[inline]
            fn log2(self) -> Self {
                #struct_name::new(nalgebra::ComplexField::log2(#r#type::from(self.0)))
            }
            #[inline]
            fn modulus_squared(self) -> Self::RealField {
                nalgebra::ComplexField::modulus_squared(#r#type::from(self.0))
            }
            #[inline]
            fn mul_add(self,a:Self,b:Self) -> Self {
                #struct_name::new(nalgebra::ComplexField::mul_add(#r#type::from(self.0),#r#type::from(a.0),#r#type::from(b.0)))
            }
            #[inline]
            fn norm1(self) -> Self::RealField {
                nalgebra::ComplexField::norm1(#r#type::from(self.0))
            }
            #[inline]
            fn powc(self,n:Self) -> Self {
                #struct_name::new(nalgebra::ComplexField::powc(#r#type::from(self.0),#r#type::from(n.0)))
            }
            #[inline]
            fn real(self) -> Self::RealField {
                nalgebra::ComplexField::real(#r#type::from(self.0))
            }
            #[inline]
            fn scale(self,factor:Self::RealField) -> Self {
                #struct_name::new(nalgebra::ComplexField::scale(#r#type::from(self.0),factor))
            }
            #[inline]
            fn sin_cos(self) -> (Self,Self) {
                let (s,c) = nalgebra::ComplexField::sin_cos(#r#type::from(self.0));
                (#struct_name::new(s),#struct_name::new(c))
            }
            #[inline]
            fn sinc(self) -> Self {
                #struct_name::new(nalgebra::ComplexField::sinc(#r#type::from(self.0)))
            }
            #[inline]
            fn sinh_cosh(self) -> (Self,Self) {
                let (s,c) = nalgebra::ComplexField::sinh_cosh(#r#type::from(self.0));
                (#struct_name::new(s),#struct_name::new(c))
            }
            #[inline]
            fn to_polar(self) -> (Self::RealField,Self::RealField) {
                let (r,theta) = nalgebra::ComplexField::to_polar(#r#type::from(self.0));
                (r,theta)
            }
            #[inline]
            fn unscale(self,factor:Self::RealField) -> Self {
                #struct_name::new(nalgebra::ComplexField::unscale(#r#type::from(self.0),factor))
            }
        }
    };

    let type_str = r#type.to_string();

    output.extend(struct_item);
    output.extend(impl_basic_block);
    output.extend(impl_math_block);
    output.extend(impl_num_traits_block);
    output.extend(impl_into_from_block);
    output.extend(impl_serde_block);
    output.extend(impl_partial_eq_block);

    if !type_str.contains('u') {
        output.extend(impl_negative_block);
    }
    if type_str.contains("f64") || type_str.contains("f32") {
        output.extend(impl_simd_block);
    }

    output
}