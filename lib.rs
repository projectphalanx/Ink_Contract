#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
pub mod dark_dex {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.

    use ink_storage::{
      traits::SpreadLayout,
    };

    #[derive(PartialEq, Eq, Debug, Clone,scale::Encode, scale::Decode,SpreadLayout)]
    #[cfg_attr(feature="std",derive(scale_info::TypeInfo,::ink_storage::traits::StorageLayout,::ink_storage::traits::PackedLayout))]
    pub enum Status {
        Active,
        Inactive
    }

    #[derive(PartialEq, Eq, Debug, Clone, scale::Encode, scale::Decode,SpreadLayout)]
    #[cfg_attr(feature="std",derive(scale_info::TypeInfo,::ink_storage::traits::StorageLayout,::ink_storage::traits::PackedLayout))]
    pub enum Side {
        Buy,
        Sell
    }

    #[derive(PartialEq, Eq, Debug, Clone, scale::Encode, scale::Decode,SpreadLayout)]
    #[cfg_attr(feature="std",derive(scale_info::TypeInfo,::ink_storage::traits::StorageLayout,::ink_storage::traits::PackedLayout))]
    pub enum Order {
      acct(AccountId),
      status(Status),
      side(Side),
      size(u64),
      filled(u64),
  }


    

    #[ink(storage)]
    #[cfg_attr(feature="std",derive(::scale_info::TypeInfo))]
    pub struct dark_dex {  
        pub bids: Vec<Order>,
        pub asks: Vec<Order>,        
    }

    impl dark_dex {       
 
        #[ink(constructor)]
        pub fn new(acct:AccountId,  size: u64)->Self{
          Self{         
          bids:vec![Order::acct(acct),
                    Order::status(Status::Active),
                    Order::side(Side::Buy),
                    Order::size(size),
                    Order::filled(0)] ,
          asks:vec![Order::acct(acct),
                    Order::status(Status::Active),
                    Order::side(Side::Sell),
                    Order::size(size),
                    Order::filled(0)],
        }
    }


  
    
        #[ink(message)]
        pub fn _cancel(&mut self,side:Side){
          if side == Side::Buy{
            let index= self._index(side) as usize;
            self.bids[index]=self::Order::status(Status::Inactive);
      }else{
        let index = self._index(side) as usize;
        self.asks[index]=self::Order::status(Status::Inactive);
          }
            
            }

        #[ink(message)]
        pub fn _index(&mut self,side:Side)->u8{

          match side{
            Side::Buy => self.bids.iter().position(|r| r == &Order::side(Side::Buy)).unwrap() as u8,
            Side::Sell => self.bids.iter().position(|r| r == &Order::side(Side::Sell)).unwrap() as u8,

          }
        }

          }

    }



  

