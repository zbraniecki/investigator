use actix_web::{get, web, App, HttpResponse, HttpServer};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use std::fs;

static PRICE_URL: &str = "https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd&ids={IDS}&order=market_cap_desc&per_page=100&page=1&sparkline=false";
static COIN_LIST_URL: &str = "https://api.coingecko.com/api/v3/coins/list?include_platform=false";

// {
//   "id": "bitcoin",
//   "symbol": "btc",
//   "name": "Bitcoin",
//   "image": "https://assets.coingecko.com/coins/images/1/large/bitcoin.png?1547033579",
//   "current_price": 55489,
//   "market_cap": 1039411052850,
//   "market_cap_rank": 1,
//   "fully_diluted_valuation": 1168036950459,
//   "total_volume": 63005120513,
//   "high_24h": 57270,
//   "low_24h": 53820,
//   "price_change_24h": 1668.52,
//   "price_change_percentage_24h": 3.10018,
//   "market_cap_change_24h": 27002465431,
//   "market_cap_change_percentage_24h": 2.66715,
//   "circulating_supply": 18687450,
//   "total_supply": 21000000,
//   "max_supply": 21000000,
//   "ath": 64805,
//   "ath_change_percentage": -14.49002,
//   "ath_date": "2021-04-14T11:54:46.763Z",
//   "atl": 67.81,
//   "atl_change_percentage": 81621.46126,
//   "atl_date": "2013-07-06T00:00:00.000Z",
//   "roi": null,
//   "last_updated": "2021-04-21T07:36:34.511Z"
// },
#[derive(Serialize, Deserialize, Clone)]
struct Price {
    pub pair: (String, String),
    pub value: f64,
}

