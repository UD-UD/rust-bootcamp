fn main() {
    // Slices are references to a contiguous sequence of
    // elements in a collection.

    let tweet = String::from("This is my ");
    let trimmed_tweet: &str = trim_tweet(&tweet);

    let tweet2 = "This is my tweet and it's very very long";
    let trimmed_tweet2 = trim_tweet(tweet2);
    
    trimmed_tweet2

    println!("{trimmed_tweet}");
    println!("{trimmed_tweet2}");

    let a = [1, 2, 3, 4, 5, 6];
    let a_slice = &a[..3];
    println!("{:?}", a_slice);
}

fn trim_tweet(tweet: &str) -> &str {
    &tweet[..20]
}

/*
 String types:
 1. String
    let tweet = String::from("This is my ");
    Growable, heap allocated Strings (UTF-8 encoded)
    Has an addr, length and capacity
    used when string mutation is required
 2. &str
    - immutable sequence of UTF-8 bytes somewhere in memory (stack, heap or static memory)
    - Handle behind a reference 
        has addr and length
    - used only when we need a string snapshot/view
    - should be used for string manipulation if original string mutation is not required.
    
    String -> &str : dref coercion
    
    let a = [1,2,3,4,5];
    let a_slice = &a[..3];
    println!("{:?}",a_slice);
 */

