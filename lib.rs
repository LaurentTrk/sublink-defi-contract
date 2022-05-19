#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract(env = sublink_ink_contract_extension::SubLinkEnvironment)]
mod sublink_defi_contract {

    
    #[ink(storage)]
    pub struct SubLinkDefiContract {
        /// We store the latest value retrieve from the extension
        latest_price_value: u128
    }

    #[ink(event)]
    pub struct FeedUpdated {
        #[ink(topic)]
        feed_id: u32,
        answer: u128,
    }

    impl SubLinkDefiContract {
        // Instantiate a new contract
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { latest_price_value: Default::default() }
        }

        /// Get the latest price value of a given feed.
        /// Will call the SubLink ink! extension
        #[ink(message)]
        pub fn latest_data(&mut self, feed_id: u32) -> Result<u128, sublink_ink_contract_extension::SubLinkErr> {
            let caller = self.env().caller();
            let message = ink_prelude::format!("got a call from {:?}", caller);
            ink_env::debug_println!("{}", &message);
         
            log::info!("called latest_data contract");
            let feed = self.env().extension().latest_data(feed_id)?;
            ink_env::debug_println!("############ Received value {:?}", &feed);
            
            self.latest_price_value = feed;

            self.env().emit_event(FeedUpdated { feed_id: feed_id, answer: feed });
            Ok(feed)
        }

        /// Return the latest value received from the SubLink ink! extension
        #[ink(message)]
        pub fn latest_price_value(&self) -> u128 {
            self.latest_price_value
        }        
        
        /// Return the latest value received from the SubLink ink! extension
        #[ink(message)]
        pub fn get_latest_price(&self, feed_id: u32) -> u128 {
            let latest_price = self.env().extension().latest_data(feed_id).unwrap_or_default();
            ink_env::debug_println!("######################## get_latest_price value {:?}", &latest_price);
            latest_price
            
        }
    }
}