#[derive(Serialize, Deserialize, Clone)]
struct Coin {
    pub id: String,
    pub symbol: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Holding {
    pub symbol: String,
    pub quantity: f64,
}

#[derive(Serialize, Deserialize, Clone)]
struct Target {
    pub symbol: String,
    pub percent: f64,
}

struct ServerState {
    pub prices: Vec<Price>,
    pub coins: Vec<Coin>,
    pub portfolio: Vec<Holding>,
    pub target: Vec<Target>,
}

async fn fetch_prices(coins: &[Coin]) -> Vec<Price> {
    #[derive(Serialize, Deserialize, Clone)]
    struct ExternalPrice {
        symbol: String,
        current_price: f64,
    }
    use actix_web::client::Client;
    let client = Client::default();

    let ids = coins
        .iter()
        .map(|coin| coin.id.to_string())
        .collect::<Vec<_>>()
        .join("%2C");
    let price_url = PRICE_URL.replace("{IDS}", &ids);

    let mut resp = client
        .get(price_url)
        .header("User-Agent", "Actix-web")
        .send()
        .await
        .unwrap();

    let body = resp.body().await.unwrap();

    let prices: Vec<ExternalPrice> = serde_json::from_slice(&body).unwrap();

    prices
        .into_iter()
        .map(|price| Price {
            pair: (price.symbol, "USD".to_string()),
            value: price.current_price,
        })
        .collect()
}

async fn read_prices(coins: &[Coin]) -> Vec<Price> {
    #[derive(Serialize, Deserialize)]
    struct PriceList {
        price: Vec<Price>,
    }

    let path = "res/prices.toml";

    if !fs::metadata(path).is_ok() {
        let prices = fetch_prices(coins).await;
        let price_list = PriceList { price: prices };
        let toml_string = toml::to_string(&price_list).unwrap();
        fs::write(path, toml_string).expect("Could not write to file!");
        price_list.price
    } else {
        let source = fs::read_to_string(path).expect("Something went wrong reading the file");
        let result: PriceList = toml::from_str(&source).unwrap();
        result.price
    }
}

async fn fetch_coins() -> Vec<Coin> {
    use actix_web::client::Client;
    let client = Client::default();

    let mut resp = client
        .get(COIN_LIST_URL) // <--- notice the "s" in "https://..."
        .header("User-Agent", "Actix-web")
        .send()
        .await
        .unwrap();

    let body = resp.body().limit(51200000).await.unwrap();

    serde_json::from_slice(&body).unwrap()
}

async fn read_coins() -> Vec<Coin> {
    #[derive(Serialize, Deserialize)]
    struct CoinList {
        coin: Vec<Coin>,
    }

    let path = "res/coins.toml";

    if !fs::metadata(path).is_ok() {
        let coins = fetch_coins().await;
        // let coins = vec![];
        let coin_list = CoinList { coin: coins };
        let toml_string = toml::to_string(&coin_list).unwrap();
        fs::write(path, toml_string).expect("Could not write to file!");
        coin_list.coin
    } else {
        let source = fs::read_to_string(path).expect("Something went wrong reading the file");
        let result: CoinList = toml::from_str(&source).unwrap();
        result.coin
    }
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum CoinID {
    Symbol(String),
    SymbolAndID((String, String)),
}

fn read_supported_coins() -> Vec<CoinID> {
    #[derive(Serialize, Deserialize)]
    struct CoinList {
        coins: Vec<CoinID>,
    }

    let path = "res/supported_coins.toml";

    if !fs::metadata(path).is_ok() {
        vec![]
    } else {
        let source = fs::read_to_string(path).expect("Something went wrong reading the file");
        let result: CoinList = toml::from_str(&source).unwrap();
        result.coins
    }
}

async fn get_portfolio() -> Vec<Holding> {
    #[derive(Serialize, Deserialize)]
    struct HoldingList {
        holding: Vec<Holding>,
    }

    let path = "res/portfolio.toml";

    if !fs::metadata(path).is_ok() {
        vec![]
    } else {
        let source = fs::read_to_string(path).expect("Something went wrong reading the file");
        let result: HoldingList = toml::from_str(&source).unwrap();
        result.holding
    }
}

async fn get_target() -> Vec<Target> {
    #[derive(Serialize, Deserialize)]
    struct TargetList {
        coin: Vec<Target>,
    }

    let path = "res/target.toml";

    if !fs::metadata(path).is_ok() {
        vec![]
    } else {
        let source = fs::read_to_string(path).expect("Something went wrong reading the file");
        let result: TargetList = toml::from_str(&source).unwrap();
        result.coin
    }
}

async fn get_supported_coins() -> Vec<Coin> {
    let all_coins = read_coins().await;
    let supported_coins = read_supported_coins();

    all_coins
        .into_iter()
        .filter(|coin| {
            supported_coins
                .iter()
                .find(|c| match c {
                    CoinID::Symbol(c) => c == &coin.symbol,
                    CoinID::SymbolAndID((s, id)) => s == &coin.symbol && id == &coin.id,
                })
                .is_some()
        })
        .collect()
}

#[get("/coins")]
async fn get_coins(data: web::Data<ServerState>) -> HttpResponse {
    let coins = &data.coins;
    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(coins).unwrap())
}

#[get("/prices")]
async fn index(data: web::Data<ServerState>) -> HttpResponse {
    let prices = &data.prices;
    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(prices).unwrap())
}

#[get("/portfolio")]
async fn serve_portfolio(data: web::Data<ServerState>) -> HttpResponse {
    let portfolio = &data.portfolio;
    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(portfolio).unwrap())
}

#[get("/target")]
async fn serve_target(data: web::Data<ServerState>) -> HttpResponse {
    let target = &data.target;
    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(target).unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let coins = get_supported_coins().await;
    let prices = read_prices(&coins).await;
    let portfolio = get_portfolio().await;
    let target = get_target().await;

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:1234")
            .allowed_origin("http://127.0.0.1:1234")
            .supports_credentials();
        App::new()
            .wrap(cors)
            .data(ServerState {
                prices: prices.clone(),
                coins: coins.clone(),
                portfolio: portfolio.clone(),
                target: target.clone(),
            })
            .service(index)
            .service(get_coins)
            .service(serve_portfolio)
            .service(serve_target)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
