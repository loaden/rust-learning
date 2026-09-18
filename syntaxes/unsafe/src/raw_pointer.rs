pub fn raw_pointer_test() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        *r2 = 10;
        println!("{}", *r1);
    }
}