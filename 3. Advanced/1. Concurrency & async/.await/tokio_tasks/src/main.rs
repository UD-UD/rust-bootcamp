use std::thread::sleep;
use std::time::Duration;

use rand::random;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut handles = vec![];
    first(0).await;

    println!("------------------------");
    let f2 = first(1);
    // expensive_task(); // this will block all futures in current thread

    // handles.push(tokio::spawn(async { expensive_task() })); // this will block all futures in current thread if flavor is current thread

    handles.push(tokio::spawn(async { // this will not block, it will keep things in a separate thread pool
        let _res = tokio::task::spawn_blocking(|| {
            expensive_task();
        }).await;
    }));

    println!("AWAITING");
    f2.await;

    for i in 3..4 {
        let handel = tokio::spawn(async move {
            first(i).await;
        });
        handles.push(handel);
    }
    println!("---- AWAITING AGAIN ----");
    for handle in handles {
        handle.await.unwrap()
    }
}

async fn first(i: i32) {
    println!("[{i}] First Value");
    println!("{}", get_val().await);
    println!("[{i}] Second Value");
    println!("{}", get_val().await);
}

async fn get_val() -> String {
    sleep(Duration::from_millis(get_millis()));
    "Ujjal Dutta".to_owned()
}

fn get_millis() -> u64 {
    let rand = random::<u8>();
    println!("Random millis is {}", rand);
    100 as u64
}

fn expensive_task() {
    let mut k = 0;
    for i in 0..400_000_000 {
        k = k + 1;
    }
    println!("Expensive task is done, avlue is {}", k);
}

/**
Futures are 0 runtime cost. They will not run until we use them.
They are easy to cancel, all we need to do is stop polling the future.

Async Await is good for IO ops
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
