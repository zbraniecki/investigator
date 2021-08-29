mod account;
mod model;
mod oracle;

use futures::future;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let oracle_server = oracle::server::new_server();
    let account_server = account::server::new_server();
    future::try_join(oracle_server, account_server).await?;
    Ok(())
}
