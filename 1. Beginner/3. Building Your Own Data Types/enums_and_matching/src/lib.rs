use std::fmt::format;

#[derive(Debug)]
struct Product {
    name: String,
    product_category: ProductCategory,
    price: f32,
    in_stock: bool
}

#[derive(Debug)]
enum  ProductCategory{
    Books,
    Clothing,
    Electronics
}

fn main(){
    let category = ProductCategory::Books;
    let product  = Product{
        name: String::from("TV"),
        product_category:category,
        price:300.0,
        in_stock:true
    };
    
    println!("{:?}",product);
    
    let cmd1 = Command::Redo;
    let cmd2 = Command::Addtext(String::from("hello"));
    let cmd3 = Command::ModeCurcer(22,0);
    let cmd4 = Command::Replace {
        from: String::from("Hello"),
        to: String::from("bar")
    };
    
    println!("{}", cmd1.serialize());
    println!("{}", cmd2.serialize());
    println!("{}", cmd3.serialize());
    println!("{}", cmd4.serialize());
    
    let age = 35;
    match age {
        1 => println!("Hello"),
        13..=19 => println!("World"),
        x => println!("{}",x),
        _ => println!(""),
    }
}

impl Command {
    fn serialize(&self) -> String {
        let json_match = match self {
            Command::Undo => String::from("Command : Undo") ,
            Command::Redo=> String::from("Command : Redo"),
            Command::Addtext(s) => format!("Add string {s}"),
            Command::ModeCurcer(a, b) => format!("Move Curser to {a} {b}"),
            Command::Replace { from , to } => format!("Replace {from} to {to}")
        };
        json_match
    }
}


enum Command {
    Undo,
    Redo,
    Addtext(String),
    ModeCurcer(i32,i32),
    Replace{
        from: String,
        to: String
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_product() {
        main()
    }
}