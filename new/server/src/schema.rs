table! {
    coins (id) {
        id -> Text,
        symbol -> Nullable<Text>,
        name -> Nullable<Text>,
    }
}

table! {
    prices (base, target) {
        base -> Text,
        target -> Text,
        ts -> Timestamp,
        value -> Double,
    }
}

allow_tables_to_appear_in_same_query!(
    coins,
    prices,
);
