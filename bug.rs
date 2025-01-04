fn main() {
    let mut v = vec![1, 2, 3];
    let ptr = v.as_mut_ptr();
    unsafe {
        *ptr = 4; // Modify the first element
    }
    println!("{:?}", v); // Output: [4, 2, 3]

    // ERROR: v is moved here and can not be used
    // The vector is deallocated when it goes out of scope, so accessing the pointer after that is undefined behaviour.
    drop(v);
    println!("{:?}", *ptr); // Undefined behavior
}