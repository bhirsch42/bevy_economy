// @generated automatically by Diesel CLI.

diesel::table! {
    agents (id) {
        id -> Integer,
        created_at -> Timestamp,
        name -> Text,
    }
}

diesel::table! {
    commodities (id) {
        id -> Integer,
        created_at -> Timestamp,
        name -> Text,
    }
}

diesel::table! {
    inventories (id) {
        id -> Integer,
        created_at -> Timestamp,
        agent_id -> Integer,
        commodity_id -> Integer,
        quantity -> Integer,
        max_quantity -> Integer,
    }
}

diesel::table! {
    offers (id) {
        id -> Integer,
        created_at -> Timestamp,
        agent_id -> Integer,
        commodity_id -> Integer,
        is_buy -> Bool,
        price -> Float,
        quantity -> Integer,
        round -> Integer,
    }
}

diesel::table! {
    price_beliefs (id) {
        id -> Integer,
        created_at -> Timestamp,
        agent_id -> Integer,
        commodity_id -> Integer,
        max_price -> Float,
        min_price -> Float,
    }
}

diesel::table! {
    trades (id) {
        id -> Integer,
        created_at -> Timestamp,
        buy_offer_id -> Integer,
        sell_offer_id -> Integer,
        commodity_id -> Integer,
        price -> Float,
        quantity -> Integer,
        round -> Integer,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    agents,
    commodities,
    inventories,
    offers,
    price_beliefs,
    trades,
);
