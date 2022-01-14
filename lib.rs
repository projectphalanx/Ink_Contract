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
      Acct(AccountId),
      Status(Status),
      Side(Side),
      Size(u64),
      Filled(u64),
  }


    

    #[ink(storage)]
    #[cfg_attr(feature="std",derive(::scale_info::TypeInfo))]
    pub struct DarkDex {
        pub ordersb:u64,
        pub ordersa:u64,
        pub bids: Vec<Order>,
        pub asks: Vec<Order>,        
    }

    impl DarkDex {       
 
        #[ink(constructor)]
        pub fn new()->Self{
          Self{
          ordersb:0, //number of bids
          ordersa:0, //number of asks
          bids:Vec::new(),
          asks:Vec::new(),

        }
    }


        #[ink(message)]
        pub fn _remaining(&mut self,side:Side,order:u64) -> u64 {

          match side{
            Side::Buy =>{
              assert_eq!(order<= self.ordersb,true);
              let vc=self._order_get(side,order);
              let ind0= vc.iter().position(|r| r == &Order::Side(Side::Buy)).unwrap()+1 as usize;
              let size = vc[ind0].clone();
              let filled = vc[ind0+1].clone();
              let sz=match size{
                Order::Size(value)=>value,
                _ =>0
              };
              let fl=match filled{
                Order::Filled(value)=>value,
                _=>0
              };
              let remain = sz-fl;
              return remain
            }

            Side::Sell =>{
              assert_eq!(order<= self.ordersa,true);
              let vc=self._order_get(side,order);
              let ind0= vc.iter().position(|r| r == &Order::Side(Side::Sell)).unwrap()+1 as usize;
              let size = vc[ind0].clone();
              let filled = vc[ind0+1].clone();
              let sz=match size{
                Order::Size(value)=>value,
                _ =>0
              };
              let fl=match filled{
                Order::Filled(value)=>value,
                _=>0
              };
              let remain = sz-fl;
              return remain
        }

        }

        }

           #[ink(message)]
           //Add a bid order
        pub fn add_bid(&mut self,acct:AccountId,  size: u64){
          self.bids.push(Order::Acct(acct));
          self.bids.push(Order::Status(Status::Active));
          self.bids.push(Order::Side(Side::Buy));
          self.bids.push(Order::Size(size));
          self.bids.push(Order::Filled(0));
          self.ordersb+=1;       
          }

          #[ink(message)]
          //Add an ask order
        pub fn add_ask(&mut self,acct:AccountId,  size: u64){         
          self.asks.push(Order::Acct(acct));
          self.asks.push(Order::Status(Status::Active));
          self.asks.push(Order::Side(Side::Sell));
          self.asks.push(Order::Size(size));
          self.asks.push(Order::Filled(0));
          self.ordersa+=1;          
          }
    
        #[ink(message)]
        //deactivating a given order, using a IF method, but we could also use a Match pattern method
        pub fn _deactivate(&mut self,side:Side,order:u64){
          if side == Side::Buy{
            let a = 5*(order-1);
            for i in a..a+5{
              let ii= i as usize;
              if self.bids[ii]==self::Order::Status(Status::Active){
                self.bids[ii]=self::Order::Status(Status::Inactive);
              }
            }
      }else{
            let a = 5*(order-1);
                for i in a..a+5{
                  let ii= i as usize;
                  if self.asks[ii]==self::Order::Status(Status::Active){
                    self.asks[ii]=self::Order::Status(Status::Inactive);
                  }
                }
          }
            
            }

        #[ink(message)]
        //Here we get an order, based on its side, and its number (order nbr.1, order nbr.2, order nbr.n)
        //We are basically getting a slice of the vector of interest
        pub fn _order_get(&mut self,side:Side,order:u64)->Vec<Order>{
          
          assert_eq!(order>= 1,true);
          match side{
            Side::Buy => {
              assert_eq!(order<= self.ordersb,true);
              let id0=5*(order-1) as usize;
              (&self.bids[id0..id0+5]).to_vec()
              },
            Side::Sell => {
              assert_eq!(order<= self.ordersa,true);              
              let id0=5*(order-1) as usize;
              (&self.asks[id0..id0+5]).to_vec()
            
            },

          }


        }

          }

    }

