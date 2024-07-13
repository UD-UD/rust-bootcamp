use std::collections::HashMap;

use declarative_macros::{hello_world, map};

fn main() {
    hello_world!(); // these parenthesis could be any grouping tokens hello_world![] also works
    vec![1, 2, 3]; // vec macro uses [] grouping tokens because it makes sense to put a list

    let mut m = map!(String,i64);
    m.insert("Ujjal".to_owned(), 500000000000);

    // expands rust syntax
    let m2: HashMap<String, i64> = map!(
        "Ujjal".to_owned() => 500000000000,
        "Dutta".to_owned() => 500000000000
    );

    let mm = vec![5; 10];
}


#[cfg(test)]
mod tests {
    use declarative_macros::vec2;

    #[test]
    fn creates_empty_vector() {
        let first: Vec<i32> = vec![];
        let second: Vec<i32> = vec2![];
        assert_eq!(first, second);
    }

    #[test]
    fn creates_vector_from_list() {
        assert_eq!(vec![1, 2, 3], vec2![1, 2, 3,]);
        assert_eq!(vec!['a', 'b', 'b', 'a'], vec2!['a', 'b', 'b', 'a']);
    }

    #[test]
    fn creates_vector_with_repeating_element() {
        assert_eq!(vec![5; 10], vec2![5;10]);
    }
}
