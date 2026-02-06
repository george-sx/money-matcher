use std::collections::BTreeMap;
use std::collections::VecDeque;
use crate::lob::types::{OrderId, Price};

#[derive(Debug, Default)]
pub struct PriceLevel {
	pub total_qty: u64,
	pub orders: VecDeque<OrderId>,
}

#[derive(Debug, Default)]
pub struct OrderBook {
	best_buy: Price,
	best_sell: Price,
	buy_orders: BTreeMap<Price, PriceLevel>, 
	sell_orders: BTreeMap<Price, PriceLevel>, 
}