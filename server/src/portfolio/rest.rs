use crate::asset;
use crate::db::establish_connection;
use crate::server::State;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct PortfolioInfo {
    pub portfolio: super::models::Portfolio,
    pub assets: Vec<asset::models::Asset>,
}

#[derive(Serialize, Deserialize)]
struct Portfolios(Vec<PortfolioInfo>);

#[derive(Deserialize)]
pub struct PriceViewQuery {
    #[serde(default)]
    _refresh: bool,
}

pub async fn filter(_data: web::Data<State>, _query: web::Query<PriceViewQuery>) -> HttpResponse {
    let connection = establish_connection();
    let portfolios = crate::portfolio::db::portfolio::filter(&connection);
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

            PortfolioInfo { portfolio, assets }
        })
        .collect::<Vec<_>>();
    // let mut r = vec![Portfolio {
    //     name: "S&P500".to_string(),
    //     assets: vec!["INTL".to_string(), "TSLA".to_string()],
    // }];
    // r.extend(result);
    let response = serde_json::to_string(&result).unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .body(response)
}
