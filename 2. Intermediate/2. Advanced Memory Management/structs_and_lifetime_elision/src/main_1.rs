struct Tweet<'a> {
    content: &'a str,
}

// 1. Each parameter that is a reference gets its own lifetime parameter.
// 2. If there is exactly one input lifetime parameter, that lifetime
//    is assigned to all output lifetime parameters.
// 3. If there are multiple input lifetime parameters, but one of them is
//    &self or &mut self, the lifetime of self is assigned to all output
//    lifetime parameters.

// parameters have input lifetime
// return value has output lifetime

impl<'a> Tweet<'a> {
    fn replace_content(&mut self, content: &'a str) -> &'a str {
        let old_content = self.content;
        self.content = content;
        old_content
    }

    // NOTE: the output lifetime is the same as the input lifetime due to the Rule 2 above
    // fn replace_content(&mut self, content: &'a str) -> &str {
    //     let old_content = self.content;
    //     self.content = content;
    //     old_content
    // }
}

fn run() {
    let mut tweet = Tweet {
        content: "example",
    };

    let old_content = tweet.replace_content("replace_example");
    println!("{old_content}");
}


fn take_return_content<'a, 'b>(content: &'a str, content2: &'b str) -> &'a str {
    content
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        run()
    }
}