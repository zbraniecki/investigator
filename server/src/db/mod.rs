// mod assets;
// mod identities;
// mod markets;
// mod prices;
pub mod schema;
// mod wallets;

// pub use assets::*;
// pub use identities::*;
// pub use markets::*;
// pub use prices::*;
// pub use wallets::*;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
