// Find all our documentation at https://docs.near.org
use near_sdk::{log, near};

const PUZZLE_NUMBER: u8 = 1;

// Define the contract structure
#[near(contract_state)]
pub struct Contract {
    crossword_solution: String,
}

// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        Self {
            crossword_solution: "Hello".to_string(),
        }
    }
}

// Implement the contract structure
#[near]
impl Contract {
    // Public method - returns the puzzle number
    pub fn get_puzzle_number(&self) -> u8 {
        PUZZLE_NUMBER
    }

    // Public method - accepts a soulution, such as "howdy", and records it
    pub fn set_solution(&mut self, solution: String) {
        log!("Saving solution: {solution}");
        self.crossword_solution = solution;
    }

    pub fn guess_solution(&mut self, solution: String) -> bool {
        if solution == self.crossword_solution {
            log!("You guessed right!");
            true
        } else {
            log!("Try again.");
            false
        }
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_puzzle_number() {
        let contract = Contract::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_eq!(contract.get_puzzle_number(), 1);
    }

    #[test]
    fn incorrect_guess() {
        let mut contract = Contract::default();
        assert_eq!(contract.guess_solution("Hellox".to_string()), false);
    }

    #[test]
    fn correct_guess() {
        let mut contract = Contract::default();
        contract.set_solution("Hello".to_string());
        assert_eq!(contract.guess_solution("Hello".to_string()), true);
    }
}
