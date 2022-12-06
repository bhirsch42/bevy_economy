use diesel::prelude::*;

use crate::tables::{agents, commodities};

#[derive(Queryable, Debug, Selectable)]
struct Agent {
    id: i32,
    name: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = agents)]
struct NewAgent {
    name: String,
}

#[derive(Queryable, Debug, Selectable)]
#[diesel(table_name = commodities)]
struct Commodity {
    id: i32,
    name: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = commodities)]
struct NewCommodity {
    name: String,
}
