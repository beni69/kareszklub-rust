// https://doc.rust-lang.org/stable/std/primitive.pointer.html

pub fn run() {
    // unsafe {
    //     let _x = *(0xdeadbeef as *mut i32);
    // };

    let num = 1;
    dbg!(&num);
    dangerous(&num);
    dbg!(&num);
}

fn dangerous(num: &i32) {
    // mutate the value of num
    // *num = 0;

    // not unsafe
    let p: *mut i32 = num as *const i32 as *mut i32;
    println!("ref: {num:?}\tp: {p:?}");

    // unsafe { *p = 69420 };

    // let r: &mut i32 = unsafe { &mut *p };
    // *r = 420;
}
