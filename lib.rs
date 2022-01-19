#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;






#[ink::contract]
mod dark_dex {
    use ink_storage::{
        Lazy,
        traits::{
            PackedLayout,
            SpreadAllocate,
            SpreadLayout,
        },
    };

    #[derive(Copy,PartialEq, Eq, Debug, Clone,scale::Encode, scale::Decode,PackedLayout,SpreadLayout)]
    #[cfg_attr(feature="std",derive(scale_info::TypeInfo,::ink_storage::traits::StorageLayout))]
    pub enum Status {
        Active,
        Inactive
    }

    #[derive(Copy,PartialEq, Eq, Debug, Clone, scale::Encode, scale::Decode,PackedLayout,SpreadLayout)]
    #[cfg_attr(feature="std",derive(scale_info::TypeInfo,::ink_storage::traits::StorageLayout))]
    pub enum Side {
        Buy,
        Sell
    }



    #[derive(Copy,PartialEq, Eq, Debug, Clone, scale::Encode, scale::Decode,PackedLayout,SpreadLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Order {
        pub acct: AccountId,
        pub status: Status,
        pub side: Side,
        pub size: u64,
        pub filled: u64
      }

      impl Order {
        pub fn new(acct:AccountId, side: Side, size: u64) -> Self {
          Order {
            acct,
            status: Status::Active,
            side,
            size,
            filled: 0
          }
        }

        pub fn fill(&mut self, size: u64){
            self.filled = self.filled + size;
            if self.filled == self.size {
              self.cancel();
            }
          }
        
          pub fn remaining(&self) -> u64 {
            self.size - self.filled
          }
        
          pub fn cancel(&mut self){
            self.status = Status::Inactive;
          }
    }


    #[derive(PartialEq, Eq, Debug, Clone, scale::Encode, scale::Decode,PackedLayout,SpreadLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Trade {
        pub side: Side,
        pub size: u64,
        pub px_u64: u64
      }
      
      impl Trade {
        pub fn new(side:Side, size: u64, px_u64: u64) -> Self {
          Trade {
            side,
            size,
            px_u64
          }
        }
      }
        


    #[ink(storage)]
    //#[cfg_attr(feature="std",derive(scale_info::TypeInfo))]
    pub struct dark_dex {        
        pub bids: Vec<Order>,
        pub asks: Vec<Order>,
        trade_histories: ink_storage::collections::HashMap<AccountId, Vec::<Trade>>
        
    }

    impl dark_dex {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(acct:AccountId, size: u64) -> Self {
            //let order = Order::new(acct,size);
            Self{            
                bids: Vec::new(),
                asks: Vec::new(),
                trade_histories: ink_storage::collections::HashMap::new()
              
            }
        }


        #[ink(message)]
        pub fn insert_order(&mut self, order: Order, px_u64: u64){
            if order.status == Status::Active {
              //Users are only allowed to have one active order at a time.
              //If the user has any existing orders, we cancel them first.
              self.cancel_user_orders(order.acct.clone());
              if order.side == Side::Buy {
                self.match_buy_order(order, px_u64);
              }else if order.side == Side::Sell {
                self.match_sell_order(order, px_u64);
              }
            }
            //Remove inactive orders
            self.cleanup();
          }

        #[ink(message)]
        pub fn find_order_by_acct(&mut self, acct: AccountId) -> Option<Order>{
            let bids = &self.bids;
            for bid in bids.iter() {
              if bid.acct == acct {
                return Some(*bid);
              }
            }
            let asks = &self.asks;
            for ask in asks.iter() {
              if ask.acct == acct {
                return Some(*ask);
              }
            }
            None
          }

        #[ink(message)]
        pub fn cancel_user_orders(&mut self, acct: AccountId){
            match self.find_order_by_acct(acct) {
              Some(mut order) => order.cancel(),
              None => {}
            }
          }
        
        #[ink(message)]
        pub fn match_buy_order(&mut self, buy_order: Order, px_u64: u64){
            let asks = self.asks.clone();
            for ask in asks.iter() {
              if Self::pretrade_checks(&self,*ask, px_u64) {
                let a = &mut buy_order.clone();
                let b = &mut ask.clone();
                Self::exec_trade(a, b, px_u64,  &mut self.trade_histories);
              }
              if buy_order.remaining() == 0 {
                break;
              }
            }
            self.bids.push(buy_order);
          }

        #[ink(message)]
        pub fn match_sell_order(&mut self, sell_order: Order, px_u64: u64){
            let bids = &self.bids;
            for bid in bids.iter() {
              if Self::pretrade_checks(&self,*bid, px_u64) {
                let a = &mut sell_order.clone();
                let b = &mut bid.clone();
                Self::exec_trade(b, a, px_u64, &mut self.trade_histories);
              }
              if sell_order.remaining() == 0 {
                break;
              }
            }
            self.asks.push(sell_order);
          }

        #[ink(message)]
        pub fn pretrade_checks(&self,order: Order, px_u64: u64) -> bool {
            let mut is_valid = true;
            if order.status != Status::Active {
              is_valid = false;
            }
            is_valid
          }
        
        #[ink(message)]
        pub fn cleanup(&mut self){
            self.bids.retain(|order| order.status == Status::Active);
            self.asks.retain(|order| order.status == Status::Active);
          }

        
          // This function needs to:
          // 1) Store the size filled by both the buy order and the sell order
          // 2) Save the trade to the trade history vec of both the buyer and seller
          // 3) Transfer the corresponding tokens to/from the buyer and seller
          pub fn exec_trade(
            buy_order: &mut Order,
            sell_order: &mut Order,
            px_u64: u64,
            trade_histories: &mut ink_storage::collections::HashMap<AccountId, Vec<Trade>>
          ) {
            let exec_size = scale_info::prelude::cmp::min(buy_order.remaining(), sell_order.remaining());
            buy_order.fill(exec_size);
            sell_order.fill(exec_size);
        
            // Save the trade to both buyer/seller account's trade histories
            let buy_trade = Trade::new(Side::Buy, exec_size, px_u64);
            let buyer_trade_history = trade_histories.get(&buy_order.acct);
            match buyer_trade_history {
              Some(buyer_trade_history_prev) => {
                let mut buyer_trade_history = buyer_trade_history_prev.to_vec();
                buyer_trade_history.push(buy_trade);
                trade_histories.insert(buy_order.acct.clone(), buyer_trade_history);
              },
              None => {
                let buyer_trade_history = vec!(buy_trade);
                trade_histories.insert(buy_order.acct.clone(), buyer_trade_history);
              }
            };
        
            let sell_trade = Trade::new(Side::Sell, exec_size, px_u64);
            let seller_trade_history = trade_histories.get(&sell_order.acct);
            match seller_trade_history {
              Some(seller_trade_history_prev) => {
                let mut seller_trade_history = seller_trade_history_prev.to_vec();
                seller_trade_history.push(sell_trade);
                trade_histories.insert(sell_order.acct.clone(), seller_trade_history);
              },
              None => {
                let seller_trade_history = vec!(sell_trade);
                trade_histories.insert(sell_order.acct.clone(), seller_trade_history);
              }
            };
        
            println!("TRADE EXECUTED @ {:?}, SIZE={}", px_u64, exec_size);
            //TODO: finish the rest of the logic when executing a trade
        
          }



        


    }


   
}
