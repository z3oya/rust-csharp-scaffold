#[used]
#[allow(clippy::type_complexity)]
static KEEP_EXPORTS: (
    extern "C" fn() -> i32,
    extern "C" fn(i32, i32) -> i32,
    extern "C" fn() -> usize,
    extern "C" fn(usize, i32) -> i32,
    unsafe extern "C" fn(usize, i64, *mut i64) -> i32,
    extern "C" fn(usize) -> i32,
) = (
    scaffold_calc::scaffold_calc_abi_version,
    scaffold_calc::scaffold_calc_add,
    scaffold_calc::scaffold_calc_engine_create,
    scaffold_calc::scaffold_calc_engine_set_factor,
    scaffold_calc::scaffold_calc_engine_apply,
    scaffold_calc::scaffold_calc_engine_destroy,
);

unsafe extern "C" {
    fn scaffold_calc_abi_version() -> i32;
    fn scaffold_calc_add(a: i32, b: i32) -> i32;
    fn scaffold_calc_engine_create() -> usize;
    fn scaffold_calc_engine_set_factor(handle: usize, factor: i32) -> i32;
    fn scaffold_calc_engine_apply(handle: usize, input: i64, out: *mut i64) -> i32;
    fn scaffold_calc_engine_destroy(handle: usize) -> i32;
}

#[test]
fn abi_version_is_current() {
    let version = unsafe { scaffold_calc_abi_version() };
    assert_eq!(version, 1);
}

#[test]
fn add_returns_sum() {
    let sum = unsafe { scaffold_calc_add(2, 3) };
    assert_eq!(sum, 5);
}

#[test]
fn engine_lifecycle_works() {
    let handle = unsafe { scaffold_calc_engine_create() };
    assert_ne!(handle, 0);

    let rc = unsafe { scaffold_calc_engine_set_factor(handle, 3) };
    assert_eq!(rc, 0);

    let mut acc: i64 = -1;
    let rc = unsafe { scaffold_calc_engine_apply(handle, 2, &mut acc) };
    assert_eq!(rc, 0);
    assert_eq!(acc, 6);

    let rc = unsafe { scaffold_calc_engine_apply(handle, 2, &mut acc) };
    assert_eq!(rc, 0);
    assert_eq!(acc, 12);

    let rc = unsafe { scaffold_calc_engine_destroy(handle) };
    assert_eq!(rc, 0);
}

#[test]
fn engine_rejects_unknown_handle() {
    let rc = unsafe { scaffold_calc_engine_set_factor(999_999, 2) };
    assert_eq!(rc, -1);

    let rc = unsafe { scaffold_calc_engine_destroy(999_999) };
    assert_eq!(rc, -1);
}

#[test]
fn engine_apply_rejects_null_out() {
    let handle = unsafe { scaffold_calc_engine_create() };
    assert_ne!(handle, 0);
    let rc = unsafe { scaffold_calc_engine_apply(handle, 1, std::ptr::null_mut()) };
    assert_eq!(rc, -2);
    let rc = unsafe { scaffold_calc_engine_destroy(handle) };
    assert_eq!(rc, 0);
}
