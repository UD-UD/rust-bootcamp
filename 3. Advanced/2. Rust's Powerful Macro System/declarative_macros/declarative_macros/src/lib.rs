/*
    To define a macro we use the following syntax.

    #[macro_export] // exports a macro
    macro_rules! name {
        rule0 :
        rule1 :
        ...
        ruleN :
    }

     format for rules : ( matcher ) => { expansion aka transcriber }
     Rules are evaluated from top down and the first matching rule is executed.

     #[macro_use] will let it escape to the parent context, including all submodules from there.

     Macros to be imported with #[macro_use] must be exported with #[macro_export], which is described below.

     NOTE: Good book on macros : https://veykril.github.io/tlborm/introduction.html
*/
#[macro_export]
macro_rules! hello_world {
    () => {
        println!("Hello World!")
    };
}

#[macro_export]
macro_rules! map {
    // match should be of the following format $IDENTIFIER_NAME:FRAGMENT-SPECIFIER
    // List for fragment specifier  : https://veykril.github.io/tlborm/decl-macros/minutiae/fragment-specifiers.html
    ($key:ty,$value:ty) => {
        {
            let map:HashMap<$key,$value> = HashMap::new();
            map
        }
    };
    // $( ... ) sep rep
    ($($key:expr => $value:expr) , * ) => {
        {
            let mut map= HashMap::new();
            $(map.insert($key,$value);)*
            map
        }
    };
}

#[macro_export]
macro_rules! vec2 {
    [] => { 
        {
            let mut vecs = Vec::new();
            vecs
        }
    };
    [$($a:expr),+ $(,)?] => {
        {
            let mut vecs = Vec::new();
            $(vecs.push($a);) + 
            vecs
        }
    };
    [$m:expr; $n:expr] => {
        {
            let mut vecs = Vec::new();
            vecs.resize($n, $m);
            vecs
        }
    };
}