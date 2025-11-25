use fn_ptr::{Abi, FunctionPtr};
use static_assertions::assert_type_eq_all;

#[test]
fn unsafe_fn() {
    type F = unsafe fn(i32) -> i32;

    assert_type_eq_all!(<F as FunctionPtr>::Args, (i32,));
    assert_type_eq_all!(<F as FunctionPtr>::Output, i32);

    assert_eq!(<F as FunctionPtr>::ARITY, 1);
    assert!(!<F as FunctionPtr>::SAFE);
    assert!(!<F as FunctionPtr>::EXTERN);
    assert_eq!(<F as FunctionPtr>::ABI, Abi::Rust);
}

#[test]
fn extern_c_fn() {
    type F = extern "C" fn(i32) -> i32;

    assert_type_eq_all!(<F as FunctionPtr>::Args, (i32,));
    assert_type_eq_all!(<F as FunctionPtr>::Output, i32);

    assert_eq!(<F as FunctionPtr>::ARITY, 1);
    assert!(<F as FunctionPtr>::SAFE);
    assert!(<F as FunctionPtr>::EXTERN);
    assert_eq!(<F as FunctionPtr>::ABI, Abi::C);
}

#[test]
fn zero_arg_fn() {
    type F = fn() -> i32;

    assert_type_eq_all!(<F as FunctionPtr>::Args, ());
    assert_type_eq_all!(<F as FunctionPtr>::Output, i32);

    assert_eq!(<F as FunctionPtr>::ARITY, 0);
    assert!(<F as FunctionPtr>::SAFE);
    assert!(!<F as FunctionPtr>::EXTERN);
    assert_eq!(<F as FunctionPtr>::ABI, Abi::Rust);
}

#[test]
fn multi_arg_fn() {
    type F = fn(i32, i32, i32) -> i32;

    assert_type_eq_all!(<F as FunctionPtr>::Args, (i32, i32, i32));
    assert_type_eq_all!(<F as FunctionPtr>::Output, i32);

    assert_eq!(<F as FunctionPtr>::ARITY, 3);
    assert!(<F as FunctionPtr>::SAFE);
    assert!(!<F as FunctionPtr>::EXTERN);
    assert_eq!(<F as FunctionPtr>::ABI, Abi::Rust);
}

#[test]
fn no_ret() {
    type F = fn(i32);

    assert_type_eq_all!(<F as FunctionPtr>::Args, (i32,));
    assert_type_eq_all!(<F as FunctionPtr>::Output, ());

    assert_eq!(<F as FunctionPtr>::ARITY, 1);
    assert!(<F as FunctionPtr>::SAFE);
    assert!(!<F as FunctionPtr>::EXTERN);
    assert_eq!(<F as FunctionPtr>::ABI, Abi::Rust);
}
