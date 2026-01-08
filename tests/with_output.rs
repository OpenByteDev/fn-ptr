use fn_ptr::with_output;

use static_assertions::assert_type_eq_all;

#[test]
fn with_output_changes_return_type_preserving_safety_and_abi() {
    type F = unsafe extern "C" fn(i32) -> f64;
    assert_type_eq_all!(with_output!(u64, F), unsafe extern "C" fn(i32) -> u64);
}

#[test]
fn with_output_on_rust_fn_pointer() {
    type F = fn(i32) -> i32;
    assert_type_eq_all!(with_output!((), F), fn(i32) -> ());
}
