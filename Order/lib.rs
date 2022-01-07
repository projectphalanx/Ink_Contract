#![cfg_attr(not(feature = "std"), no_std)]
pub use self::order::{
    Order,
    OrderRef,
    Side
};
use ink_lang as ink;

#[ink::contract]
pub mod order {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.

    use ink_storage::{
      traits::SpreadLayout,
    };

    #[derive(PartialEq, Eq, Debug, Clone,scale::Encode, scale::Decode,SpreadLayout)]
    #[cfg_attr(feature="std",derive(::scale_info::TypeInfo,::ink_storage::traits::StorageLayout))]
    pub enum Status {
        Active,
        Inactive
    }

    #[derive(PartialEq, Eq, Debug, Clone, scale::Encode, scale::Decode,SpreadLayout)]
    #[cfg_attr(feature="std",derive(::scale_info::TypeInfo,::ink_storage::traits::StorageLayout))]
    pub enum Side {
        Buy,
        Sell
    }
    

    #[ink(storage)]
    pub struct Order {
        pub acct: AccountId,
        pub status: Status,
        pub side: Side,
        pub size: u64,
        pub filled: u64
    }

    impl Order {
        
 
        #[ink(constructor)]
        pub fn new(acct:AccountId, side: Side, size: u64) -> Self {
            Order {
                acct,
                status: Status::Active,
                side,
                size,
                filled: 0
              }
        }


        #[ink(message)]
        pub fn _fill(&mut self, size: u64){
            self.filled = self.filled + size;
            if self.filled == self.size {
              self._cancel();
            }
          }

        #[ink(message)]
        pub fn _remaining(&self) -> u64 {
            self.size - self.filled
          }

        #[ink(message)]
        pub fn _cancel(&mut self){
            self.status = Status::Inactive;
          }
    }

 
}
