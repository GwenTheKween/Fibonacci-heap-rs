use std::io;
use heap::FibHeap;
mod heap;

fn print_menu() {
    println!("Welcome to Rusted Fibonacci Heap. My implementation of a bad structure type");
}

fn print_options () {
    println!("Please choose an option to proceed!
    1. insert an element;
    2. remove an element;
    3. print heap;
    4. restart heap;
    0. exit");
}

fn read_input() -> String{
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    input.trim().to_string()
}

fn insert(h: &mut FibHeap){
    println!("Please enter element to be inserted");
    match read_input().parse() {
        Ok(n) => h.insert(n),
        Err(e) => println!("there was an error: {}", e),
    }
}

fn remove(){
    println!("not yet implemented");
}

fn print_heap(h: &FibHeap){
    h.print_heap()
}

fn restart(){
    println!("not yet implemented");
}

fn main(){
    let mut h = FibHeap::new();
    print_menu();
    loop{
        print_options();
        let option;
        match read_input().parse() {
            Ok(n) => option = n,
            Err(_) => option = -1,
        }

        match option {
            1 => insert(&mut h),
            2 => remove(),
            3 => print_heap(&h),
            4 => restart(),
            _ => break,
        }
    }
    return;
}
