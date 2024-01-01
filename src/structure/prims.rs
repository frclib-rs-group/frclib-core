macro_rules! empty_schema_supplier {
    () => {
        || String::with_capacity(0)
    };
}

impl super::FrcStructure for f64 {
    const SCHEMA_SUPPLIER: fn() -> String = empty_schema_supplier!();
    const TYPE: &'static str = "float64";
    const SIZE: usize = 8;

    #[inline]
    fn pack(&self, buffer: &mut impl bytes::BufMut) {
        buffer.put_f64(*self);
    }

    #[inline]
    fn unpack(buffer: &mut impl bytes::Buf) -> Self {
        buffer.get_f64()
    }
}

impl super::FrcStructure for f32 {
    const TYPE: &'static str = "float32";
    const SIZE: usize = 4;
    const SCHEMA_SUPPLIER: fn() -> String = empty_schema_supplier!();

    #[inline]
    fn pack(&self, buffer: &mut impl bytes::BufMut) {
        buffer.put_f32(*self);
    }

    #[inline]
    fn unpack(buffer: &mut impl bytes::Buf) -> Self {
        buffer.get_f32()
    }
}

impl super::FrcStructure for i64 {
    const TYPE: &'static str = "int64";
    const SIZE: usize = 8;
    const SCHEMA_SUPPLIER: fn() -> String = empty_schema_supplier!();

    #[inline]
    fn pack(&self, buffer: &mut impl bytes::BufMut) {
        buffer.put_i64(*self);
    }

    #[inline]
    fn unpack(buffer: &mut impl bytes::Buf) -> Self {
        buffer.get_i64()
    }
}

impl super::FrcStructure for i32 {
    const TYPE: &'static str = "int32";
    const SIZE: usize = 4;
    const SCHEMA_SUPPLIER: fn() -> String = empty_schema_supplier!();

    #[inline]
    fn pack(&self, buffer: &mut impl bytes::BufMut) {
        buffer.put_i32(*self);
    }

    #[inline]
    fn unpack(buffer: &mut impl bytes::Buf) -> Self {
        buffer.get_i32()
    }
}

impl super::FrcStructure for i16 {
    const TYPE: &'static str = "int16";
    const SIZE: usize = 2;
    const SCHEMA_SUPPLIER: fn() -> String = empty_schema_supplier!();

    #[inline]
    fn pack(&self, buffer: &mut impl bytes::BufMut) {
        buffer.put_i16(*self);
    }

    #[inline]
    fn unpack(buffer: &mut impl bytes::Buf) -> Self {
        buffer.get_i16()
    }
}

impl super::FrcStructure for i8 {
    const TYPE: &'static str = "int8";
    const SIZE: usize = 1;
    const SCHEMA_SUPPLIER: fn() -> String = empty_schema_supplier!();

    #[inline]
    fn pack(&self, buffer: &mut impl bytes::BufMut) {
        buffer.put_i8(*self);
    }

    #[inline]
    fn unpack(buffer: &mut impl bytes::Buf) -> Self {
        buffer.get_i8()
    }
}

impl super::FrcStructure for u64 {
    const TYPE: &'static str = "uint64";
    const SIZE: usize = 8;
    const SCHEMA_SUPPLIER: fn() -> String = empty_schema_supplier!();

    #[inline]
    fn pack(&self, buffer: &mut impl bytes::BufMut) {
        buffer.put_u64(*self);
    }

    #[inline]
    fn unpack(buffer: &mut impl bytes::Buf) -> Self {
        buffer.get_u64()
    }
}

impl super::FrcStructure for u32 {
    const TYPE: &'static str = "uint32";
    const SIZE: usize = 4;
    const SCHEMA_SUPPLIER: fn() -> String = empty_schema_supplier!();

    #[inline]
    fn pack(&self, buffer: &mut impl bytes::BufMut) {
        buffer.put_u32(*self);
    }

    #[inline]
    fn unpack(buffer: &mut impl bytes::Buf) -> Self {
        buffer.get_u32()
    }
}

impl super::FrcStructure for u16 {
    const TYPE: &'static str = "uint16";
    const SIZE: usize = 2;
    const SCHEMA_SUPPLIER: fn() -> String = empty_schema_supplier!();

    #[inline]
    fn pack(&self, buffer: &mut impl bytes::BufMut) {
        buffer.put_u16(*self);
    }

    #[inline]
    fn unpack(buffer: &mut impl bytes::Buf) -> Self {
        buffer.get_u16()
    }
}

impl super::FrcStructure for u8 {
    const TYPE: &'static str = "uint8";
    const SIZE: usize = 1;
    const SCHEMA_SUPPLIER: fn() -> String = empty_schema_supplier!();

    #[inline]
    fn pack(&self, buffer: &mut impl bytes::BufMut) {
        buffer.put_u8(*self);
    }

    #[inline]
    fn unpack(buffer: &mut impl bytes::Buf) -> Self {
        buffer.get_u8()
    }
}

impl super::FrcStructure for bool {
    const TYPE: &'static str = "bool";
    const SIZE: usize = 1;
    const SCHEMA_SUPPLIER: fn() -> String = empty_schema_supplier!();

    #[inline]
    fn pack(&self, buffer: &mut impl bytes::BufMut) {
        buffer.put_u8(u8::from(*self));
    }

    #[inline]
    fn unpack(buffer: &mut impl bytes::Buf) -> Self {
        buffer.get_u8() != 0
    }
}

impl super::FrcStructure for char {
    const TYPE: &'static str = "char";
    const SIZE: usize = 1;
    const SCHEMA_SUPPLIER: fn() -> String = empty_schema_supplier!();

    #[inline]
    fn pack(&self, buffer: &mut impl bytes::BufMut) {
        buffer.put_u8(*self as u8);
    }

    #[inline]
    fn unpack(buffer: &mut impl bytes::Buf) -> Self {
        buffer.get_u8() as Self
    }
}

impl<T, const N: usize> super::FrcStructure for [T; N]
where
    T: super::FrcStructure + Copy,
{
    const TYPE: &'static str = T::TYPE;
    const SIZE: usize = T::SIZE * N;
    const SCHEMA_SUPPLIER: fn() -> String = empty_schema_supplier!();

    #[inline]
    fn pack(&self, buffer: &mut impl bytes::BufMut) {
        for item in self {
            item.pack(buffer);
        }
    }

    #[inline]
    fn unpack(buffer: &mut impl bytes::Buf) -> Self {
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
