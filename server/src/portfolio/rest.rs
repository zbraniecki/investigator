use crate::asset;
use crate::db::establish_connection;
use crate::server::State;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct PortfolioInfo {
    pub portfolio: super::models::Portfolio,
    pub assets: Vec<AssetInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
struct AssetInfo {
    pub asset: asset::models::Asset,
    pub info: Option<asset::models::AssetInfo>,
}

#[derive(Serialize, Deserialize)]
struct Portfolios(Vec<PortfolioInfo>);

#[derive(Deserialize)]
pub struct PortfolioFilterQuery {
    #[serde(default)]
    owner: Option<i64>,
}

pub async fn filter(
    _data: web::Data<State>,
    query: web::Query<PortfolioFilterQuery>,
) -> HttpResponse {
    let connection = establish_connection();
    let portfolios = crate::portfolio::db::portfolio::filter(&connection, query.owner);
    let result = portfolios
        .into_iter()
        .map(|portfolio| {
            let portfolio_assets =
                crate::portfolio::db::portfolio_assets::filter(&connection, portfolio.id);
            let asset_ids = portfolio_assets
                .iter()
                .map(|pa| pa.asset.as_str())
                .collect::<Vec<_>>();

            let assets = crate::asset::db::asset::filter(&connection, Some(asset_ids));
            let asset_ids: Vec<&str> = assets.iter().map(|a| a.id.as_str()).collect();
            let infos = crate::asset::db::info::filter(&connection, asset_ids);

            let asset_infos = assets
                .into_iter()
                .map(|asset| {
                    let info = infos.iter().find(|info| info.asset == asset.id).cloned();
                    AssetInfo { asset, info }
                })
                .collect();

            PortfolioInfo {
                portfolio,
                assets: asset_infos,
            }
        })
        .collect::<Vec<_>>();
    let response = serde_json::to_string(&result).unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .body(response)
}

#[derive(Deserialize)]
pub struct PortfolioCreateQuery {
    #[serde(default)]
    owner: Option<i64>,
    name: String,
    slug: String,
}

pub async fn create(
    _data: web::Data<State>,
    query: web::Query<PortfolioCreateQuery>,
) -> HttpResponse {
    let connection = establish_connection();
    crate::portfolio::db::portfolio::create(&connection, &query.slug, &query.name, query.owner);

    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
pub struct PortfolioDeleteQuery {
    id: i64,
}

pub async fn delete(
    _data: web::Data<State>,
    query: web::Query<PortfolioDeleteQuery>,
) -> HttpResponse {
    let connection = establish_connection();
    crate::portfolio::db::portfolio::delete(&connection, query.id);

    HttpResponse::Ok().finish()
}
