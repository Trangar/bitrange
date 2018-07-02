#![deny(warnings)]
#![cfg_attr(not(feature = "std"), no_std)]

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
/// #[macro_use]
/// extern crate bitrange;
/// #[macro_use]
/// extern crate bitrange_plugin;
/// # fn main() {
/// bitrange! {
///     Test: u8, "u8",         // the name of the struct and the size of the internal integer
///     "aaa1_0bbb",            // the format of the bits in the internal integer
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
        $struct_size_string:tt,
        $format:tt,
        $(
            $field_format:ident:
            $(
                $field_name:ident
            )+
        ),+
    ) => {
        #[derive(Bitrange)]
        #[BitrangeMask = $format]
        #[BitrangeSize = $struct_size_string]
        pub struct $struct_name {
            #[allow(dead_code)]
            bits: $struct_size
        }
        impl Default for $struct_name {
            #[allow(dead_code)]
            fn default() -> $struct_name {
                #[allow(dead_code, non_snake_case)]
                let DEFAULT_VALUE: $struct_size = $struct_name::__bitrange_get_default_value();
                $struct_name {
                    bits: DEFAULT_VALUE
                }
            }
        }
        impl $struct_name {
            #[allow(dead_code)]
            pub fn from(bits: $struct_size) -> Result<$struct_name, ::bitrange::Error<$struct_size>> {
                #[allow(dead_code, non_snake_case)]
                let DEFAULT_VALUE: $struct_size = $struct_name::__bitrange_get_default_value();
                #[allow(dead_code, non_snake_case)]
                let DEFAULT_MASK: $struct_size = $struct_name::__bitrange_get_default_mask();

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
    ($struct_name:ident, $struct_size:tt, $format:tt, $field_format:ident, $field_get:ident) => {
        impl $struct_name {
            pub fn $field_get(&self) -> $struct_size {
                #[allow(dead_code, non_snake_case)]
                let MASK: $struct_size = $struct_name::__bitrange_get_mask(stringify!($field_format));
                #[allow(dead_code, non_snake_case)]
                let OFFSET: usize = $struct_name::__bitrange_get_offset(stringify!($field_format));

                (self.bits & MASK) >> OFFSET
            }
        }
    };
    (
        $struct_name:ident,
        $struct_size:tt,
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
                #[allow(dead_code, non_snake_case)]
                let MASK: $struct_size = $struct_name::__bitrange_get_mask(stringify!($field_format));
                #[allow(dead_code, non_snake_case)]
                let OFFSET: usize = $struct_name::__bitrange_get_offset(stringify!($field_format));
                self.bits &= !MASK;
                self.bits |= (value << OFFSET) & MASK;
                self
            }
        }
    };
}
