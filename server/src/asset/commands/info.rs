use super::super::api;
use super::super::db;
use crate::db::establish_connection;

pub fn get_prefix() -> &'static str {
    "asset_info"
}

pub fn get_list() -> Vec<&'static str> {
    vec!["fetch", "fetchAll"]
}

pub async fn handle_command(cmd: &str, args: &[String]) -> bool {
    match cmd {
        "fetch" => fetch(args).await,
        "fetchAll" => fetch_all(args).await,
        _ => {
            return false;
        }
    }
    return true;
}

pub async fn fetch(args: &[String]) {
    let connection = establish_connection();
    let id = args.get(2).unwrap();
    let mut infos = api::crypto::fetch_price_info(vec![id.to_string()]).await.unwrap();
    infos[0].reference_asset = "usd".to_string();
    db::info::delete(&connection, id);
    db::info::create(&connection, infos[0].clone());
}

pub async fn fetch_all(_args: &[String]) {
    let connection = establish_connection();
    let assets = crate::asset::db::asset::filter(&connection, None);

    let ids = assets.into_iter().map(|asset| asset.id).collect();
    let infos = api::crypto::fetch_price_info(ids).await.unwrap();
    for mut info in infos {
        info.reference_asset = "usd".to_string();
        db::info::delete(&connection, &info.asset);
        db::info::create(&connection, info);
    }
}
