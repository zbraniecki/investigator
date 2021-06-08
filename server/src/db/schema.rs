table! {
    asset_categories (id) {
        id -> Varchar,
        owner -> Nullable<Int4>,
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
        owner -> Nullable<Int4>,
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
    identities (id) {
        id -> Int4,
        name -> Varchar,
        password -> Varchar,
    }
}

table! {
    portfolio_assets (portfolio, asset) {
        portfolio -> Varchar,
        asset -> Varchar,
    }
}

table! {
    portfolios (id) {
        id -> Varchar,
        slug -> Varchar,
        name -> Nullable<Varchar>,
        owner -> Int4,
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
        name -> Nullable<Varchar>,
        url -> Nullable<Varchar>,
        owner -> Int4,
    }
}

table! {
    sessions (id) {
        id -> Int4,
        identity -> Int4,
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
        owner -> Int4,
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
