use fn_ptr::{make_safe, make_unsafe};

use static_assertions::assert_type_eq_all;

#[test]
fn make_safe() {
    type UnsafeF = unsafe fn(i32) -> i32;
    type SafeF = fn(i32) -> i32;

    assert_type_eq_all!(make_safe!(UnsafeF), SafeF);
}

#[test]
fn make_unsafe() {
    type UnsafeF = unsafe fn(i32) -> i32;
    type SafeF = fn(i32) -> i32;

    assert_type_eq_all!(make_unsafe!(SafeF), UnsafeF);
}
