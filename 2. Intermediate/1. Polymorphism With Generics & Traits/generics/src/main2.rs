struct BrowserCommand<T> {
    name: String,
    payload: T,
}

impl<T> BrowserCommand<T> {
    fn new(name: String, payload: T) -> Self {
        BrowserCommand {
            name,
            payload,
        }
    }

    fn get_payload(&self) -> &T {
        &self.payload
    }
}

impl BrowserCommand<String> {
    fn print_payload(&self) {
        println!("{}", self.payload);
    }
}

fn main() {
    let cmd1 = BrowserCommand {
        name: "navigate".to_owned(),
        payload: "asdsa".to_owned(),
    };

    let cmd1 = BrowserCommand::new("navidate".to_owned(), "ddasa".to_owned());

    let cmd2 = BrowserCommand {
        name: "navigate".to_owned(),
        payload: 200,
    };

    cmd1.print_payload();
    // cmd2.print_payload(); does not work because cmd2 is BrowCommand instance for type i32 which does not have a print payload impl

    let a = cmd1.get_payload();
    let b = cmd2.get_payload();
}

// No performance impact  due to monomorphisation, happens with Enums, Struct and impl blocks


