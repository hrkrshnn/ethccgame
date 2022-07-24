use ethers::prelude::*;
use ethers::prelude::{account::TokenQueryOption, Chain, PathOrString};
use ethers::signers::coins_bip39::English;
use ethers::signers::coins_bip39::Wordlist;

use ethers::etherscan::Client;

use eyre::Result;

use itertools::Itertools;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Hint {
    /// The length of the word
    len: usize,
    /// The index and the character
    hint: (usize, char),
    /// How the indices 8, 9, 10, 11 are permuted
    permute: (usize, usize, usize, usize),
}

impl Hint {
    fn new(len: usize, hint: (usize, char), permute: (usize, usize, usize, usize)) -> Self {
        Hint { len, hint, permute }
    }

    fn rule(self, str: String) -> bool {
        self.len == str.len() && str.chars().nth(self.hint.0) == Some(self.hint.1)
    }

    fn permute(self, words: &[String; 12]) -> [String; 12] {
        let mut words_copy = words.clone();
        let (a, b, c, d) = self.permute;
        words_copy[8] = words[a].clone();
        words_copy[9] = words[b].clone();
        words_copy[10] = words[c].clone();
        words_copy[11] = words[d].clone();

        words_copy
    }
}

fn make_phrase(str: &[String; 12]) -> String {
    format!("{}", str.iter().format(" "))
}

#[tokio::main]
async fn main() -> Result<()> {
    let words: [String; 12] = [
        "gorilla", "weird", "alien", "solid", "elephant", "none", "frog", "black", "pool",
        "eternal", "ghost", "escape",
    ]
    .map(|s| s.to_string())
    .into();

    // Need to set ETHERSCAN_API_KEY as env variable
    let client = Client::new_from_env(Chain::Mainnet)?;

    let hints: Vec<Hint> = vec![
        Hint::new(5, (3, 'e'), (8, 9, 11, 10)),
        Hint::new(5, (0, 'r'), (10, 8, 11, 9)),
        Hint::new(4, (3, 'y'), (10, 9, 8, 11)),
        Hint::new(5, (0, 'c'), (11, 8, 10, 9)),
        Hint::new(4, (2, 't'), (10, 11, 9, 8)),
    ];

    let mut phrases: Vec<String> = Vec::new();

    for hint in hints {
        for word in English::get_all() {
            let word = word.to_string();
            if hint.rule(word.clone()) {
                let mut phrase = hint.permute(&words);
                phrase[5] = word;
                phrases.push(make_phrase(&phrase));
            }
        }
    }

    let mut solutions = Vec::new();
    for phrase in phrases {
        let index = 0u32;

        let wallet = MnemonicBuilder::<English>::default()
            .phrase(PathOrString::String(phrase.clone()))
            .index(index)?
            .build();

        if let Ok(wallet) = wallet {
            let address = wallet.address();

            let erc1155_txs = client
                .get_erc1155_token_transfer_events(
                    TokenQueryOption::ByAddressAndContract(
                        address.clone(),
                        "0x495f947276749ce646f68ac8c248420045cb7b5e"
                            .parse()
                            .unwrap(),
                    ),
                    None,
                )
                .await;

            if let Ok(erc1155_txs) = erc1155_txs {
                if erc1155_txs.len() > 0 {
                    println!("Phrase: {:?}", phrase);
                    solutions.push(phrase);
                }
            }
        }
    }

    // TODO need to build flashbots integration and sending bundles

    Ok(())
}
