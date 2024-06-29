fn main() {
    // Example 1
    let player1 = String::from("player 1");
    let player2 = String::from("player 2");

    let result = first_turn(player1.as_str(), player2.as_str());

    // How does the borrow checker know result is not a dangling reference?
    println!("Player going first is: {}", result);

    // Example 2
    let player1 = String::from("player 1");
    {
        let player2 = String::from("player 2");
        let result = first_turn(player1.as_str(), player2.as_str());
        println!("Player going first is: {}", result);
    }

    // Example 2
    // let player1 = String::from("player 1");
    // let result;
    // {
    //     let player2 = String::from("player 2");
    //     result = first_turn(player1.as_str(), player2.as_str()); // Player 2
    //
    // }
    // println!("Player going first is: {}", result);

    // Example 3
    let player1 = String::from("player 1");
    let result;
    {
        let player2 = String::from("player 2");
        result = first_turn(player1.as_str(), player2.as_str());
    }
    println!("Player going first is: {}", result);
}

fn first_turn<'a>(p1: &'a str, p2: &'a str) -> &'a str { // lifetime of return mavlue will be the minumum/ smallest of lifetimes passed in the parameter
    if rand::random() {
        p1
    } else {
        p2
    }
}

fn first_turna(p1: &str, p2: &str) -> &'static str { // lifetime of return mavlue will be the minumum/ smallest of lifetimes passed in the parameter
    let s: &'static str = "lets rock";
    s
}


// Lifetime of  references  is called generics lifetime annotation
// Static lifetime is one where the lifetime is defined for the lifetime of teh propgramm.
