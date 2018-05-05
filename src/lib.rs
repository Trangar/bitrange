#![deny(warnings)]
#![cfg_attr(not(feature = "std"), no_std)]

#![feature(proc_macro)]
#![feature(proc_macro_non_items)]
extern crate bitrange_plugin;


/// Return the default mask of a format
/// This are all the fields that are set to either `0` or `1`
/// 
/// usage:  `proc_default_mask([aaa0_1bbb]);`
/// output: `0b0001_1000`
pub use bitrange_plugin::proc_default_mask;

/// Returns the default value of a format
/// This is a value with 1 for every `1` in the format
/// 
/// usage:  `proc_default_value([aaa0_1bbb]);`
/// output: `0b0000_1000`
pub use bitrange_plugin::proc_default_value;

/// Create a mask based on a given format and a character
/// This will map all the bits that match the given character, to 1
/// All other bits will be set to 0
/// 
/// usage:  `proc_mask!([aaa0_1bbb], a);`
/// output: `0b1110_0000`
pub use bitrange_plugin::proc_mask;

/// Return the offset of a given character in a format
/// This is the amount of least-significant bits in the proc_mask that are 0
/// 
/// usage:  `proc_offset([aaa0_1bbb], a);`
/// output: `5` (0b1110_0000 has 5 least-significant bits that are 0)
pub use bitrange_plugin::proc_offset;

#[cfg(feature = "panic")]
mod error {
    #[cfg(feature = "std")]
    use std::marker::PhantomData;
    #[cfg(feature = "std")]
    use std::fmt::Binary;
    #[cfg(not(feature = "std"))]
    use core::marker::PhantomData;
    #[cfg(not(feature = "std"))]
    use core::fmt::Binary;

    #[derive(Debug)]
    pub struct Error<T> { _phantomdata: PhantomData<T> }
    
    impl<T> Error<T> where T : Binary {
        pub fn invalid_bits(expected: T, provided: T) -> Error<T> {
            panic!("Invalid bits, expected 0b{:0b}, got 0b{:0b}", expected, provided);
        }
    }
}

#[cfg(not(feature = "panic"))]
mod error {
    #[derive(Debug)]
    pub struct Error<T> {
        pub expected: T,
        pub provided: T,
    }

    impl<T> Error<T> {
        pub fn invalid_bits(expected: T, provided: T) -> Error<T> {
            Error {
                expected,
                provided,
            }
        }
    }
}

pub use error::Error;

/// Create a bitrange struct.
/// 
/// ```rust
/// #![deny(warnings)]
/// #![feature(proc_macro)]
/// #[macro_use]
/// extern crate bitrange;
/// # fn main() {
/// bitrange! {
///     Test: u8,               // the name of the struct and the size of the internal integer
///     [aaa1_0bbb],            // the format of the bits in the internal integer
///     a: first,               // map the bits that are marked as `a` to field `first`
///     b: second set_second    // map the bits that are marked as `b` to field `second`
///                             // and create a setter `set_second` that sets a given value to `b`
/// }
/// # }
/// ```
#[macro_export]
macro_rules! bitrange {
    (
        $struct_name:ident:
        $struct_size:ty,
        $format:tt,
        $(
            $field_format:ident:
            $(
                $field_name:ident
            )+
        ),+
    ) => {
        pub struct $struct_name {
            #[allow(dead_code)]
            bits: $struct_size
        }

        impl Default for $struct_name {
            #[allow(dead_code)]
            fn default() -> $struct_name {
                const DEFAULT_VALUE: $struct_size = ::bitrange::proc_default_value!($format);
                $struct_name {
                    bits: DEFAULT_VALUE
                }
            }
        }
        impl $struct_name {
            #[allow(dead_code)]
            pub fn from(bits: $struct_size) -> Result<$struct_name, ::bitrange::Error<$struct_size>> {
                #[allow(dead_code)]
                const DEFAULT_VALUE: $struct_size = ::bitrange::proc_default_value!($format);
                #[allow(dead_code)]
                const DEFAULT_MASK: $struct_size = ::bitrange::proc_default_mask!($format);

                if bits & DEFAULT_MASK == DEFAULT_VALUE {
                    Ok($struct_name {
                        bits
                    })
                } else {
                    Err(::bitrange::Error::invalid_bits(DEFAULT_VALUE, bits & DEFAULT_MASK))
                }
            }
        }
        $(
            bitrange_impl_field!(
                $struct_name,
                $struct_size,
                $format,
                $field_format,
                $($field_name),+
            );
        )+
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! bitrange_impl_field {
    ($struct_name:ident, $struct_size:ty, $format:tt, $field_format:ident, $field_get:ident) => {
        impl $struct_name {
            pub fn $field_get(&self) -> $struct_size {
                #[allow(dead_code)]
                const MASK: $struct_size = ::bitrange::proc_mask!($format, $field_format);
                #[allow(dead_code)]
                const OFFSET: $struct_size = ::bitrange::proc_offset!($format, $field_format);

                (self.bits & MASK) >> OFFSET
            }
        }
    };
    (
        $struct_name:ident,
        $struct_size:ty,
        $format:tt,
        $field_format:ident,
        $field_get:ident,
        $field_set:ident
    ) => {
        bitrange_impl_field!(
            $struct_name,
            $struct_size,
            $format,
            $field_format,
            $field_get
        );

        impl $struct_name {
            pub fn $field_set(&mut self, value: $struct_size) -> &mut Self {
                #[allow(dead_code)]
                const MASK: $struct_size = ::bitrange::proc_mask!($format, $field_format);
                #[allow(dead_code)]
                const OFFSET: $struct_size = ::bitrange::proc_offset!($format, $field_format);
                self.bits &= !MASK;
                self.bits |= (value << OFFSET) & MASK;
                self
            }
        }
    };
}
