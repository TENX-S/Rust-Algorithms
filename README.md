# Rust & Algorithms
Learning "Introduction to Algorithms" with Rust. 

If you want to see specific code of these algorithms, go to /src/bin/algorithms

If you want to check that the algorithm you implemented is correct or not，such like checking your sort algorithm, you need to define your function signature as `fn your_algorithm(&mut Vec<i32>)` since Rust does not have function generic parameters,
and all other kinds of function signatures will be explained in detail in /src/bin/tests.rs