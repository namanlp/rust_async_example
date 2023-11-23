use std::thread;


fn print_hello(){
    for i in 0..10 {
        println!("HELLO {}", i);
    }
}

fn main() {

    let t1 = thread::spawn(print_hello);
    let t2 = thread::spawn(print_hello);
    let t3 = thread::spawn(print_hello);
    let t4 = thread::spawn(print_hello);

    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();
    t4.join().unwrap();
}
