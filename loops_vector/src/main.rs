// Loops
// Rust currently provides three approaches to performing some kind of iterative activity. They are: loop, while and for. Each approach has its own set of uses.

#![allow(unused_variables)]
fn main() {
// let mut x = 5;
// let mut done = false; 

//     while !done {
//         x += x - 3;

//         println!("{}", x);

//         if x % 5 == 0 {
//             done = true;
//         }
//     }

// let arr = vec![10, 20, 30];
// for i in arr {
//     println!("{}", i);
// }
// let v = vec![1, 2, 3, 4, 5];

// for i in &v {
//     println!("This is a reference to {}", i);
// }

// for i in &v {
//     println!("This is a reference to {}", i);
// }


#![allow(unused_variables)]

fn foo(v: Vec<i32>) -> Vec<i32> {
    // Do stuff with `v`.
    for i in &v{
        let result = i*2;
        println!("{}", result);
    }
    // Hand back ownership.
    v
}
foo(vec![1, 2, 3, 4, 5]);
}
