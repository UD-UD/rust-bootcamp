use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for i in 0..5 {
            println!("Spawn thread: {i}");
        }
    });

    for i in 0..10 {
        println!("Main Thread: {i}");
    }

    let dimmy_s = "Ujjal is great".to_owned();

    let handle_again = thread::spawn(move || { // moving values into threads
        println!("{dimmy_s}");
    });

    handle.join().unwrap();
    handle_again.join().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn texts_main() {
        main();
    }
}