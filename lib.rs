#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

// We use the SubLink environment that provides us the SubLink ink! extension
#[ink::contract(env = sublink_ink_contract_extension::SubLinkEnvironment)]
mod sublink_defi_contract {

    use sublink_ink_contract_extension::RoundData;

    
    #[ink(storage)]
    pub struct SubLinkDefiContract {
    }


    impl SubLinkDefiContract {
        // Instantiate a new contract
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { }
        }
        
        /// Return the latest round data received from the SubLink ink! extension
        #[ink(message)]
        pub fn get_latest_round_data(&self, feed_id: u32) -> RoundData {
            // The extension gives us the latest_round_data function
            // (as defined in Chainlink API https://docs.chain.link/docs/price-feeds-api-reference/#latestrounddata)
            self.env().extension().latest_round_data(feed_id).unwrap_or_default()
        }
    }
}
