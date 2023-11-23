use std::collections::VecDeque;
use std::thread;
use std::time::Duration;


fn print_hello(num : usize){
    println!("Executing thread number : {}", num);

    // Thread gets blocked after 10 second,
    // hence the execution passes to other threads.

    // Current thread is sent to waiting queue.
    // Read Operating Systems for more information.
    thread::sleep(Duration::from_millis(10_000));
}

fn main() {

    let mut thread_queue = VecDeque::new();

    for i in 0..100 {

        // Push Thread to Queue
        thread_queue.push_back(
            thread::spawn(move || {
                print_hello(i);
            })
        )
    }

    // Join for each thread in the queue
    while thread_queue.len() > 0 {
        println!("Pulled a thread");
        thread_queue.pop_front().unwrap().join().unwrap();
    }
}
