use std::time::Duration;
use futures::executor::block_on;

use async_std::task::sleep;
use futures::join;


async fn count_to_ten_seconds(thread_num:usize) {
    for i in 1..10 {
        println!("{} {}", thread_num, i);

        // This is async sleep.
        // While thread is sleeping, other async tasks can be performed.
        sleep(Duration::from_millis( 1000 )).await;
    }
}

async fn my_async_function(){
    let t1=count_to_ten_seconds(1);
    let t2=count_to_ten_seconds(2);

    join!(t1, t2);
}

fn main() {
    println!("Code Starts");

    // We perform a blocking call on async function
    // In this way, we can call async function in synchronous way.
    block_on(my_async_function());

    println!("Code Finishes");

}