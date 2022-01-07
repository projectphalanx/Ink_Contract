#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod dark_dex {
    
    use Order::{OrderRef, Side};
    use ink_storage::{
        traits::{
            PackedLayout,
            SpreadLayout,
        },
        Lazy,
    };
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct dark_dex {
        pub bids: Vec<OrderRef>,
        pub asks: Vec<OrderRef>,
    }

    impl dark_dex {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(acct:AccountId, side: Side, size: u64) -> Self {
            let order = OrderRef::new(acct,side,size);
            Self{
            
                bids: Vec::new(),
                asks: Vec::new(),
              
            }
        }


        #[ink(message)]
        pub fn _pretrade_checks(&self,order: OrderRef, px_u64: u64) -> bool {
            let mut is_valid = true;
            //if order.status != OrderRef::Status::Active {
             // is_valid = false;
            //}
            is_valid
          }
        


    }


   
}
