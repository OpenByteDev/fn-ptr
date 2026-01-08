use fn_ptr::with_abi;

use static_assertions::assert_type_eq_all;

#[test]
fn with_c_abi() {
    type F = unsafe fn(i32) -> String;
    assert_type_eq_all!(with_abi!("C", F), unsafe extern "C" fn(i32) -> String);
}

#[test]
fn with_system_abi() {
    type F = extern "C" fn(i32);
    assert_type_eq_all!(with_abi!("system", F), extern "system" fn(i32));
}

#[test]
fn with_rust_abi() {
    type F = extern "C" fn(i32);
    assert_type_eq_all!(with_abi!("Rust", F), fn(i32));
}
