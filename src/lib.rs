use bevy::prelude::*;

pub struct EconomyPlugin;

impl Plugin for EconomyPlugin {
    fn build(&self, app: &mut App) {
        app;
    }
}

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

struct Market {
    trades: Vec<Trade>,
    historical_trade_eval_count: usize,
    default_price: f32,
}

impl Market {
    fn get_historical_price(&self, commodity: &Commodity) -> Price {
        let recent_trades: Mean = self
            .trades
            .iter()
            .rev()
            .filter(|trade| trade.commodity_id == commodity.id)
            .map(|trade| trade.price)
            .take(self.historical_trade_eval_count)
            .map(f64::from)
            .collect();

        if recent_trades.is_empty() {
            self.default_price
        } else {
            recent_trades.mean() as f32
        }
    }
}
