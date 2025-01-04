fn main() {
    let mut v = Box::new(vec![1, 2, 3]);
    let ptr = v.as_mut_ptr();
    unsafe {
        *ptr = 4; // Modify the first element
    }
    println!("{:?}", *v); // Output: [4, 2, 3]
    //No undefined behavior here, Box is freed at the end of scope
    //Since v is a Box, the pointer is still valid until the end of this scope.
}