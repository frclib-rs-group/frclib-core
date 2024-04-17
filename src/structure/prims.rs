use std::io::{Cursor, Read};

use num::traits::{FromBytes, ToBytes};

fn empty_schema_supplier() -> String {
    String::with_capacity(0)
}

macro_rules! prim_structure {
    ($typ:ident , $string:literal) => {
        impl super::FrcStructure for $typ {
            const SCHEMA_SUPPLIER: fn() -> String = empty_schema_supplier;
            const TYPE: &'static str = $string;
            const SIZE: usize = core::mem::size_of::<Self>();

            #[inline]
            fn pack(&self, buffer: &mut Vec<u8>) {
                buffer.extend_from_slice(&<Self as ToBytes>::to_le_bytes(self));
            }

            #[inline]
            fn unpack(buffer: &mut Cursor<&[u8]>) -> Self {
                let mut value_buffer = [0u8; Self::SIZE];
                let _ = buffer.read_exact(&mut value_buffer);
                <Self as FromBytes>::from_le_bytes(&value_buffer)
            }
        }
    };
}

prim_structure!(f64, "float64");
prim_structure!(f32, "float32");
prim_structure!(i64, "int64");
prim_structure!(i32, "int32");
prim_structure!(i16, "int16");
prim_structure!(i8, "int8");
prim_structure!(u64, "uint64");
prim_structure!(u32, "uint32");
prim_structure!(u16, "uint16");
prim_structure!(u8, "uint8");

impl super::FrcStructure for bool {
    const SCHEMA_SUPPLIER: fn() -> String = empty_schema_supplier;
    const TYPE: &'static str = "bool";
    const SIZE: usize = core::mem::size_of::<Self>();
    #[inline]
    fn pack(&self, buffer: &mut Vec<u8>) {
        buffer.push(u8::from(*self));
    }
    #[inline]
    fn unpack(buffer: &mut Cursor<&[u8]>) -> Self {
        let mut value_buffer = [0u8; Self::SIZE];
        let _ = buffer.read_exact(&mut value_buffer);
        value_buffer[0] != 0
    }
}

impl super::FrcStructure for char {
    const SCHEMA_SUPPLIER: fn() -> String = empty_schema_supplier;
    const TYPE: &'static str = "char";
    const SIZE: usize = 1;
    #[inline]
    fn pack(&self, buffer: &mut Vec<u8>) {
        buffer.push(*self as u8);
    }
    #[inline]
    fn unpack(buffer: &mut Cursor<&[u8]>) -> Self {
        let mut value_buffer = [0u8; Self::SIZE];
        let _ = buffer.read_exact(&mut value_buffer);
        value_buffer[0] as Self
    }
}

impl<T, const N: usize> super::FrcStructure for [T; N]
where
    T: super::FrcStructure + Copy,
{
    const TYPE: &'static str = T::TYPE;
    const SIZE: usize = T::SIZE * N;
    const SCHEMA_SUPPLIER: fn() -> String = empty_schema_supplier;

    #[inline]
    fn pack(&self, buffer: &mut Vec<u8>) {
        for item in self {
            item.pack(buffer);
        }
    }

    #[inline]
    fn unpack(buffer: &mut Cursor<&[u8]>) -> Self {
        let mut arr = [T::unpack(buffer); N];
        let mut is_first = true;
        for item in &mut arr {
            if is_first {
                is_first = false;
            } else {
                *item = T::unpack(buffer);
            }
        }
        arr
    }

    fn format_field(field: &str) -> String {
        format!("{} {}[{}]", T::TYPE, field, N)
    }
}
