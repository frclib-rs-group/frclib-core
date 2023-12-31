

macro_rules! unit_nalgebra {
    ($unit_name:ident) => {
        impl nalgebra::SimdValue for $unit_name {
            type Element = $unit_name;
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
        impl nalgebra::Field for $unit_name {}
        impl simba::scalar::SubsetOf<$unit_name> for $unit_name {
            #[inline]
            fn is_in_subset(_element: &Self) -> bool {true}
            fn to_superset(&self) -> $unit_name {*self}
            fn from_superset(element: &$unit_name) -> Option<Self> {Some(*element)}
            fn from_superset_unchecked(element: &$unit_name) -> Self {*element}
        }
        impl simba::scalar::SubsetOf<$unit_name> for f64 {
            #[inline]
            fn is_in_subset(_element: &$unit_name) -> bool {true}
            fn to_superset(&self) -> $unit_name {$unit_name::new(*self)}
            fn from_superset(element: &$unit_name) -> Option<Self> {Some(element.0)}
            fn from_superset_unchecked(element: &$unit_name) -> Self {element.0}
        }
        impl nalgebra::ComplexField for $unit_name {
            type RealField = f64;
            #[inline]
            fn is_finite(&self) -> bool {self.0.is_finite()}
            #[inline]
            fn try_sqrt(self) -> Option<Self> {Some($unit_name::new(self.0.sqrt()))}
            #[inline]
            fn abs(self) -> Self::RealField {
                nalgebra::ComplexField::abs(f64::from(self.0))
            }
            #[inline]
            fn acos(self) -> Self {
                $unit_name::new(nalgebra::ComplexField::acos(f64::from(self.0)))
            }
            #[inline]
            fn acosh(self) -> Self {
                $unit_name::new(nalgebra::ComplexField::acosh(f64::from(self.0)))
            }
            #[inline]
            fn asin(self) -> Self {
                $unit_name::new(nalgebra::ComplexField::asin(f64::from(self.0)))
            }
            #[inline]
            fn asinh(self) -> Self {
                $unit_name::new(nalgebra::ComplexField::asinh(f64::from(self.0)))
            }
            #[inline]
            fn atan(self) -> Self {
                $unit_name::new(nalgebra::ComplexField::atan(f64::from(self.0)))
            }
            #[inline]
            fn atanh(self) -> Self {
                $unit_name::new(nalgebra::ComplexField::atanh(f64::from(self.0)))
            }
            #[inline]
            fn cos(self) -> Self {
                $unit_name::new(nalgebra::ComplexField::cos(f64::from(self.0)))
            }
            #[inline]
            fn cosh(self) -> Self {
                $unit_name::new(nalgebra::ComplexField::cosh(f64::from(self.0)))
            }
            #[inline]
            fn exp(self) -> Self {
                $unit_name::new(nalgebra::ComplexField::exp(f64::from(self.0)))
            }
            #[inline]
            fn ln(self) -> Self {
                $unit_name::new(nalgebra::ComplexField::ln(f64::from(self.0)))
            }
            #[inline]
            fn log(self, base: f64) -> Self {
                $unit_name::new(nalgebra::ComplexField::log(f64::from(self.0), base))
            }
            #[inline]
            fn powf(self, n: Self::RealField) -> Self {
                $unit_name::new(nalgebra::ComplexField::powf(f64::from(self.0), n))
            }
            #[inline]
            fn powi(self, n: i32) -> Self {
                $unit_name::new(nalgebra::ComplexField::powi(f64::from(self.0), n))
            }
            #[inline]
            fn recip(self) -> Self {
                $unit_name::new(nalgebra::ComplexField::recip(f64::from(self.0)))
            }
            #[inline]
            fn sin(self) -> Self {
                $unit_name::new(nalgebra::ComplexField::sin(f64::from(self.0)))
            }
            #[inline]
            fn sinh(self) -> Self {
                $unit_name::new(nalgebra::ComplexField::sinh(f64::from(self.0)))
            }
            #[inline]
            fn sqrt(self) -> Self {
                $unit_name::new(nalgebra::ComplexField::sqrt(f64::from(self.0)))
            }
            #[inline]
            fn tan(self) -> Self {
                $unit_name::new(nalgebra::ComplexField::tan(f64::from(self.0)))
            }
            #[inline]
            fn tanh(self) -> Self {
                $unit_name::new(nalgebra::ComplexField::tanh(f64::from(self.0)))
            }
            #[inline]
            fn argument(self) -> Self::RealField {
                nalgebra::ComplexField::argument(f64::from(self.0))
            }
            #[inline]
            fn modulus(self) -> Self::RealField {
                nalgebra::ComplexField::modulus(f64::from(self.0))
            }
            #[inline]
            fn to_exp(self) -> (Self::RealField, Self) {
                let (r, theta) = nalgebra::ComplexField::to_exp(f64::from(self.0));
                (r, $unit_name::new(theta))
            }
            #[inline]
            fn cbrt(self) -> Self {
                $unit_name::new(nalgebra::ComplexField::cbrt(f64::from(self.0)))
            }
            #[inline]
            fn hypot(self, other: Self) -> Self::RealField {
                nalgebra::ComplexField::hypot(f64::from(self.0), f64::from(other.0))
            }
            #[inline]
            fn ceil(self) -> Self {
                $unit_name::new(nalgebra::ComplexField::ceil(f64::from(self.0)))
            }
            #[inline]
            fn floor(self) -> Self {
                $unit_name::new(nalgebra::ComplexField::floor(f64::from(self.0)))
            }
            #[inline]
            fn round(self) -> Self {
                $unit_name::new(nalgebra::ComplexField::round(f64::from(self.0)))
            }
            #[inline]
            fn trunc(self) -> Self {
                $unit_name::new(nalgebra::ComplexField::trunc(f64::from(self.0)))
            }
            #[inline]
            fn conjugate(self) -> Self {
                $unit_name::new(nalgebra::ComplexField::conjugate(f64::from(self.0)))
            }
            #[inline]
            fn cosc(self) -> Self {
                $unit_name::new(nalgebra::ComplexField::cosc(f64::from(self.0)))
            }
            #[inline]
            fn sinhc(self) -> Self {
                $unit_name::new(nalgebra::ComplexField::sinhc(f64::from(self.0)))
            }
            #[inline]
            fn signum(self) -> Self {
                $unit_name::new(nalgebra::ComplexField::signum(f64::from(self.0)))
            }
            #[inline]
            fn coshc(self) -> Self {
                $unit_name::new(nalgebra::ComplexField::coshc(f64::from(self.0)))
            }
            #[inline]
            fn exp2(self) -> Self {
                $unit_name::new(nalgebra::ComplexField::exp2(f64::from(self.0)))
            }
            #[inline]
            fn exp_m1(self) -> Self {
                $unit_name::new(nalgebra::ComplexField::exp_m1(f64::from(self.0)))
            }
            #[inline]
            fn ln_1p(self) -> Self {
                $unit_name::new(nalgebra::ComplexField::ln_1p(f64::from(self.0)))
            }
            #[inline]
            fn log10(self) -> Self {
                $unit_name::new(nalgebra::ComplexField::log10(f64::from(self.0)))
            }
            #[inline]
            fn fract(self) -> Self {
                $unit_name::new(nalgebra::ComplexField::fract(f64::from(self.0)))
            }
            #[inline]
            fn from_real(re: Self::RealField) -> Self {
                $unit_name::new(nalgebra::ComplexField::from_real(re))
            }
            #[inline]
            fn imaginary(self) -> Self::RealField {
                nalgebra::ComplexField::imaginary(f64::from(self.0))
            }
            #[inline]
            fn log2(self) -> Self {
                $unit_name::new(nalgebra::ComplexField::log2(f64::from(self.0)))
            }
            #[inline]
            fn modulus_squared(self) -> Self::RealField {
                nalgebra::ComplexField::modulus_squared(f64::from(self.0))
            }
            #[inline]
            fn mul_add(self,a:Self,b:Self) -> Self {
                $unit_name::new(nalgebra::ComplexField::mul_add(f64::from(self.0),f64::from(a.0),f64::from(b.0)))
            }
            #[inline]
            fn norm1(self) -> Self::RealField {
                nalgebra::ComplexField::norm1(f64::from(self.0))
            }
            #[inline]
            fn powc(self,n:Self) -> Self {
                $unit_name::new(nalgebra::ComplexField::powc(f64::from(self.0),f64::from(n.0)))
            }
            #[inline]
            fn real(self) -> Self::RealField {
                nalgebra::ComplexField::real(f64::from(self.0))
            }
            #[inline]
            fn scale(self,factor:Self::RealField) -> Self {
                $unit_name::new(nalgebra::ComplexField::scale(f64::from(self.0),factor))
            }
            #[inline]
            fn sin_cos(self) -> (Self,Self) {
                let (s,c) = nalgebra::ComplexField::sin_cos(f64::from(self.0));
                ($unit_name::new(s),$unit_name::new(c))
            }
            #[inline]
            fn sinc(self) -> Self {
                $unit_name::new(nalgebra::ComplexField::sinc(f64::from(self.0)))
            }
            #[inline]
            fn sinh_cosh(self) -> (Self,Self) {
                let (s,c) = nalgebra::ComplexField::sinh_cosh(f64::from(self.0));
                ($unit_name::new(s),$unit_name::new(c))
            }
            #[inline]
            fn to_polar(self) -> (Self::RealField,Self::RealField) {
                let (r,theta) = nalgebra::ComplexField::to_polar(f64::from(self.0));
                (r,theta)
            }
            #[inline]
            fn unscale(self,factor:Self::RealField) -> Self {
                $unit_name::new(nalgebra::ComplexField::unscale(f64::from(self.0),factor))
            }
        }
    }
}