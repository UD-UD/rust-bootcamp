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

fn understand_mut_not_mut(){
    
    let x = 100; // x is an immutable variable
    // x = 40; this will throw error as x being immutable, it value cannot be changes
    
    
    let y = &x; //  y is an immutable variable holding an immutable reference to an immutable variable x;
    // y = &z ; will throw an error as y is immutable
    // *y = 100 ; will throw an error as *y (a.k.a) x is also immutable
    
    let mut a = 100; // mutable variable a;
    a = 200; // we can change the variable of al
    
    let b = &a; // b is an immutable variable holding an immutable reference to a mutable variable a.
    // b = &x // this will throw an error
    
    // *b = 300 // this will also throw an error as we are modifying a value using an immutable reference 
    
    let b = &mut a;
    // b = &x; // this is throw an error as b is immutable
    // b = &mut x; // this is throw an error as b is immutable
    
    *b = 300; //this (*b is actually the value of a) will work as b is holding a mutable reference to a;
    
    let c = 100;
    let mut d = &c; // d is mutable variable holding an immutable reference to an immutable variable.
    // *d = 100 // this will not work as we cannot change value of an immutable variable
    d = &y; // Possible because d is mutable
    
    let mut i = 100;
    i = 200; // Possible
    
    let mut j = &i; // j is mutable variable holding an immutable reference to a mutable variable
    // *j =300; // Not possible as reference in immutable
    j = &d; // Posible as j is iself mutable
    
    let mut k = &mut i; // k is a mutable variable holding a mutable reference to a mutable variable
    *k = 400;
    k = b;   // this would work as k being mutable can be changed to another variable of same type
    
    
}   