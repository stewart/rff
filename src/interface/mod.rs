pub struct Interface {
    choices: Vec<String>
}

impl Interface {
    /// Creates a new Interface with the provided input choices.
    pub fn with_choices(choices: Vec<String>) -> Interface {
        Interface {
            choices: choices
        }
    }

    // Starts the interface
    pub fn run(&mut self) {
        println!("{} choices to search through", self.choices.len());
    }
}
