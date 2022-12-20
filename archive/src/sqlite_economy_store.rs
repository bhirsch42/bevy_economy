use crate::{models::*, tables::*};
use bevy::utils::HashMap;
use diesel::prelude::*;

#[derive(Default, Debug)]
pub struct SqliteEconomyStore {
    round: Round,
    historical_prices: HashMap<Id, Price>,
    config: EconomyConfig,
}

impl SqliteEconomyStore {
    fn update_historical_prices(&mut self, conn: &mut SqliteConnection) {
        use diesel::dsl::*;

        let avg_price = avg(trades::price);

        let query = trades::table
            .filter(
                trades::dsl::round
                    .gt(self.round - self.config.num_rounds_to_consider_for_historical_price),
            )
            .group_by(trades::commodity_id)
            .select((trades::commodity_id, avg_price));

        let id_price_pairs: Vec<(Id, Option<Price>)> =
            query.load(conn).expect("Error getting historical price");

        let sanitized_id_price_pairs = id_price_pairs.iter().map(|pair| (pair.0, pair.1.unwrap()));

        self.historical_prices = HashMap::from_iter(sanitized_id_price_pairs);
    }

    fn get_historical_prices(&self) -> &HashMap<i32, Price> {
        &self.historical_prices
    }
}

impl EconomyStore for SqliteEconomyStore {
    fn historical_price_of(&self, commodity: &Commodity) -> Price {
        let historical_price = self.historical_prices.get(&commodity.id);

        match historical_price {
            Some(price) => *price,
            None => self.config.default_init_price,
        }
    }

    fn add_agents(agents: &Vec<EconomyAgent>) {}
}

#[derive(Insertable, Debug)]
#[diesel(table_name = agents)]
struct NewAgent {
    pub name: String,
}

impl NewAgent {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Queryable, Debug, Selectable)]
#[diesel(table_name = commodities)]
struct CommodityRecord {
    pub id: Id,
    pub name: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = commodities)]
struct NewCommodity {
    pub name: String,
}
