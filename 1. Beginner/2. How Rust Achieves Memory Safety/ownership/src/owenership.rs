
fn main(){
    let s1 = String::from("Rust"); // heap allocated string

    let s2 = s1; // s1 is moved to s2

    let s3 = s2.clone(); // s2 is cloned to s3
    println!("s1 is: {}", s2);

    let x = 10;
    let y = x;
    println!("x is: {}", s1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main_test() {
        main();
    }
}