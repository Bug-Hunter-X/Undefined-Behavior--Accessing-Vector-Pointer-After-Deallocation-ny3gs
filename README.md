# Rust Undefined Behavior Example

This repository demonstrates a common source of undefined behavior in Rust: accessing a pointer to data after the data has been deallocated.  The example uses a vector and its mutable pointer.  The solution showcases safer memory management techniques using `Box` for heap allocation and better lifecycle control.