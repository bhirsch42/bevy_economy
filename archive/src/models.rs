use bevy::prelude::*;

pub type Price = f64;
pub type Id = i32;
pub type Round = i32;

#[derive(Debug)]
pub struct EconomyConfig {
    pub default_init_price: Price,
    pub default_historical_price_trade_count: i64,
    pub default_price_relaxation: f32,
    pub default_inventory_space_per_commodity: u32,
    pub num_rounds_to_consider_for_historical_price: Round,
}

impl Default for EconomyConfig {
    fn default() -> Self {
        Self {
            default_init_price: 100.0,
            default_historical_price_trade_count: 10,
            default_price_relaxation: 0.1,
            default_inventory_space_per_commodity: 100,
            num_rounds_to_consider_for_historical_price: 3,
        }
    }
}

pub trait EconomyStore {
    fn historical_price_of(&self, commodity: &Commodity) -> Price;
    fn add_agents(agents: &Vec<EconomyAgent>);

    fn id_for(entity: &Entity) -> String {
        format!("{}:{}", entity.index(), entity.generation())
    }
}

pub struct Commodity {
    pub id: Id,
    pub name: String,
}

struct PriceBelief {
    min_price: Price,
    max_price: Price,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct EconomyAgent {}

impl EconomyAgent {
    // fn price_belief_for(&self, commodity: &Commodity) -> PriceBelief {
    //     let price_belief = self.price_beliefs.get(&commodity.id);

    //     if let Some(belief) = price_belief {
    //         *belief
    //     } else {
    //         let price = self.market.get_historical_price(commodity);
    //         PriceBelief {
    //             min: price * (1. - DEFAULT_PRICE_RELAXATION),
    //             max: price * (1. + DEFAULT_PRICE_RELAXATION),
    //         }
    //     }
    // }
}

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

// Commodity
