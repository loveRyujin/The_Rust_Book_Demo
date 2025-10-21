use std::slice;

fn main() {
    let mut num = 5;
    let r1 =&raw const num;
    let r2 = &raw mut num;

    unsafe {
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);
    }

    unsafe {
        noop();
    }

    let mut v = vec![1, 2, 3, 4];
    let (v1, v2) = split_as_mut(&mut v, 2);
    println!("v1: {:?}", v1);
    println!("v2: {:?}", v2);
}

unsafe fn noop() {}

fn split_as_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid)
        )
    }
}
