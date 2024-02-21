use serde::Deserialize;
use serde_json;
use std::fs;

#[derive(Deserialize, Debug, Clone)]
pub struct Asset {
    pub name: String,
    pub ticker: String,
    pub coingecko_id: String,
}

#[derive(Debug)]
pub struct AssetList {
    assets: Vec<Asset>,
}

impl AssetList {
    pub fn new() -> Self {
        let json_data = fs::read_to_string("assets.json").expect("Unable to read file");
        let asset_list: Vec<Asset> =
            serde_json::from_str(&json_data).expect("Unable to parse JSON");
        Self { assets: asset_list }
    }

    pub fn display_asset_selection(&self) {
        self.assets
            .iter()
            .enumerate()
            .for_each(|(count, asset)| println!("{}) {:?}", count + 1, asset.name));
        println!();
    }

    pub fn match_input_to_asset(&self, int_input: usize) -> Option<Asset> {
        self.assets.get(int_input - 1).cloned()
    }
}
