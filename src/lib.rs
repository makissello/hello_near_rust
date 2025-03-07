use near_sdk::{log, near};

#[near(contract_state)]
pub struct Contract {
    greeting: String,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            greeting: "Hello".to_string(),
        }
    }
}

#[near]
impl Contract {
    pub fn get_greeting(&self) -> String {
        self.greeting.clone()
    }

    pub fn set_greeting(&mut self, greeting: String) {
        log!("Saving greeting: {}", greeting);
        self.greeting = greeting;
    }
}
