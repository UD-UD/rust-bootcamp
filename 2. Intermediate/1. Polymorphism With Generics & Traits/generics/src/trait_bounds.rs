/*
This line imports the Add trait from the standard library's ops module.
The Add trait is used to overload the + operator for addition operations.
*/
use std::ops::Add;

struct Pair<T>(T, T);

/*
This line specifies trait bounds for the generic type T (covered in Trait Bounds post here).
`T: Add` means T must implement the Add trait.
This is necessary because we want to be able to add two instances of T together in the add method.

`T: Copy` is another trait bound indicating that T should be a type that implements the Copy trait.
This trait is required because the add method will implicitly copy the values when performing the addition,
as Rust by default moves values unless they implement the Copy trait.
*/
impl<T> Pair<T> where T: Add<Output=T> + Copy
{
    fn add(&self) -> T {
        self.0 + self.1
    }
}

fn main3() {
    let p1 = Pair(10, 23);
    let addition = p1.add();
    assert_eq!(addition, 33);
}

fn take_and_give_ownership<T>(input: T) -> T { // Correct way of defining generic function
    input
}

struct User {
    name: String,
    id: u32,
}

fn main() {
    let str1 = String::from("Ferris the 🦀!");
    let user1 = User {
        name: "Alice".to_string(),
        id: 199,
    };
    let _str2 = take_and_give_ownership(str1);
    let _user2 = take_and_give_ownership(user1);
}
