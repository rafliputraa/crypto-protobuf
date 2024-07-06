pub mod crypto {
    pub mod watchlist {
        include!(concat!(env!("CARGO_MANIFEST_DIR"), "/protobuf/crypto_watchlist.rs"));
    }
}