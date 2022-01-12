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
        pub fn new()->Self{
          Self{
          bids:Vec::new(),
          asks:Vec::new(),

        }
    }


        #[ink(message)]
        //nbr is the order number--> from 1 to n
        pub fn _remaining(&mut self,side:Side,nbr:u8) -> u64 {

          match side{
            Side::Buy =>{

          let size = self.bids[3+5*(nbr-1)].clone();
          let filled = self.bids[4+5*(nbr-1)].clone();
          let sz=match size{
            Order::size(value)=>value,
            _ =>0
          };
          let fl=match filled{
            Order::filled(value)=>value,
            _=>0
          };
          let remain = sz-fl;
          return remain
        }

        Side::Sell =>{

          let size = self.asks[3+5*(nbr-1)].clone();
          let filled = self.asks[4+5*(nbr-1)].clone();
          let sz=match size{
            Order::size(value)=>value,
            _ =>0
          };
          let fl=match filled{
            Order::filled(value)=>value,
            _=>0
          };
          let remain = sz-fl;
          return remain
        }



        }

        }
        //   self.size - self.filled
          

           
           #[ink(message)]
        pub fn _init(&mut self,acct:AccountId,  size: u64){
          self.bids.push(Order::acct(acct));
          self.bids.push(Order::status(Status::Active));
          self.bids.push(Order::side(Side::Buy));
          self.bids.push(Order::size(size));
          self.bids.push(Order::filled(0));

          
          self.asks.push(Order::acct(acct));
          self.asks.push(Order::status(Status::Active));
          self.asks.push(Order::side(Side::Buy));
          self.asks.push(Order::size(size));
          self.asks.push(Order::filled(0));


          
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



  
