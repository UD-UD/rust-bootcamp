fn are_both_true<T, U, V>(fuction1: T, function2: U, item: &V) -> bool
where
    T: Fn(&V) -> bool,
    U: Fn(&V) -> bool,
{
    fuction1(item) && function2(item)
}

fn are_both_true_funtion_pointer<V>(fuction1: fn(&V) -> bool, function2: fn(&V) -> bool, item: &V) -> bool
{
    fuction1(item) && function2(item)
}

fn less_than(x: &i32) -> bool {
    *x < 20
}

fn _main() {
    let greater_than = |x: &i32| *x > 10;
    // let less_than = |x: &i32| *x < 20;

    let result = are_both_true(greater_than, less_than, &12);
    let result2 = are_both_true_funtion_pointer(greater_than, less_than, &12);
    println!("{}", result);
}

/*
    Function pointers can be coerced to closures as long as they don't capture variables in there environment.
    for eg:
        let ten = 10;
        let greater_tan =|x: &i32| *x > 10;
    This function will not be coerced. 
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        _main()
    }
}