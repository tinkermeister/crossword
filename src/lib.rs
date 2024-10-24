use near_sdk::{env, log, near};

#[near(contract_state)]
#[derive(Default)]
pub struct Contract {
    crossword_solution: String,
}

#[near]
impl Contract {
    #[init]
    pub fn new(solution: String) -> Self {
        Self {
            crossword_solution: solution,
        }
    }

    pub fn get_solution(&self) -> String {
        self.crossword_solution.clone()
    }

    pub fn set_solution(&mut self, solution: String) {
        log!("Saving solution: {solution}");
        self.crossword_solution = solution;
    }

    pub fn guess_solution(&self, solution: String) -> bool {
        let hashed_input = env::sha256(solution.as_bytes());
        let hashed_input = hex::encode(&hashed_input);

        if hashed_input == self.crossword_solution {
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
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{env, testing_env, AccountId};

    #[test]
    fn incorrect_guess() {
        let contract = Contract::new("Hello".to_string());
        assert_eq!(contract.guess_solution("Hellox".to_string()), false);
    }

    #[test]
    fn debug_get_hash() {
        testing_env!(VMContextBuilder::new().build());
        let debug_solution = "near nomicon ref finance";
        let debug_hash_bytes = env::sha256(debug_solution.as_bytes());
        let debug_hash_string = hex::encode(debug_hash_bytes);
        println!("Let's debug: {:?}", debug_hash_string);
    }

    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    #[test]
    fn check_guess_solution() {
        let alice: AccountId = "alice.testnet".parse().unwrap();
        let context = get_context(alice);
        testing_env!(context.build());

        let contract = Contract::new(
            "69c2feb084439956193f4c21936025f14a5a5a78979d67ae34762e18a7206a0f".to_string(),
        );

        let guess_result = contract.guess_solution("wrong answer".to_string());
        assert!(!guess_result, "Expected a failure from a wrong guess");
        assert_eq!(get_logs(), ["Try again."], "Expected a failure log.");

        let guess_result = contract.guess_solution("near nomicon ref finance".to_string());
        assert!(guess_result, "Expected the correct answer to return true");
        assert_eq!(
            get_logs(),
            ["Try again.", "You guessed right!"],
            "Expected a successful log after the previous failure log."
        );
    }
}
