# rustman

A commandline game implemented using Rust. First time using Rust, interesting concepts learned: 

Passing a struct by value removes it from the program (why?). 
A struct can only have ONE mutable reference of it in a single execution thread. Which is a neat concept, but unlimited unmutable references. 