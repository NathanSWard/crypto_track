use kraken::rest_api::response::AssetPairs;

fn main() -> failure::Fallible<()> {
    let req = reqwest::blocking::get("https://api.kraken.com/0/public/AssetPairs")?.text()?;
    let asset_pairs = serde_json::from_str::<AssetPairs>(req.as_str())?;

    Ok(())
}
