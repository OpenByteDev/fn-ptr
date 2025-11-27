#![allow(unpredictable_function_pointer_comparisons)]

use fn_ptr::{Abi, FnPtr, SafeFnPtr, UnsafeFnPtr, abi, arity, is_extern, is_safe, is_unsafe};

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

#[test]
fn addr_and_from_addr() {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    type F = fn(i32, i32) -> i32;
    let f: F = add;

    let addr = f.addr();
    assert_ne!(addr, 0);

    let f2: F = unsafe { F::from_addr(addr) };
    assert_eq!(f2.addr(), addr);
    assert_eq!(f, f2);
}

#[test]
fn as_ptr_and_from_ptr() {
    unsafe fn mul(a: i32, b: i32) -> i32 {
        a * b
    }

    type F = unsafe fn(i32, i32) -> i32;
    let f: F = mul;

    let ptr = f.as_ptr();
    assert!(!ptr.is_null());

    let f2: F = unsafe { F::from_ptr(ptr) };
    assert_eq!(f2.as_ptr(), ptr);
    assert_eq!(f, f2);
}

#[test]
fn invoke_safe_fnptr() {
    fn square(x: i32) -> i32 {
        x * x
    }

    type F = fn(i32) -> i32;
    let f: F = square;

    assert_eq!(f.invoke((5,)), 25);
    assert_eq!(f.invoke((0,)), 0);
}

#[test]
fn invoke_unsafe_fnptr() {
    unsafe fn negate(x: i32) -> i32 {
        -x
    }

    type F = unsafe fn(i32) -> i32;
    let f: F = negate;

    unsafe {
        assert_eq!(f.invoke((10,)), -10);
        assert_eq!(f.invoke((0,)), 0);
    }
}
