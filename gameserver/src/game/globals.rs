use std::env;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref LINEUP: Vec<u32> = {
        let lineup_str = env::var("LINEUP").expect("Missing .env parameter: LINEUP");
        let mut lineup = Vec::new();

        for s in lineup_str.trim().replace(" ", "").split(",") {
            lineup.push(s.parse().unwrap());
        }

        lineup
    };
}
