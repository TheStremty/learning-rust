static mut COUNTER: u32 = 0;


unsafe fn dangerous() {
    let mut secret_value = 42;

    let r1 = &secret_value as *const i32;
    let r2 = &mut secret_value as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        *r2 = 69;
        println!("r1 is {}", *r1)
    }
}

fn add_to_counter(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    unsafe { dangerous(); }

    add_to_counter(3);
    add_to_counter(5);

    println!("Counter: {}", unsafe { COUNTER });

}