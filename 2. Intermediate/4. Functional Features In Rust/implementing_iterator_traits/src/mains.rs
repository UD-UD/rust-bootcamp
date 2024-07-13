struct Person {
    first_name: String,
    last_name: String,
    occupation: String,
}

struct PersonIterator {
    values: Vec<String>,
}

impl Iterator for PersonIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.values.is_empty() {
            return None;
        }
        Some(self.values.remove(0))
    }
}

impl IntoIterator for Person {
    type Item = String;
    type IntoIter = PersonIterator;

    fn into_iter(self) -> Self::IntoIter {
        PersonIterator {
            values: vec![self.first_name, self.last_name, self.occupation]
        }
    }
}

pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();

    // match c.next() {
    //     None => String::new(),
    //     Some(first) => format!("{}{}", first.to_uppercase(), &input[1..])
    // }

    match c.next() {
        None => String::new(),
        Some(first) => format!("{}{}", first.to_uppercase(), c.as_str()) // Can use c from the scope
    }
}

pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    let mut cap: Vec<String> = Vec::new();
    // for word in words {
    //     cap.push(capitalize_first(word)) 
    // }

    // println!("{:?}", cap);
    // cap

    words.iter().map(|x| capitalize_first(x)).collect()
}

pub fn capitalize_words_string(words: &[&str]) -> String {
    let ss = capitalize_words_vector(words);
    // ss.iter().map(|x| x.chars()).flatten().collect()
    ss.join(" ")
}


fn main() {
    let p = Person {
        first_name: "Ujjal".to_string(),
        last_name: "Dutta".to_string(),
        occupation: "Software Engineer".to_string(),
    };


    // let mut i = p.into_iter();
    //
    // println!("{:?}", i.next());
    // println!("{:?}", i.next());
    // println!("{:?}", i.next());

    for item in p {
        println!("{}", item)
    }
    println!("{}", capitalize_first("bjjal"));

    capitalize_words_vector(&["djjal", "sutta"]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mains() {
        main();
    }
}