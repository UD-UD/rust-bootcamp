trait UIComponent {
    fn render(&self) {
        println!("Rendering component...");
    }
}

struct Button {
    text: String,
}

impl UIComponent for Button {}

struct Container {
    name: String,
    child: Box<Container>,
}

impl UIComponent for Container {}

fn main() {
    let button_a = Button { text: "button a".to_owned() }; // Button is on the stack
    let button_b = Box::new(Button { text: "button b".to_owned() }); // Button is on the Heap

    let button_c = button_a; // while moving, the entire memory is copied
    let button_d = button_b; // Only reference is copied.

    let components: Vec<Box<dyn UIComponent>> = vec![
        Box::new(button_c),
        button_d,
    ];
}
