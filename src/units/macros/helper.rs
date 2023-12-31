


#[macro_export]
macro_rules! inner_unit_conversion {
    ($unit_a:ident $type_a:ty | $unit_b:ident $type_b:ty : $conv_fn:ident) => {

        #[allow(trivial_numeric_casts)]
        impl From<$unit_a> for $unit_b {
            fn from(value: $unit_a) -> Self {
                $unit_b($conv_fn(value.0))
            }
        }
        #[allow(trivial_numeric_casts)]
        impl From<&$unit_a> for $unit_b {
            fn from(value: &$unit_a) -> Self {
                $unit_b($conv_fn(value.0))
            }
        }
        #[allow(trivial_numeric_casts)]
        impl From<$unit_b> for $unit_a {
            fn from(value: $unit_b) -> Self {
                $unit_a((value.0 / $conv_fn(<$type_a>::from(1.0)) as $type_b) as $type_a)
            }
        }
        #[allow(trivial_numeric_casts)]
        impl From<&$unit_b> for $unit_a {
            fn from(value: &$unit_b) -> Self {
                $unit_a((value.0 / $conv_fn(<$type_a>::from(1.0)) as $type_b) as $type_a)
            }
        }

        #[allow(trivial_numeric_casts)]
        impl std::cmp::PartialEq<$unit_a> for $unit_b {
            fn eq(&self, other: &$unit_a) -> bool {
                self.0 == ($conv_fn(other.0) as $type_b)
            }
        }
        #[allow(trivial_numeric_casts)]
        impl std::cmp::PartialOrd<$unit_a> for $unit_b {
            fn partial_cmp(&self, other: &$unit_a) -> Option<std::cmp::Ordering> {
                self.0.partial_cmp(&$conv_fn(other.0))
            }
        }
        #[allow(trivial_numeric_casts)]
        impl std::cmp::PartialEq<$unit_b> for $unit_a {
            fn eq(&self, other: &$unit_b) -> bool {
                self.0 == ((other.0 / $conv_fn(<$type_a>::from(1.0)) as $type_b) as $type_a)
            }
        }
        #[allow(trivial_numeric_casts)]
        impl std::cmp::PartialOrd<$unit_b> for $unit_a {
            fn partial_cmp(&self, other: &$unit_b) -> Option<std::cmp::Ordering> {
                self.0.partial_cmp(&((other.0 / $conv_fn(<$type_a>::from(1.0)) as $type_b) as $type_a))
            }
        }
    };
}

#[macro_export]
macro_rules! unit_dim_analysis {
    ($unit_a:ident * $unit_b:ident = $ret:ident) => {
        impl std::ops::Mul<$unit_b> for $unit_a {
            type Output = $ret;
            #[inline]
            fn mul(self, rhs: $unit_b) -> Self::Output {
                $ret::from(self.0 * rhs.0)
            }
        }
        impl std::ops::Mul<$unit_a> for $unit_b {
            type Output = $ret;
            #[inline]
            fn mul(self, rhs: $unit_a) -> Self::Output {
                $ret::from(self.0 * rhs.0)
            }
        }
        impl std::ops::Mul<&$unit_b> for $unit_a {
            type Output = $ret;
            #[inline]
            fn mul(self, rhs: &$unit_b) -> Self::Output {
                $ret::from(self.0 * rhs.0)
            }
        }
        impl std::ops::Mul<$unit_a> for &$unit_b {
            type Output = $ret;
            #[inline]
            fn mul(self, rhs: $unit_a) -> Self::Output {
                $ret::from(self.0 * rhs.0)
            }
        }
        impl std::ops::Mul<$unit_b> for &$unit_a {
            type Output = $ret;
            #[inline]
            fn mul(self, rhs: $unit_b) -> Self::Output {
                $ret::from(self.0 * rhs.0)
            }
        }
        impl std::ops::Mul<&$unit_a> for $unit_b {
            type Output = $ret;
            #[inline]
            fn mul(self, rhs: &$unit_a) -> Self::Output {
                $ret::from(self.0 * rhs.0)
            }
        }
        impl std::ops::Mul<&$unit_b> for &$unit_a {
            type Output = $ret;
            #[inline]
            fn mul(self, rhs: &$unit_b) -> Self::Output {
                $ret::from(self.0 * rhs.0)
            }
        }
        impl std::ops::Mul<&$unit_a> for &$unit_b {
            type Output = $ret;
            #[inline]
            fn mul(self, rhs: &$unit_a) -> Self::Output {
                $ret::from(self.0 * rhs.0)
            }
        }

        //other order
        impl std::ops::Div<$unit_a> for $ret {
            type Output = $unit_b;
            #[inline]
            fn div(self, rhs: $unit_a) -> Self::Output {
                $unit_b::from(self.0 / rhs.0)
            }
        }
        impl std::ops::Div<$unit_b> for $ret {
            type Output = $unit_a;
            #[inline]
            fn div(self, rhs: $unit_b) -> Self::Output {
                $unit_a::from(self.0 / rhs.0)
            }
        }
        impl std::ops::Div<&$unit_a> for $ret {
            type Output = $unit_b;
            #[inline]
            fn div(self, rhs: &$unit_a) -> Self::Output {
                $unit_b::from(self.0 / rhs.0)
            }
        }
        impl std::ops::Div<$unit_a> for &$ret {
            type Output = $unit_b;
            #[inline]
            fn div(self, rhs: $unit_a) -> Self::Output {
                $unit_b::from(self.0 / rhs.0)
            }
        }
        impl std::ops::Div<&$unit_a> for &$ret {
            type Output = $unit_b;
            #[inline]
            fn div(self, rhs: &$unit_a) -> Self::Output {
                $unit_b::from(self.0 / rhs.0)
            }
        }
        impl std::ops::Div<&$unit_b> for $ret {
            type Output = $unit_a;
            #[inline]
            fn div(self, rhs: &$unit_b) -> Self::Output {
                $unit_a::from(self.0 / rhs.0)
            }
        }
        impl std::ops::Div<$unit_b> for &$ret {
            type Output = $unit_a;
            #[inline]
            fn div(self, rhs: $unit_b) -> Self::Output {
                $unit_a::from(self.0 / rhs.0)
            }
        }
        impl std::ops::Div<&$unit_b> for &$ret {
            type Output = $unit_a;
            #[inline]
            fn div(self, rhs: &$unit_b) -> Self::Output {
                $unit_a::from(self.0 / rhs.0)
            }
        }
    };
    ($unit_a:ident / $unit_b:ident = $ret:ident) => {
        impl std::ops::Div<$unit_b> for $unit_a {
            type Output = $ret;
            fn div(self, rhs: $unit_b) -> Self::Output {
                $ret::from(self.0 / rhs.0)
            }
        }
        impl std::ops::Div<&$unit_b> for $unit_a {
            type Output = $ret;
            fn div(self, rhs: &$unit_b) -> Self::Output {
                $ret::from(self.0 / rhs.0)
            }
        }
        impl std::ops::Div<$unit_b> for &$unit_a {
            type Output = $ret;
            fn div(self, rhs: $unit_b) -> Self::Output {
                $ret::from(self.0 / rhs.0)
            }
        }
        impl std::ops::Div<&$unit_b> for &$unit_a {
            type Output = $ret;
            fn div(self, rhs: &$unit_b) -> Self::Output {
                $ret::from(self.0 / rhs.0)
            }
        }
    };
}