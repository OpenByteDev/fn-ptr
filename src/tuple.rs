use crate::arity::{self, A0, A1, A2, A3, A4, A5, A6};
#[cfg(feature = "max-arity-12")]
use crate::arity::{A7, A8, A9, A10, A11, A12};

cfg_tt::cfg_tt! {
/// A trait implemented for all tuple types up to 6 or 12 with feature `max-arity-12` enabled.
pub trait Tuple
    #[cfg(nightly_build)]
    (: core::marker::Tuple) {
    /// Type-level representation of this tuple’s arity.
    ///
    /// For example:
    /// - `()` -> `A0`
    /// - `(T,)` -> `A1`
    /// - `(T, U)` -> `A2`
    type Arity: arity::Arity;
}
}

/// Internal helper macro to generate `Tuple` implementations.
macro_rules! impl_tuple {
    // arity 0
    (0, $arity:ty) => {
        impl Tuple for () {
            type Arity = $arity;
        }
    };

    // arity N >= 1
    ($n:tt, $arity:ty, ( $($T:ident),+ )) => {
        impl< $($T),+ > Tuple for ( $($T,)+ ) {
            type Arity = $arity;
        }
    };
}

impl_tuple!(0, A0);
impl_tuple!(1, A1, (T1));
impl_tuple!(2, A2, (T1, T2));
impl_tuple!(3, A3, (T1, T2, T3));
impl_tuple!(4, A4, (T1, T2, T3, T4));
impl_tuple!(5, A5, (T1, T2, T3, T4, T5));
impl_tuple!(6, A6, (T1, T2, T3, T4, T5, T6));
#[cfg(feature = "max-arity-12")]
impl_tuple!(7, A7, (T1, T2, T3, T4, T5, T6, T7));
#[cfg(feature = "max-arity-12")]
impl_tuple!(8, A8, (T1, T2, T3, T4, T5, T6, T7, T8));
#[cfg(feature = "max-arity-12")]
impl_tuple!(9, A9, (T1, T2, T3, T4, T5, T6, T7, T8, T9));
#[cfg(feature = "max-arity-12")]
impl_tuple!(10, A10, (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10));
#[cfg(feature = "max-arity-12")]
impl_tuple!(11, A11, (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11));
#[cfg(feature = "max-arity-12")]
impl_tuple!(12, A12, (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12));
