use clap::Parser;
use serde::Deserialize;
use std::{collections::HashMap, error::Error};

#[derive(Parser, Debug)]
#[command(name = "conv", version, about = "Crypto to USD converter")]
struct Args {
    
    #[arg(long)]
    xmr: bool,

   
    #[arg(long)]
    btc: bool,

    
    #[arg(long)]
    ltc: bool,

    
    #[arg(long)]
    eth: bool,

    
    #[arg(long)]
    sol: bool,

    
    #[arg(long)]
    xrp: bool,
}

#[derive(Deserialize)]
struct PriceResponse {
    #[serde(flatten)]
    prices: HashMap<String, HashMap<String, f64>>,
}

fn fetch_price(crypto_id: &str) -> Result<f64, Box<dyn Error>> {
    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd",
        crypto_id
    );
    let response: PriceResponse = reqwest::blocking::get(&url)?.json()?;
    Ok(response.prices[crypto_id]["usd"])
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let currencies = [
        ("xmr", args.xmr),
        ("btc", args.btc),
        ("ltc", args.ltc),
        ("eth", args.eth),
        ("sol", args.sol),
        ("xrp", args.xrp),
    ];

    for (crypto, selected) in currencies {
        if selected {
            let name_map = [
                ("xmr", "monero"),
                ("btc", "bitcoin"),
                ("ltc", "litecoin"),
                ("eth", "ethereum"),
                ("sol", "solana"),
                ("xrp", "ripple"),
            ];
            let id = name_map.iter().find(|(key, _)| key == &crypto).unwrap().1;
            let price = fetch_price(id)?;
            println!("1 {} is worth {:.2} USD", crypto.to_uppercase(), price);
        }
    }

    Ok(())
}
