use super::super::api;
use super::super::db;
use super::super::models::{AssetTag, TagCategory};
use crate::db::establish_connection;
use diesel::prelude::*;

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

fn get_class_tag_for_aset(
    conn: &PgConnection,
    class_tags: &[TagCategory],
    asset_id: &str,
) -> Option<AssetTag> {
    let asset_tags = db::tag::get_for_asset(conn, asset_id);

    let class_tag = asset_tags
        .iter()
        .filter(|at| class_tags.iter().any(|tag| at.tag == tag.tag))
        .collect::<Vec<_>>();
    assert!(class_tag.len() < 2);
    class_tag.get(0).cloned().cloned()
}

pub async fn fetch(args: &[String]) {
    let connection = establish_connection();
    let id = args.get(2).unwrap();

    let class_tags = db::category::get_tags(&connection, "class");
    let asset = db::asset::get(&connection, id).unwrap();
    let class_tag = get_class_tag_for_aset(&connection, &class_tags, &asset.id);

    if let Some(class_tag) = class_tag {
        match class_tag.tag.as_str() {
            "crypto" => {
                let infos = api::crypto::fetch_price_info(vec![id.to_string()])
                    .await
                    .unwrap();
                db::info::delete(&connection, id);
                db::info::create(&connection, infos[0].clone());
            }
            "stock" => {
                let infos = api::stock::fetch_price_info(vec![id.to_string()])
                    .await
                    .unwrap();
                db::info::delete(&connection, id);
                db::info::create(&connection, infos[0].clone());
            }
            _ => {
                println!("Don't know how to fetch info for this asset");
            }
        }
    } else {
        println!("Don't know how to fetch info for this asset");
    }
}

pub async fn fetch_all(_args: &[String]) {
    println!("Fetching latest price info for all assets.");
    let connection = establish_connection();

    let class_tags = db::category::get_tags(&connection, "class");

    let assets = crate::asset::db::asset::filter(&connection, None);

    let mut stocks = vec![];
    let mut crypto = vec![];

    for asset in assets {
        let class_tag = get_class_tag_for_aset(&connection, &class_tags, &asset.id);
        if let Some(class_tag) = class_tag {
            match class_tag.tag.as_str() {
                "crypto" => {
                    crypto.push(asset.id);
                }
                "stock" => {
                    stocks.push(asset.id);
                }
                _ => {}
            }
        }
    }

    if !stocks.is_empty() {
        println!("Fetching latest price info for: {:?}.", stocks);
        let infos = api::stock::fetch_price_info(stocks.clone()).await.unwrap();
        for (asset, info) in stocks.into_iter().zip(infos) {
            db::info::delete(&connection, &asset);
            db::info::create(&connection, info);
        }
    }

    if !crypto.is_empty() {
        println!("Fetching latest price info for: {:?}.", crypto);
        let infos = api::crypto::fetch_price_info(crypto.clone()).await.unwrap();
        for info in infos {
            let asset = &info.asset;
            db::info::delete(&connection, &asset);
            db::info::create(&connection, info);
        }
    }
}
