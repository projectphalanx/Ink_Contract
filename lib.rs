#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod phalanx {
    use ink_storage::{
        traits::{PackedLayout, SpreadAllocate, SpreadLayout},
        Lazy,
    };

    use std::cmp;

    // #[derive(
    //     Copy, PartialEq, Eq, Debug, Clone, scale::Encode, scale::Decode, PackedLayout, SpreadLayout,
    // )]
    // #[cfg_attr(
    //     feature = "std",
    //     derive(scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
    // )]
    // pub enum Status {
    //     Active,
    //     Inactive,
    // }

    #[derive(
        Copy, PartialEq, Eq, Debug, Clone, scale::Encode, scale::Decode, PackedLayout, SpreadLayout,
    )]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
    )]
    pub enum Side {
        Bid,
        Ask,
    }

    #[derive(
        Copy, PartialEq, Eq, Debug, Clone, scale::Encode, scale::Decode, PackedLayout, SpreadLayout,
    )]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Order {
        pub acct: AccountId,
        // pub status: Status,
        // pub side: Side,
        pub size: u64,
        // pub filled: u64,
    }

    impl Order {
        pub fn new(acct: AccountId, size: u64) -> Self {
            // pub fn new(acct: AccountId, side: Side, size: u64) -> Self {
            Order {
                acct,
                // status: Status::Active,
                // side,
                size,
                // filled: 0,
            }
        }

        // pub fn fill(&mut self, size: u64) {
        //     self.filled = self.filled + size;
        //     if self.filled == self.size {
        //         self.cancel();
        //     }
        // }

        // pub fn remaining(&self) -> u64 {
        //     self.size - self.filled
        // }

        // pub fn cancel(&mut self) {
        //     self.status = Status::Inactive;
        // }
    }

    // #[derive(
    //     PartialEq, Eq, Debug, Clone, scale::Encode, scale::Decode, PackedLayout, SpreadLayout,
    // )]
    // #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    // pub struct Trade {
    //     pub side: Side,
    //     pub size: u64,
    //     pub px_u64: u64,
    // }

    // impl Trade {
    //     pub fn new(side: Side, size: u64, px_u64: u64) -> Self {
    //         Trade { side, size, px_u64 }
    //     }
    // }

    fn _trigger_trade(size: u64, price: u64, ask_acct: &AccountId, bid_acct: &AccountId) {
        println!(
            "Triggering Trade @ {:?}, Size={}, Accounts={:?}/{:?}",
            price, size, ask_acct, bid_acct
        );
    }

    #[ink(storage)]
    //#[cfg_attr(feature="std",derive(scale_info::TypeInfo))]
    pub struct Phalanx {
        pub bids: Vec<Order>,
        pub asks: Vec<Order>,
        // trade_histories: ink_storage::collections::HashMap<AccountId, Vec<Trade>>,
    }

    impl Phalanx {
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new()
        }

        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                bids: Vec::new(),
                asks: Vec::new(),
                // trade_histories: ink_storage::collections::HashMap::new(),
            }
        }

        #[ink(message)]
        //The "order" function adds a bid ora ask order for acct
        // acct can only have a single order in the queues. With the call of "order", any previous order of acct
        // is deleted
        pub fn order(&mut self, acct: AccountId, side: Side, size: u64) {
            // Check Account is valid?
            // If not, return an error?

            // Locate an existing order for this account in bids and asks queues (only 1 order per account)
            // If order found, remove it. Will be replace by a the new order (can change side)
            let o_acct_pos_bid = self.bids.iter().position(|&x| x.acct == acct);
            match o_acct_pos_bid {
                Some(index) => {
                    self.bids.remove(index);
                }
                None => {
                    let o_acct_pos_ask = self.asks.iter().position(|&x| x.acct == acct);
                    match o_acct_pos_ask {
                        Some(index) => {
                            self.asks.remove(index);
                        }
                        None => {}
                    }
                }
            }
            // Now acocunt has no order in queue. Add order
            let order = Order::new(acct, size);
            match side {
                Side::Bid => {
                    self.bids.push(order);
                }
                Side::Ask => {
                    self.asks.push(order);
                }
            }
        }

        // Return length of queue
        #[ink(message)]
        pub fn queue_get_length(&mut self, side: Side) -> u32 {
            let length = match side {
                Side::Bid => self.bids.len(),
                Side::Ask => self.asks.len(),
            };
            length as u32
        }

        // Return size of queue
        #[ink(message)]
        pub fn queue_get_size(&mut self, side: Side) -> u64 {
            match side {
                Side::Bid => self.bids.iter().map(|x| x.size).sum(),
                Side::Ask => self.asks.iter().map(|x| x.size).sum(),
            }
        }

        // Internal function to matches bids and asks and triggers transaction at the current price
        fn _clear_orders_at_price(&mut self, price: u64) {
            // Repeat until 1 queue is empty
            //  Take 1st orders in both queues
            //  Create a transaction at price between the 2 accounts
            //  Remove smallest order and reduce largest order accordingly (or remove both is same size)

            // If any of the transactions fails?? (ex: lack of gas, network issues)
            // Probably resolve anyways and clear the queue
            // trade exec should be async in some ways. Not possible to wait for result else order book would be stuck

            loop {
                if self.bids.len() == 0 {
                    break;
                }
                if self.asks.len() == 0 {
                    break;
                }

                let bid = self.bids.first().unwrap();
                let ask = self.asks.first().unwrap();
                let trade_size = cmp::min(bid.size, ask.size);
                _trigger_trade(trade_size, price, &ask.acct, &bid.acct); // Check for success?
                if bid.size == trade_size {
                    self.bids.remove(0);
                } else {
                    let first = self.bids.first_mut().unwrap();
                    first.size = first.size - trade_size;
                };
                if ask.size == trade_size {
                    self.asks.remove(0);
                } else {
                    let first = self.asks.first_mut().unwrap();
                    first.size = first.size - trade_size;
                };
            }
        }

        // #[ink(message)]
        // pub fn insert_order(&mut self, order: Order, px_u64: u64) {
        //     if order.status == Status::Active {
        //         //Users are only allowed to have one active order at a time.
        //         //If the user has any existing orders, we cancel them first.
        //         self.cancel_user_orders(order.acct.clone());
        //         if order.side == Side::Bid {
        //             self.match_buy_order(order, px_u64);
        //         } else if order.side == Side::Ask {
        //             self.match_sell_order(order, px_u64);
        //         }
        //     }
        //     //Remove inactive orders
        //     self.cleanup();
        // }

        #[ink(message)]
        pub fn find_order_by_acct(&mut self, acct: AccountId) -> Option<Order> {
            // let bids = &self.bids;
            // for bid in bids.iter() {
            //     if bid.acct == acct {
            //         return Some(*bid);
            //     }
            // }
            // let asks = &self.asks;
            // for ask in asks.iter() {
            //     if ask.acct == acct {
            //         return Some(*ask);
            //     }
            // }
            None
        }

        #[ink(message)]
        pub fn cancel_user_orders(&mut self, acct: AccountId) {
            // match self.find_order_by_acct(acct) {
            //     Some(mut order) => order.cancel(),
            //     None => {}
            // }
        }

        #[ink(message)]
        pub fn match_buy_order(&mut self, buy_order: Order, px_u64: u64) {
            // let asks = self.asks.clone();
            // for ask in asks.iter() {
            //     if Self::pretrade_checks(&self, *ask, px_u64) {
            //         let a = &mut buy_order.clone();
            //         let b = &mut ask.clone();
            //         Self::exec_trade(a, b, px_u64, &mut self.trade_histories);
            //     }
            //     if buy_order.remaining() == 0 {
            //         break;
            //     }
            // }
            // self.bids.push(buy_order);
        }

        #[ink(message)]
        pub fn match_sell_order(&mut self, sell_order: Order, px_u64: u64) {
            // let bids = &self.bids;
            // for bid in bids.iter() {
            //     if Self::pretrade_checks(&self, *bid, px_u64) {
            //         let a = &mut sell_order.clone();
            //         let b = &mut bid.clone();
            //         Self::exec_trade(b, a, px_u64, &mut self.trade_histories);
            //     }
            //     if sell_order.remaining() == 0 {
            //         break;
            //     }
            // }
            // self.asks.push(sell_order);
        }

        // #[ink(message)]
        // pub fn pretrade_checks(&self, order: Order, px_u64: u64) -> bool {
        // let mut is_valid = true;
        // if order.status != Status::Active {
        //     is_valid = false;
        // }
        // is_valid
        // }

        #[ink(message)]
        pub fn cleanup(&mut self) {
            // self.bids.retain(|order| order.status == Status::Active);
            // self.asks.retain(|order| order.status == Status::Active);
        }

        // This function needs to:
        // 1) Store the size filled by both the buy order and the sell order
        // 2) Save the trade to the trade history vec of both the buyer and seller
        // 3) Transfer the corresponding tokens to/from the buyer and seller
        pub fn exec_trade(
            buy_order: &mut Order,
            sell_order: &mut Order,
            // px_u64: u64,
            // trade_histories: &mut ink_storage::collections::HashMap<AccountId, Vec<Trade>>,
        ) {
            //     let exec_size =
            //         scale_info::prelude::cmp::min(buy_order.remaining(), sell_order.remaining());
            //     buy_order.fill(exec_size);
            //     sell_order.fill(exec_size);

            //     // Save the trade to both buyer/seller account's trade histories
            //     let buy_trade = Trade::new(Side::Bid, exec_size, px_u64);
            //     let buyer_trade_history = trade_histories.get(&buy_order.acct);
            //     match buyer_trade_history {
            //         Some(buyer_trade_history_prev) => {
            //             let mut buyer_trade_history = buyer_trade_history_prev.to_vec();
            //             buyer_trade_history.push(buy_trade);
            //             trade_histories.insert(buy_order.acct.clone(), buyer_trade_history);
            //         }
            //         None => {
            //             let buyer_trade_history = vec![buy_trade];
            //             trade_histories.insert(buy_order.acct.clone(), buyer_trade_history);
            //         }
            //     };

            //     let sell_trade = Trade::new(Side::Ask, exec_size, px_u64);
            //     let seller_trade_history = trade_histories.get(&sell_order.acct);
            //     match seller_trade_history {
            //         Some(seller_trade_history_prev) => {
            //             let mut seller_trade_history = seller_trade_history_prev.to_vec();
            //             seller_trade_history.push(sell_trade);
            //             trade_histories.insert(sell_order.acct.clone(), seller_trade_history);
            //         }
            //         None => {
            //             let seller_trade_history = vec![sell_trade];
            //             trade_histories.insert(sell_order.acct.clone(), seller_trade_history);
            //         }
            //     };

            //     println!("TRADE EXECUTED @ {:?}, SIZE={}", px_u64, exec_size);
            //     //TODO: finish the rest of the logic when executing a trade
        }
        fn _init_scenario_1(&mut self) {
            self.order(AccountId::from([0x01; 32]), Side::Ask, 50000);
            self.order(AccountId::from([0x02; 32]), Side::Ask, 30000);
            self.order(AccountId::from([0x03; 32]), Side::Ask, 2580);
            self.order(AccountId::from([0x04; 32]), Side::Bid, 20000);
        }

        fn _init_scenario_2(&mut self) {
            self.order(AccountId::from([0x03; 32]), Side::Ask, 2580);
            self.order(AccountId::from([0x01; 32]), Side::Ask, 50000);
            self.order(AccountId::from([0x02; 32]), Side::Ask, 30000);
            self.order(AccountId::from([0x04; 32]), Side::Bid, 20000);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

        #[ink::test]
        fn test_scenario_1() {
            // Init scenario 1
            // Check the queue length and size
            // Trigger clearing at price
            // Check the queue length and size
            let mut phalanx = Phalanx::default();
            phalanx._init_scenario_1();

            assert_eq!(3, phalanx.queue_get_length(Side::Ask));
            assert_eq!(1, phalanx.queue_get_length(Side::Bid));
            assert_eq!(82580, phalanx.queue_get_size(Side::Ask));
            assert_eq!(20000, phalanx.queue_get_size(Side::Bid));

            phalanx._clear_orders_at_price(1);

            assert_eq!(3, phalanx.queue_get_length(Side::Ask));
            assert_eq!(0, phalanx.queue_get_length(Side::Bid));
            assert_eq!(62580, phalanx.queue_get_size(Side::Ask));
            assert_eq!(0, phalanx.queue_get_size(Side::Bid));
        }

        #[ink::test]
        fn test_scenario_2() {
            // Init scenario 2
            // Check the queue length and size
            // Trigger clearing at price
            // Check the queue length and size
            let mut phalanx = Phalanx::default();
            phalanx._init_scenario_2();

            assert_eq!(3, phalanx.queue_get_length(Side::Ask));
            assert_eq!(1, phalanx.queue_get_length(Side::Bid));
            assert_eq!(82580, phalanx.queue_get_size(Side::Ask));
            assert_eq!(20000, phalanx.queue_get_size(Side::Bid));

            phalanx._clear_orders_at_price(1);

            assert_eq!(2, phalanx.queue_get_length(Side::Ask));
            assert_eq!(0, phalanx.queue_get_length(Side::Bid));
            assert_eq!(62580, phalanx.queue_get_size(Side::Ask));
            assert_eq!(0, phalanx.queue_get_size(Side::Bid));
        }
    }
}
