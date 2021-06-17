table! {
    asset_categories (id) {
        id -> Varchar,
        owner -> Nullable<Int8>,
    }
}

table! {
    asset_tag_categories (tag, category) {
        tag -> Varchar,
        category -> Varchar,
    }
}

table! {
    asset_tags (id) {
        id -> Varchar,
        owner -> Nullable<Int8>,
    }
}

table! {
    asset_tags_intermediate (tag, asset) {
        tag -> Varchar,
        asset -> Varchar,
    }
}

table! {
    assets (id) {
        id -> Varchar,
        symbol -> Nullable<Varchar>,
        name -> Nullable<Varchar>,
    }
}

table! {
    assets_info (asset, reference_asset) {
        asset -> Varchar,
        reference_asset -> Varchar,
        current_price -> Nullable<Float8>,
        market_cap -> Nullable<Int8>,
        market_cap_rank -> Nullable<Int8>,
        total_volume -> Nullable<Int8>,
        high_24h -> Nullable<Float8>,
        low_24h -> Nullable<Float8>,
        price_change_24h -> Nullable<Float8>,
        market_cap_change_24h -> Nullable<Float8>,
        market_cap_change_percentage_24h -> Nullable<Float8>,
        circulating_supply -> Nullable<Float8>,
        total_supply -> Nullable<Float8>,
        max_supply -> Nullable<Float8>,
        ath -> Nullable<Float8>,
        ath_change_percentage -> Nullable<Float8>,
        ath_date -> Nullable<Timestamptz>,
        atl -> Nullable<Float8>,
        atl_change_percentage -> Nullable<Float8>,
        atl_date -> Nullable<Timestamptz>,
        last_updated -> Nullable<Timestamptz>,
        price_change_percentage_1h -> Nullable<Float8>,
        price_change_percentage_24h -> Nullable<Float8>,
        price_change_percentage_7d -> Nullable<Float8>,
        price_change_percentage_14d -> Nullable<Float8>,
        price_change_percentage_30d -> Nullable<Float8>,
        price_change_percentage_200d -> Nullable<Float8>,
        price_change_percentage_1y -> Nullable<Float8>,
    }
}

table! {
    identities (id) {
        id -> Int8,
        name -> Varchar,
        password -> Varchar,
    }
}

table! {
    portfolio_assets (portfolio, asset) {
        portfolio -> Int8,
        asset -> Varchar,
    }
}

table! {
    portfolios (id) {
        id -> Int8,
        slug -> Varchar,
        name -> Nullable<Varchar>,
        owner -> Nullable<Int8>,
    }
}

table! {
    prices (base, target, ts) {
        base -> Varchar,
        target -> Varchar,
        ts -> Timestamp,
        value -> Float8,
    }
}

table! {
    services (id) {
        id -> Varchar,
        name -> Varchar,
        url -> Nullable<Varchar>,
        owner -> Nullable<Int8>,
    }
}

table! {
    sessions (id) {
        id -> Int8,
        identity -> Int8,
        expires -> Nullable<Timestamp>,
    }
}

table! {
    wallet_yield_kinds (id) {
        id -> Varchar,
    }
}

table! {
    wallet_yields (wallet, service, asset) {
        service -> Varchar,
        wallet -> Varchar,
        asset -> Varchar,
        kind -> Varchar,
        apy_lower_bound -> Float8,
        apy_upper_bound -> Nullable<Float8>,
        start_date -> Nullable<Timestamp>,
        end_date -> Nullable<Timestamp>,
    }
}

table! {
    wallets (id) {
        id -> Varchar,
        name -> Varchar,
        url -> Nullable<Varchar>,
        service -> Nullable<Varchar>,
        owner -> Nullable<Int8>,
    }
}

joinable!(asset_categories -> identities (owner));
joinable!(asset_tag_categories -> asset_categories (category));
joinable!(asset_tag_categories -> asset_tags (tag));
joinable!(asset_tags -> identities (owner));
joinable!(asset_tags_intermediate -> asset_tags (tag));
joinable!(asset_tags_intermediate -> assets (asset));
joinable!(portfolio_assets -> assets (asset));
joinable!(portfolio_assets -> portfolios (portfolio));
joinable!(portfolios -> identities (owner));
joinable!(services -> identities (owner));
joinable!(sessions -> identities (identity));
joinable!(wallet_yields -> assets (asset));
joinable!(wallet_yields -> services (service));
joinable!(wallet_yields -> wallet_yield_kinds (kind));
joinable!(wallet_yields -> wallets (wallet));
joinable!(wallets -> identities (owner));
joinable!(wallets -> services (service));

allow_tables_to_appear_in_same_query!(
    asset_categories,
    asset_tag_categories,
    asset_tags,
    asset_tags_intermediate,
    assets,
    assets_info,
    identities,
    portfolio_assets,
    portfolios,
    prices,
    services,
    sessions,
    wallet_yield_kinds,
    wallet_yields,
    wallets,
);
