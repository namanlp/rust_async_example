use tokio::join;

async fn say_hello(){
    println!("Hello World");
}

#[tokio::main]
async fn main() {

    let t1 = say_hello();
    let t2 = say_hello();

    // Nothing happens till here
    // Because we have not called await method yet

    println!("Async Main function executes first\n");

    // Join them to actually execute them
    join!(t1, t2);
}
