#![cfg_attr(not(feature = "std"), no_std)]

use ink_env::Environment;
use ink_lang as ink;

#[ink::chain_extension]
pub trait ChainlinkExtension {
    type ErrorCode = ChainlinkErr;

    #[ink(extension = 70930000, returns_result = false)]
    fn latest_data(subject: u32) -> u128;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum ChainlinkErr {
    FailGetFeed,
}

impl ink_env::chain_extension::FromStatusCode for ChainlinkErr {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        match status_code {
            0 => Ok(()),
            1 => Err(Self::FailGetFeed),
            _ => panic!("encountered unknown status code"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum CustomEnvironment {}

impl Environment for CustomEnvironment {
    const MAX_EVENT_TOPICS: usize =
        <ink_env::DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;

    type AccountId = <ink_env::DefaultEnvironment as Environment>::AccountId;
    type Balance = <ink_env::DefaultEnvironment as Environment>::Balance;
    type Hash = <ink_env::DefaultEnvironment as Environment>::Hash;
    type BlockNumber = <ink_env::DefaultEnvironment as Environment>::BlockNumber;
    type Timestamp = <ink_env::DefaultEnvironment as Environment>::Timestamp;

    type ChainExtension = ChainlinkExtension;
}


#[ink::contract(env = crate::CustomEnvironment)]
mod test_chainlink_extension {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct TestChainlinkExtension {
        last_value: u128
    }

    #[ink(event)]
    pub struct FeedUpdated {
        #[ink(topic)]
        feed_id: u32,
        answer: u128,
    }

    impl TestChainlinkExtension {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { last_value: Default::default() }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new()
        }

        /// Seed a random value by passing some known argument `subject` to the runtime's
        /// random source. Then, update the current `value` stored in this contract with the
        /// new random value.
        #[ink(message)]
        pub fn latest_data(&mut self, feed_id: u32) -> Result<u128, crate::ChainlinkErr> {
            let caller = self.env().caller();
            let message = ink_prelude::format!("got a call from {:?}", caller);
            ink_env::debug_println!("{}", &message);
         
            // Get the on-chain random seed
            log::info!("called latest_data contract");
            let feed = self.env().extension().latest_data(feed_id)?;
            ink_env::debug_println!("############ Received value {:?}", &feed);
            
            self.last_value = feed;

            self.env().emit_event(FeedUpdated { feed_id: feed_id, answer: feed });
            Ok(feed)
        }
        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn last_value(&self) -> u128 {
            self.last_value
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let test_rand_extension = TestRandExtension::default();
            assert_eq!(test_rand_extension.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut test_rand_extension = TestRandExtension::new(false);
            assert_eq!(test_rand_extension.get(), false);
            test_rand_extension.flip();
            assert_eq!(test_rand_extension.get(), true);
        }
    }
}
