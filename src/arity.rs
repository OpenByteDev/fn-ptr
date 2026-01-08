/// Type-level marker trait for function arity, from [`A0`] to [`A12`].
pub trait Arity {
    /// Number of parameters for this arity.
    const N: usize;
}
macro_rules! define_arity_marker {
    ($(($name:ident, $n:expr)),+ $(,)?) => {
        $(
            #[doc = "Type-level marker for functions with exactly "]
            #[doc = stringify!($n)]
            #[doc = " parameters."]
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub struct $name;

            impl Arity for $name {
                const N: usize = $n;
            }
        )+
    };
}
define_arity_marker!(
    (A0, 0),
    (A1, 1),
    (A2, 2),
    (A3, 3),
    (A4, 4),
    (A5, 5),
    (A6, 6),
    (A7, 7),
    (A8, 8),
    (A9, 9),
    (A10, 10),
    (A11, 11),
    (A12, 12),
);

/// Macro to convert an integral number to the corresponding [`Arity`] marker type.
#[macro_export]
macro_rules! arity {
    (0) => {
        $crate::marker::A0
    };
    (1) => {
        $crate::marker::A1
    };
    (2) => {
        $crate::marker::A2
    };
    (3) => {
        $crate::marker::A3
    };
    (4) => {
        $crate::marker::A4
    };
    (5) => {
        $crate::marker::A5
    };
    (6) => {
        $crate::marker::A6
    };
    (7) => {
        $crate::marker::A7
    };
    (8) => {
        $crate::marker::A8
    };
    (9) => {
        $crate::marker::A9
    };
    (10) => {
        $crate::marker::A10
    };
    (11) => {
        $crate::marker::A11
    };
    (12) => {
        $crate::marker::A12
    };
}
