#![feature(proc_macro)]
extern crate bitrange_macro;

pub use bitrange_macro::{proc_mask, proc_offset, proc_default_mask, proc_default_value};

#[macro_export]
macro_rules! bitrange {
    (
        $struct_name:ident:
        $struct_size:ty,
        $format:expr,
        $(
            $field_format:ident:
            $(
                $field_name:ident
            )+
        ),+
    ) => {
        pub struct $struct_name { bits: $struct_size }

        impl Default for $struct_name {
            fn default() -> $struct_name {
                const DEFAULT_VALUE: $struct_size = ::bitrange::proc_default_value!($format);
                $struct_name {
                    bits: DEFAULT_VALUE
                }
            }
        }
        impl $struct_name {
            pub fn from(bits: $struct_size) -> $struct_name {
                const DEFAULT_VALUE: $struct_size = ::bitrange::proc_default_value!($format);
                const DEFAULT_MASK: $struct_size = ::bitrange::proc_default_mask!($format);

                assert_eq!(bits & DEFAULT_MASK, DEFAULT_VALUE, "Given bits do not match the pattern");
                $struct_name {
                    bits
                }
            }
        }
        $(
            bitrange_impl_field!($struct_name, $struct_size, $format, $field_format, $($field_name),+);
        )+
    }
}

#[macro_export]
macro_rules! bitrange_impl_field {
    ($struct_name:ident, $struct_size:ty, $format:expr, $field_format:ident, $field_get:ident) => {
        impl $struct_name {
            pub fn $field_get(&self) -> $struct_size {
                const MASK: $struct_size = ::bitrange::proc_mask!($format, $field_format);
                const OFFSET: $struct_size = ::bitrange::proc_offset!($format, $field_format);

                (self.bits & MASK) >> OFFSET
            }
        }
    };
    ($struct_name:ident, $struct_size:ty, $format:expr, $field_format:ident, $field_get:ident, $field_set:ident) => {
        bitrange_impl_field!($struct_name, $struct_size, $format, $field_format, $field_get);

        impl $struct_name {
            pub fn $field_set(&mut self, value: $struct_size) -> &mut Self {
                const MASK: $struct_size = ::bitrange::proc_mask!($format, $field_format);
                const OFFSET: $struct_size = ::bitrange::proc_offset!($format, $field_format);
                self.bits &= !MASK;
                self.bits |= (value << OFFSET) & MASK;
                self
            }
        }
    }
}
