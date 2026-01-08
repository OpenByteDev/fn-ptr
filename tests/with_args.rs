use fn_ptr::with_args;

use static_assertions::assert_type_eq_all;

#[test]
fn with_args_changes_args_preserving_output_safety_and_abi() {
    type F = unsafe extern "C" fn(i32) -> f64;
    assert_type_eq_all!(
        with_args!((u8, u16), F),
        unsafe extern "C" fn(u8, u16) -> f64
    );
}

#[test]
fn with_args_to_unit_args() {
    type F = extern "system" fn(i32);
    assert_type_eq_all!(with_args!((), F), extern "system" fn());
}

#[test]
fn with_args_to_singleton_tuple() {
    type F = fn(i32) -> i32;
    assert_type_eq_all!(with_args!((u64,), F), fn(u64) -> i32);
}
