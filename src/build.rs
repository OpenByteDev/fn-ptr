use crate::{
    FnPtr, Tuple, WithAbi, WithArgs, WithOutput, WithSafety,
    abi::{self, Rust},
    safety::{self, Safe},
};

/// Constructs a function-pointer type from its components.
///
/// Given
/// - a tuple of argument types (`Self`)
/// - a safety marker ([`Safety`](crate::safety::Safety))
/// - an ABI ([`Abi`](crate::abi::Abi))
/// - an output type (`Output`)
///
/// The trait is implemented for tuples of argument types and produces the
/// corresponding function-pointer type via the associated type [`F`](BuildFn::F).
///
/// # Examples
///
/// ```rust
/// use fn_ptr::{BuildFn, safety, abi};
/// type F0 = <(i32,) as BuildFn>::F; // fn(i32)
/// type F1 = <() as BuildFn<safety::Unsafe, abi::Rust, u64>>::F; // unsafe fn() -> u64
/// type F2 = <(String, f32) as BuildFn<safety::Safe, abi::C, i32>>::F; // extern "C" fn(String, f32) -> i32
/// ```
pub trait BuildFn<Safety = Safe, Abi = Rust, Output = ()>: Tuple {
    /// The resulting function-pointer type.
    type F: FnPtr<Args = Self, Output = Output, Safety = Safety, Abi = Abi>;
}

impl<G: FnPtr, Args: BuildFn<G::Safety, G::Abi, G::Output>> WithArgs<Args> for G {
    type F = <Args as BuildFn<G::Safety, G::Abi, G::Output>>::F;
}

impl<Output, G: FnPtr> WithOutput<Output> for G
where
    G::Args: BuildFn<G::Safety, G::Abi, Output>,
{
    type F = <G::Args as BuildFn<G::Safety, G::Abi, Output>>::F;
}

impl<Safety: safety::Safety, G: FnPtr> WithSafety<Safety> for G
where
    G::Args: BuildFn<Safety, G::Abi, G::Output>,
{
    type F = <G::Args as BuildFn<Safety, G::Abi, G::Output>>::F;
}

impl<Abi: abi::Abi, G: FnPtr> WithAbi<Abi> for G
where
    G::Args: BuildFn<G::Safety, Abi, G::Output>,
{
    type F = <G::Args as BuildFn<G::Safety, Abi, G::Output>>::F;
}
