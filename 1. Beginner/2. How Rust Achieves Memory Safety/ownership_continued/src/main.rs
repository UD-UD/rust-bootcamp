mod good_one;

fn main() {
    let s1 = String::from("Rust"); // heap allocated string
    let s2 = s1.clone();
    print_string(s1.clone());
    let s3 = generate_string();
    let s4 = add_to_string(s2);

    println!("s1 is: {s1}");
    println!("s3 is: {s3}");
    println!("s4 is: {s4}");

    let x = 10;
    let y = x;
    print_integer(x);
    println!("x is: {x}");
}

fn print_integer(i: i32) {
    println!("i is: {i}");
}

fn add_to_string(mut p1: String) -> String {
    p1.push_str(" is awesome!");
    p1
}

fn generate_string() -> String {
    String::from("Ferris")
}

fn print_string(p1: String) {
    println!("{p1}");
} // p1 is dropped


/*

Always Valid: References must always point to valid memory.
Prevents: Dangling references and null pointer dereferences.

One or the Other: You can't have a mutable reference (&mut T) when there's an immutable reference (&T) to the same data.
Prevents: Data being unexpectedly modified while it's being read.

One Mutable at a Time: Only one mutable reference (&mut T) to the same data in a scope.
Prevents: Data races, where two threads might modify the same data concurrently.

Match Lifetimes: References can't outlive the data they refer to.
Prevents: Using data that has been freed, leading to undefined behavior.

Compile-Time Lifetimes: Lifetimes of references must be determinable at compile time.
Ensures: Memory safety is maintained across different parts of the code.

 */


/*
    Objective: the difference between `let mut x = ...` , `let x = &mut ...` and `let mut x = &mut ...`
    
    For anyone who doesn't understand:
    
    let mut x = .... // create a mutable variable called x, the value that x represents can be changed, e.g.:
    
    let mut x = 1;
    x = 2; // x is mutable, so you can change its value
    
    let x = &mut ... // create a non-mutable variable called x which is a mutable reference to ..., ... must also be mutable, e.g:
    
    let mut y = 100; // y is mutable
    let x = &mut y; // x points to y, but cannot be changed/reassigned. It will always point to y for the whole of it's lifetime. The mut means that I've got a mutable borrow on y, so no other borrows are allowed on y.
    *x = 200; // x is dereferenced, i.e. the value of y, and the data that it is pointing to is changed. This is possible because x is a mutable borrow;
    println!("{y}"); // y is now 200, because the value was changed in the line above
    
    let mut x = &mut ... = Similar to let x = &mut y, except this time x can be reassigned and point to a different &mut ..., e.g.:
    
    let mut y = 100;
    let mut z = 200;
    
    let mut x = &mut y; // x is now a mutable pointer to y
    *x = 200; // we are using x to change the value of y
    println!("{y}"); // prints 200, the new value of y
    x = &mut z; // we are reassigning x to now point mutably to z
    *x = 300; // we are using x to change the value of z
    println!("{z}"); // prints 300, the new value of z
 */

fn strings(){
    
    /*
    &str vs String
    
     */
    
    let s1 = "abcdfg ";
    let s2 = String::from("hello");
    
    let s3 = "abcgdgdj".to_string();
    let s4 = "afgdfggd".to_owned();
    let s5 = &s4[..];
    
    let mut s56 = String::from("foo");
    s56.push_str("hllo");
    
    let s44 = s3 + &s2; // s3 is moved at this point
    println!("{}",s2);
    
    let s33 = s1[0]; // cannot index over a string
    let s332 = &s1[0..3]; // you can create a sub
    
    // iterating over a string or &s is of liner tine and not constant time
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn strings_stest() {
        strings();
    }
}