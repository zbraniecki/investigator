table! {
    coins (id) {
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
    passive_incomes (wallet, coin, kind) {
        wallet -> Varchar,
        coin -> Varchar,
        kind -> Varchar,
        apy -> Float8,
        apy_upper_bound -> Nullable<Float8>,
        start_date -> Nullable<Timestamp>,
        end_date -> Nullable<Timestamp>,
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
    sessions (id) {
        id -> Int4,
        identity -> Int4,
        expires -> Nullable<Timestamp>,
    }
}

table! {
    wallets (id) {
        id -> Varchar,
        name -> Varchar,
        url -> Nullable<Varchar>,
    }
}

joinable!(passive_incomes -> wallets (wallet));
joinable!(sessions -> identities (identity));

allow_tables_to_appear_in_same_query!(
    coins,
    identities,
    passive_incomes,
    prices,
    sessions,
    wallets,
);
