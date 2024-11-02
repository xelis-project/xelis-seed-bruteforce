use clap::Parser;
use log::{info, warn};
use xelis_common::{
    config::VERSION,
    crypto::{Address, KeyPair},
    prompt::{LogLevel, Prompt}
};
use xelis_wallet::mnemonics::{self, LANGUAGES};

#[derive(Parser)]
#[clap(version = VERSION, about = "XELIS is an innovative cryptocurrency built from scratch with BlockDAG, Homomorphic Encryption, Zero-Knowledge Proofs, and Smart Contracts.")]
#[command(styles = xelis_common::get_cli_styles())]
pub struct Config {
    /// The seed with a missing word
    #[clap(short, long)]
    pub seed: String,
    /// The address to search for
    #[clap(short, long)]
    pub address: Address,
    /// Language index
    #[clap(short, long, default_value_t = 0)]
    pub language: usize,
}

fn main() {
    let config = Config::parse();
    if let Err(e) = Prompt::new(LogLevel::Info, "logs/", "logs.log", true, false, false, false, Vec::new(), LogLevel::Info) {
        eprintln!("Error: {}", e);
        return;
    }

    let language = &LANGUAGES[config.language];

    if !config.address.is_mainnet() {
        info!("The address must be a mainnet address");
        return;
    }

    if config.seed.split_whitespace().count() > 24 {
        info!("The seed must have 23 or 24 words");
        return;
    }

    let address = config.address.to_string();
    for pos in 0..24 {
        info!("Trying position {}", pos);
        for word in language.get_words() {
            let mut v = config.seed.split_whitespace().collect::<Vec<&str>>();
            // Insert the word at the position
            v.insert(pos, word);

            let Ok(key) = mnemonics::words_to_key(&v) else {
                continue;
            };

            let keypair = KeyPair::from_private_key(key);
            let addr = keypair.get_public_key().to_address(true).to_string();
            if addr == address {
                info!("Found address: {}", addr);
                info!("Seed: {}", v.join(" "));
                return;
            }
        }
    }

    warn!("Address not found");
}
