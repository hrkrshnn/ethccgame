use ethers::prelude::PathOrString;
use ethers::prelude::*;
use ethers::signers::coins_bip39::English;
use ethers::signers::coins_bip39::Wordlist;

use eyre::Result;

use itertools::Itertools;

use std::{convert::TryFrom, time::Duration};

fn make_phrase(str: &[String; 12]) -> String {
    format!("{}", str.iter().format(" "))
}

fn match_word(name: String) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    for phrase in English::get_all() {
        if phrase.contains(&name) {
            res.push(phrase.to_string());
        }
    }
    res
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Running the cruncher");
    let words: [String; 12] = [
        "mom", "good", "law", "joy", "rib", "end", "sad", "gun", "boy", "lum", "dog", "",
    ]
    .map(|s| s.to_string())
    .into();

    let lums: Vec<String> = ["lumber", "clump", "column", "volume"]
        .map(|s| s.to_string())
        .into();
    // let words: [String; 12] = [
    //     "mom", "joy", "sad", "lumber", "good", "rib", "gun", "dog", "law",
    //     "end", "boy", "xxxx",
    // ]
    //     .map(|s| s.to_string())
    //     .into();

    // let words: [String; 12] = [
    //     "gorilla", "weird", "alien", "solid", "elephant", "none", "frog", "black", "pool",
    //     "eternal", "ghost", "escape",
    // ]
    // .map(|s| s.to_string())
    // .into();

    // Need to set ETHERSCAN_API_KEY as env variable
    let http_endpoint = std::env::var("ETH_RPC_URL")?;

    let provider =
        Provider::<Http>::try_from(http_endpoint.clone())?.interval(Duration::from_millis(10u64));

    for word in English::get_all() {
        let word = word.to_string();
        let mut phrase = words.clone();
        phrase[11] = word;

        for lum in &lums {
            phrase[3] = lum.to_string();
            let phrase = make_phrase(&phrase);
            let wallet = MnemonicBuilder::<English>::default()
                .phrase(PathOrString::String(phrase.clone()))
                .index(1u32)?
                .build();
            if let Ok(wallet) = wallet {
                let balance = provider.get_balance(wallet.address(), None).await?;
                if balance > U256::from(0) {
                    println!("wallet address: {:?}", wallet.address());
                    println!("Balance: {:?}", balance);
                    return Ok(());
                }
            }
        }
    }

    Ok(())
}

#[test]
fn test_match() {
    let words: [String; 12] = [
        "mom", "good", "law", "joy", "rib", "end", "sad", "gun", "boy", "lum", "dog", "xxx",
    ]
    .map(|s| s.to_string())
    .into();

    for word in words {
        println!("{:?}", match_word(word));
    }
}
