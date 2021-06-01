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

joinable!(sessions -> identities (identity));

allow_tables_to_appear_in_same_query!(
    coins,
    identities,
    prices,
    sessions,
);
