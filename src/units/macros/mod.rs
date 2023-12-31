
#[macro_use]
mod components;
#[macro_use]
mod number;
#[macro_use]
mod helper;


#[macro_export]
macro_rules! unit {
    ($unit_name:ident : float) => {

        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        pub struct $unit_name(pub f64);

        impl std::hash::Hash for $unit_name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.0.to_bits().hash(state);
            }
        }

        impl $unit_name {
            #[must_use]
            #[inline]
            pub const fn new(value: f64) -> Self {
                Self(value)
            }

            #[must_use]
            #[inline]
            pub const fn value(self) -> f64 {
                self.0
            }
        }

        $crate::unit_general!($unit_name : f64);
        $crate::unit_binops!($unit_name : f64);
        $crate::unit_neg!($unit_name : f64);
        $crate::unit_serde!($unit_name : f64);
        $crate::unit_num!($unit_name : f64);
        $crate::unit_float!($unit_name);
        $crate::unit_structure!($unit_name : f64);
    };
    ($unit_name:ident : int) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $unit_name(pub i64);

        impl $unit_name {
            #[must_use]
            #[inline]
            pub const fn new(value: i64) -> Self {
                Self(value)
            }

            #[must_use]
            #[inline]
            pub const fn value(self) -> i64 {
                self.0
            }
        }

        $crate::unit_general!($unit_name : i64);
        $crate::unit_binops!($unit_name : i64);
        $crate::unit_neg!($unit_name : i64);
        $crate::unit_serde!($unit_name : i64);
        $crate::unit_num!($unit_name : i64);
        $crate::unit_integer!($unit_name);
        $crate::unit_structure!($unit_name : i64);
    };
    ($unit_name:ident : uint) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $unit_name(pub u64);

        impl $unit_name {
            #[must_use]
            #[inline]
            pub const fn new(value: u64) -> Self {
                Self(value)
            }

            #[must_use]
            #[inline]
            pub const fn value(self) -> u64 {
                self.0
            }
        }

        $crate::unit_general!($unit_name : u64);
        $crate::unit_binops!($unit_name : u64);
        $crate::unit_serde!($unit_name : u64);
        $crate::unit_num!($unit_name : u64);
        $crate::unit_uinteger!($unit_name);
        $crate::unit_structure!($unit_name : u64);
    };
}

#[macro_export]
macro_rules! unit_conversion {
    ($unit_a:ident ( float ) <-> $unit_b:ident ( float ) ~ $conv_fn:ident ) => {
        $crate::inner_unit_conversion!($unit_a f64 | $unit_b f64 : $conv_fn);
    };
    ($unit_a:ident ( int ) <-> $unit_b:ident ( int ) ~ $conv_fn:ident ) => {
        $crate::inner_unit_conversion!($unit_a i64 | $unit_b i64 : $conv_fn);
    };
    ($unit_a:ident ( uint ) <-> $unit_b:ident ( uint ) ~ $conv_fn:ident ) => {
        $crate::inner_unit_conversion!($unit_a u64 | $unit_b u64 : $conv_fn);
    };
    ($unit_a:ident ( float ) <-> $unit_b:ident ( int ) ~ $conv_fn:ident ) => {
        $crate::inner_unit_conversion!($unit_a f64 | $unit_b i64 : $conv_fn);
    };
    ($unit_a:ident ( float ) <-> $unit_b:ident ( uint ) ~ $conv_fn:ident ) => {
        $crate::inner_unit_conversion!($unit_a f64 | $unit_b u64 : $conv_fn);
    };
    ($unit_a:ident ( int ) <-> $unit_b:ident ( float ) ~ $conv_fn:ident ) => {
        $crate::inner_unit_conversion!($unit_a i64 | $unit_b f64 : $conv_fn);
    };
    ($unit_a:ident ( uint ) <-> $unit_b:ident ( float ) ~ $conv_fn:ident ) => {
        $crate::inner_unit_conversion!($unit_a u64 | $unit_b f64 : $conv_fn);
    };
    ($unit_a:ident ( int ) <-> $unit_b:ident ( uint ) ~ $conv_fn:ident ) => {
        $crate::inner_unit_conversion!($unit_a i64 | $unit_b u64 : $conv_fn);
    };
    ($unit_a:ident ( uint ) <-> $unit_b:ident ( int ) ~ $conv_fn:ident ) => {
        $crate::inner_unit_conversion!($unit_a u64 | $unit_b i64 : $conv_fn);
    };
}

#[macro_export]
macro_rules! unit_family {
    ($family_name:ident ( $standard:ident ): $($unit_name:ident),*) => {
        // #[doc = "A family of units. "]
        // #[doc = "The standard unit is "]
        // #[doc = stringify!($standard)]
        // #[doc = "The other units are "]
        // #[doc = stringify!($($unit_name),*)]
        pub trait $family_name: Into<$standard> + From<$standard> + Copy {
            fn standard(self) -> $standard {
                self.into()
            }
        }

        // impl $family_name for $standard {}
        // $(
        //     impl $family_name for $unit_name {}
        // )*

        impl<T> $family_name for T
        where
            T: Into<$standard> + From<$standard> + Copy,
        {
        }
    };
}


#[cfg(test)]
mod test {
    unit!(Degree: float);
    unit!(Millisecond: int);
    unit!(Microsecond: uint);

    #[test]
    fn ops() {
        let deg = Degree(1.0);
        let new_deg = 1.0f64 + deg;
        assert_eq!(new_deg, Degree(2.0));

        let milli = Millisecond(-1);
        let new_milli = 1i64 + milli;
        assert_eq!(new_milli, Millisecond(0));

        let micro = Microsecond(1);
        let new_micro = 1u64 + micro;
        assert_eq!(new_micro, Microsecond(2));
    }
}
