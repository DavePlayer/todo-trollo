use diesel::{prelude::MysqlConnection, Connection};
use dotenv::dotenv;

pub fn establish_connection() -> Result<MysqlConnection, Box<dyn std::error::Error>> {
    dotenv().ok();

    let url = std::env::var("DATABASE_URL")?;
    // log::debug!("||{}||", url);

    let conn = MysqlConnection::establish(&url)?;
    log::debug!("connected with mysql somehow. don't ask how");

    Ok(conn)
}
