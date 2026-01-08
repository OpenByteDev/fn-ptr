/// Type-level marker trait for function safety, either [`Safe`] or [`Unsafe`].
pub trait Safety {
    /// `true` for safe functions, `false` for unsafe ones.
    const IS_SAFE: bool;
}

/// Marker type for safe functions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Safe;
/// Marker type for unsafe functions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Unsafe;

impl Safety for Safe {
    const IS_SAFE: bool = true;
}
impl Safety for Unsafe {
    const IS_SAFE: bool = false;
}

/// Macro to convert a safety token (`safe` or `unsafe`) or a boolean literal to the corrsponding [`Safety`] marker type.
#[macro_export]
macro_rules! safety {
    (safe) => {
        $crate::safety::Safe
    };
    (unsafe) => {
        $crate::safety::Unsafe
    };
    (true) => {
        $crate::safety::Safe
    };
    (false) => {
        $crate::safety::Unsafe
    };
}
