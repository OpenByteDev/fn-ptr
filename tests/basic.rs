use fn_ptr::{Abi, FnPtr, abi, arity, is_extern, is_safe, is_unsafe};

use static_assertions::assert_type_eq_all;

#[test]
fn unsafe_fn() {
    type F = unsafe fn(i32) -> i32;

    assert_type_eq_all!(<F as FnPtr>::Args, (i32,));
    assert_type_eq_all!(<F as FnPtr>::Output, i32);

    assert_eq!(arity::<F>(), 1);
    assert!(is_unsafe::<F>());
    assert!(!is_extern::<F>());
    assert_eq!(abi::<F>(), Abi::Rust);
}

#[test]
fn extern_c_fn() {
    type F = extern "C" fn(i32) -> i32;

    assert_type_eq_all!(<F as FnPtr>::Args, (i32,));
    assert_type_eq_all!(<F as FnPtr>::Output, i32);

    assert_eq!(arity::<F>(), 1);
    assert!(is_safe::<F>());
    assert!(is_extern::<F>());
    assert_eq!(abi::<F>(), Abi::C);
}

#[test]
fn zero_arg_fn() {
    type F = fn() -> i32;

    assert_type_eq_all!(<F as FnPtr>::Args, ());
    assert_type_eq_all!(<F as FnPtr>::Output, i32);

    assert_eq!(arity::<F>(), 0);
    assert!(is_safe::<F>());
    assert!(!is_extern::<F>());
    assert_eq!(abi::<F>(), Abi::Rust);
}

#[test]
fn multi_arg_fn() {
    type F = fn(i32, i32, i32) -> i32;

    assert_type_eq_all!(<F as FnPtr>::Args, (i32, i32, i32));
    assert_type_eq_all!(<F as FnPtr>::Output, i32);

    assert_eq!(arity::<F>(), 3);
    assert!(is_safe::<F>());
    assert!(!is_extern::<F>());
    assert_eq!(abi::<F>(), Abi::Rust);
}

#[test]
fn no_ret() {
    type F = fn(i32);

    assert_type_eq_all!(<F as FnPtr>::Args, (i32,));
    assert_type_eq_all!(<F as FnPtr>::Output, ());

    assert_eq!(arity::<F>(), 1);
    assert!(is_safe::<F>());
    assert!(!is_extern::<F>());
    assert_eq!(abi::<F>(), Abi::Rust);
}
