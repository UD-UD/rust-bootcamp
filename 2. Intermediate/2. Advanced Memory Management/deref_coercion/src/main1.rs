use std::ops::{Deref, DerefMut};

fn run() {
    let s = Box::new("Hello Ujjal".to_owned());
    print(&s); // here deref coercion happening &Box -> &String -> &str, this happens only when passing references to functions or methods.


    let s2 = MySmartPointer::new(Box::new("Hello Ujjal".to_owned()));
    print(&s2);

    let s3 = &(*s2);
    let s4 = &(**s2);
    let s5 = &(***s2);

    // No performance overhead as the derefs are handled at compile time.
}

fn print(s: &str) {
    println!("{s}");
}

struct MySmartPointer<T> {
    value: T,
}

impl<T> MySmartPointer<T> {
    fn new(value: T) -> Self {
        MySmartPointer {
            value
        }
    }
}

impl<T> Deref for MySmartPointer<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for MySmartPointer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}