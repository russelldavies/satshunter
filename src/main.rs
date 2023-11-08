use clap::Parser;
use itertools::Itertools;
use rayon::prelude::*;

use bdk::bitcoin::Network;
use bdk::database::MemoryDatabase;
use bdk::keys::{
    bip39::{Language, Mnemonic},
    DerivableKey, ExtendedKey,
};
use bdk::template::Bip84;
use bdk::{KeychainKind, Wallet};

const NETWORK: Network = Network::Bitcoin;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    words: String,

    #[arg(short, long)]
    address: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 10)]
    depth: u8,
}

fn main() {
    let args = Args::parse();

    let words = args.words.split_whitespace().collect_vec();
    let num_words = words.len();
    let perms_iter = words.into_iter().permutations(num_words);
    let result = perms_iter.par_bridge().find_any(|words| {
        match Mnemonic::parse_in_normalized(Language::English, &words.join(" ")) {
            Err(_) => return false,
            Ok(mnemonic) => {
                let xkey: ExtendedKey = mnemonic.into_extended_key().unwrap();
                let xprv = xkey.into_xprv(NETWORK).unwrap();
                // Create a BDK wallet structure using BIP 84 descriptor ("m/84h/1h/0h/0" and "m/84h/1h/0h/1")
                let wallet = Wallet::new(
                    Bip84(xprv, KeychainKind::External),
                    Some(Bip84(xprv, KeychainKind::Internal)),
                    NETWORK,
                    MemoryDatabase::default(),
                )
                .unwrap();
                for _ in 0..args.depth {
                    let address = wallet.get_address(bdk::wallet::AddressIndex::New).unwrap();
                    if address.address.to_string() == args.address {
                        return true;
                    }
                }
            }
        }
        return false;
    });
    match result {
        None => std::process::exit(1),
        Some(words) => {
            println!("{}", words.join(" "));
            std::process::exit(0)
        }
    }
}
