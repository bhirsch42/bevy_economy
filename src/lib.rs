use anyhow::Result;
use diesel::prelude::*;
use diesel::{sqlite::SqliteConnection, Connection, RunQueryDsl};

pub mod models;
pub mod tables;

fn establish_connection() -> Result<SqliteConnection, diesel::ConnectionError> {
    SqliteConnection::establish(":memory:")
}
fn init_tables() -> Result<usize> {
    let mut conn = establish_connection()?;
    let result = diesel::sql_query(include_str!("../src/create_tables.sql")).execute(&mut conn)?;
    Ok(result)
}

// use std::{
//     cmp::{max, min},
//     collections::HashMap,
// };

// use average::Mean;
// use bevy::prelude::*;
// use float_ord::FloatOrd;
// use rand::{thread_rng, Rng};

// const DEFAULT_INIT_PRICE: Price = 100.0;
// const DEFAULT_HISTORICAL_PRICE_TRADE_COUNT: usize = 10;
// const DEFAULT_PRICE_RELAXATION: f32 = 0.1;
// const DEFAULT_INVENTORY_SPACE_PER_COMMODITY: u32 = 100;

// pub struct EconomyPlugin;

// impl Plugin for EconomyPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_system(hello_world);
//     }
// }

// pub struct Commodity {
//     id: Id,
//     name: String,
// }

// enum OfferType {
//     Buy,
//     Sell,
// }

// pub struct Offer {
//     offer_type: OfferType,
//     price: Price,
//     quantity: Quantity,
//     agent_id: Id,
// }

// #[derive(Clone, Copy)]
// struct PriceBelief {
//     min: Price,
//     max: Price,
// }

// struct Trade {
//     commodity_id: Id,
//     price: Price,
//     quantity: Quantity,
// }

// struct Market {
//     trades: Vec<Trade>,
//     historical_trade_eval_count: usize,
//     default_price: f32,
// }

// impl Default for Market {
//     fn default() -> Self {
//         Self {
//             trades: vec![],
//             historical_trade_eval_count: DEFAULT_HISTORICAL_PRICE_TRADE_COUNT,
//             default_price: DEFAULT_INIT_PRICE,
//         }
//     }
// }

// impl Market {
//     fn get_historical_price(&self, commodity: &Commodity) -> Price {
//         let recent_trades: Mean = self
//             .trades
//             .iter()
//             .rev()
//             .filter(|trade| trade.commodity_id == commodity.id)
//             .map(|trade| trade.price)
//             .take(self.historical_trade_eval_count)
//             .map(f64::from)
//             .collect();

//         if recent_trades.is_empty() {
//             self.default_price
//         } else {
//             recent_trades.mean() as f32
//         }
//     }
// }

// pub struct Agent {
//     price_beliefs: HashMap<Id, PriceBelief>,
//     inventory: HashMap<Id, Quantity>,
//     market: Market,
//     id: Id,
// }

// fn cmp_offers(offer1: &&Offer, offer2: &&Offer) -> std::cmp::Ordering {
//     FloatOrd(offer1.price).cmp(&FloatOrd(offer2.price))
// }

// fn process_offers(offers: Vec<Offer>) {
//     let mut buy_offers: Vec<&Offer> = vec![];
//     let mut sell_offers: Vec<&Offer> = vec![];

//     offers.iter().for_each(|offer| match offer.offer_type {
//         OfferType::Buy => buy_offers.push(offer),
//         OfferType::Sell => sell_offers.push(offer),
//     });

//     buy_offers.sort_by(cmp_offers);
//     buy_offers.reverse();
//     sell_offers.sort_by(cmp_offers);

//     while !buy_offers.is_empty() && !sell_offers.is_empty() {
//         let buy_offer = buy_offers.first().unwrap();
//         let sell_offer = sell_offers.first().unwrap();
//         let quantity_to_trade = min(buy_offer.quantity, sell_offer.quantity);
//         let clearing_price = (buy_offer.price + sell_offer.price) / 2.;
//     }
// }

// impl Agent {
//     fn price_belief_for(&self, commodity: &Commodity) -> PriceBelief {
//         let price_belief = self.price_beliefs.get(&commodity.id);

//         if let Some(belief) = price_belief {
//             *belief
//         } else {
//             let price = self.market.get_historical_price(commodity);
//             PriceBelief {
//                 min: price * (1. - DEFAULT_PRICE_RELAXATION),
//                 max: price * (1. + DEFAULT_PRICE_RELAXATION),
//             }
//         }
//     }

//     fn price_for(&self, commodity: &Commodity) -> Price {
//         let price_belief = self.price_belief_for(&commodity);
//         thread_rng().gen_range(price_belief.min..price_belief.max)
//     }

//     fn inventory_quantity_of(&self, commodity: &Commodity) -> Quantity {
//         match self.inventory.get(&commodity.id) {
//             Some(quantity) => *quantity,
//             None => 0,
//         }
//     }

//     fn inventory_space_for(&self, commodity: &Commodity) -> Quantity {
//         DEFAULT_INVENTORY_SPACE_PER_COMMODITY - self.inventory_quantity_of(commodity)
//     }

//     fn favorability_of_selling(&self, commodity: &Commodity) -> f32 {
//         let historical_price = self.market.get_historical_price(commodity);
//         let price_belief = self.price_belief_for(commodity);
//         let favorability =
//             (historical_price - price_belief.min) / (price_belief.max - price_belief.min);
//         favorability.clamp(0., 1.)
//     }

//     fn favorability_of_buying(&self, commodity: &Commodity) -> f32 {
//         1. - self.favorability_of_selling(commodity)
//     }

//     fn sell_quantity_for(&self, commodity: &Commodity) -> Quantity {
//         let favorability = self.favorability_of_selling(commodity);
//         let quantity = self.inventory_quantity_of(commodity);
//         ((quantity as f32) * favorability) as Quantity
//     }

//     fn buy_quantity_for(&self, commodity: &Commodity) -> Quantity {
//         let favorability = self.favorability_of_buying(commodity);
//         let quantity = self.inventory_space_for(commodity);
//         ((quantity as f32) * favorability) as Quantity
//     }

//     pub fn create_bid(&self, commodity: &Commodity, limit: Quantity) -> Offer {
//         Offer {
//             offer_type: OfferType::Buy,
//             price: self.price_for(commodity),
//             quantity: min(self.buy_quantity_for(commodity), limit),
//             agent_id: self.id,
//         }
//     }

//     pub fn create_sale(&self, commodity: &Commodity, limit: Quantity) -> Offer {
//         Offer {
//             offer_type: OfferType::Sell,
//             price: self.price_for(commodity),
//             quantity: max(self.sell_quantity_for(commodity), limit),
//             agent_id: self.id,
//         }
//     }
// }

// fn hello_world() {
//     println!("Hello, world!");
// }

// /*
//  * Each agent maintains a set of price beliefs
//  * Each price belief has an upper and lower bound
//  * Bid by selecting price uniformly randomly within those bounds
//  * If good outcome, narrow bounds around mean
//  * If bad outcome, increase interval around mean OR translate mean
//  * When an agent wishes to create an offer, it will need to
//  *  determine the commodity to trade, a fair price, and the quantity of the commodity to trade.
//  */
