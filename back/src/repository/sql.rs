use diesel::{prelude::PgConnection, Connection};
use dotenv::dotenv;

pub fn establish_connection() -> Result<PgConnection, Box<dyn std::error::Error>> {
    dotenv().ok();

    let url = std::env::var("DATABASE_URL")?;
    log::debug!("||{}||", url);

    let conn = PgConnection::establish(&url)?;
    log::debug!("GOT MAKAPALKA");

    Ok(conn)
}
