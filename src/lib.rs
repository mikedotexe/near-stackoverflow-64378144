use near_sdk::{near_bindgen, env };
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
// Try these other ones, too!
// See: https://docs.rs/near-sdk/2.0.0/near_sdk/collections/index.html
// use near_sdk::collections::{ LookupMap, UnorderedMap, TreeMap };
use near_sdk::collections::{ TreeMap };
use near_sdk::json_types::U128;

#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc = near_sdk::wee_alloc::WeeAlloc::INIT;

// This isn't required, but a nice way to essentially alias a type
pub type UPC = u128;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Produce {
    pub veggies_taste: TreeMap<UPC, String>
}

impl Default for Produce {
    fn default() -> Self {
        env::panic(b"The contract is not initialized.")
    }
}

#[near_bindgen]
impl Produce {
    /// Init attribute used for instantiation.
    #[init]
    pub fn new() -> Self {
        // Useful snippet to copy/paste, making sure state isn't already initialized
        assert!(env::state_read::<Self>().is_none(), "Already initialized");
        // Note this is an implicit "return" here
        Self {
            veggies_taste: TreeMap::new(b"v".to_vec()),
        }
    }

    // This functions changes state, so 1st param uses `&mut self`
    /// Add a veggie and its taste
    pub fn add_veggie_taste(&mut self, upc: U128, taste: String) {
        let existing_veggie: Option<String> = self.veggies_taste.get(&upc.into());
        if existing_veggie.is_some() {
            env::panic(b"Sorry, already added this UPC.")
        }
        self.veggies_taste.insert(&upc.into(), &taste);
    }

    // This functions simple returns state, so 1st param uses `&self`
    /// Return the stored taste for a veggie
    pub fn get_taste(&self, upc: U128) -> String {
        match self.veggies_taste.get(&upc.into()) {
            Some(stored_taste) => {
                let log_message = format!("This veggie is pretty {}", stored_taste.clone());
                env::log(log_message.as_bytes());
                // found account user in map, return the taste
                stored_taste
            },
            // did not find the veggie
            // note: curly brackets after arrow are optional in simple cases, like other languages
            None => "No veggie found.".to_string()
        }
    }

    /// Throw out all veggies. (reset the data structure)
    pub fn perish_all(&mut self) {
        assert_eq!(env::current_account_id(), env::predecessor_account_id(), "To cause all veggies to perish, this method must be called by the (implied) contract owner.");
        self.veggies_taste.clear();
        env::log(b"All veggies removed, time to restock!");
    }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test -- --nocapture
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext, AccountId};

    // part of writing unit tests is setting up a mock context
    // this is also a useful list to peek at when wondering what's available in env::*
    fn get_context(input: Vec<u8>, is_view: bool, predecessor: AccountId) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "mike.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: predecessor,
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    // mark individual unit tests with #[test] for them to be registered and fired
    // unlike other frameworks, the function names don't need to be special or have "test" in it
    #[test]
    fn add_veggie() {
        // set up the mock context into the testing environment
        let context = get_context(vec![], false, "robert.testnet".to_string());
        testing_env!(context);
        // instantiate a contract variable with the counter at zero
        let mut contract = Produce::new();
        let cucumber_upc = U128(679508051007679508);
        let soso = "so-so".to_string();
        contract.add_veggie_taste(cucumber_upc.clone(), soso.clone());
        // we can do println! in tests, but reminder to use env::log outside of tests
        let returned_taste = contract.get_taste(cucumber_upc);
        println!("Taste returned: {}", returned_taste.clone());
        // confirm
        assert_eq!(soso, returned_taste);
    }
}