extern crate bevy_economy;

use bevy_economy::models::{EconomyAgent, NewAgent};
use bevy_economy::tables::agents;
use diesel::prelude::*;

use diesel::sqlite::SqliteConnection;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = &mut SqliteConnection::establish(":memory:").unwrap();

    // Create the `users` table in the in-memory database
    diesel::sql_query(include_str!("../src/create_tables.sql")).execute(conn)?;

    let agent = NewAgent {
        name: "Steven Universe".to_string(),
    };

    // Insert the user into the `users` table
    diesel::insert_into(agents::table)
        .values(agent)
        .execute(conn)?;

    let results = agents::table
        .select(EconomyAgent::as_select())
        .load::<EconomyAgent>(conn)?;

    println!("{:?}", results);

    Ok(())
}
