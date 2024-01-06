use std::env;

use crate::synacors::{math::solve_equation, vm::VM};
use std::fs;

mod synacors;
fn main() {
    solve_equation();
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Please provide a path to a binary file to run");
    }
    solve_equation();
    let file = fs::read(&args[1]).unwrap();
    let mut vm = VM::new(&file);
    println!("{:?}", vm.memory[0..10].to_vec());
    //vm.debug = true;
    vm.run();
}
