use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel::<String>();
    let sentences = [
        "!dlroW olleH".to_owned(),
        ".tsurT eW tsuR nI".to_owned(),
        "!ytsuR teG s'teL".to_owned(),
        "!tsuB ro tsuR".to_owned(),
    ];

    for sentence in sentences {
        let tr = tx.clone();
        thread::spawn(move || {
            let reverse_sentence: String = sentence.chars().rev().collect();
            tr.send(reverse_sentence).unwrap();
        });
    }

    // rx.recv();
    drop(tx);
    for sentence in rx {
        println!("{sentence}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_main() {
        main()
    }
}