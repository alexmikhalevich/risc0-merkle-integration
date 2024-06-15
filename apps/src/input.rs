use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use clap::Parser;
use anyhow::Result;

#[derive(Clone, Deserialize, Serialize)]
pub struct Page {
    pub data: Vec<u8>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Inputs {
    pub pages: Vec<Page>,
    pub merkle_root: Vec<u8>,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    config: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    /// Ethereum chain ID
    pub chain_id: u64,
    /// Ethereum Node endpoint.
    pub eth_wallet_private_key: String,
    /// Ethereum Node endpoint.
    pub rpc_url: String,
    /// Application's contract address on Ethereum
    pub contract: String,
    /// Directory with the input memory pages
    pub directory: String,
    /// Prefix of the input memory pages files
    pub file_prefix: String,
    /// File with the original merkle root
    pub root_hash_file: String,
    /// Number of memory pages to read
    pub num_pages: usize,
    /// Size of each memory page
    pub page_size: usize,
}

fn read_file_content<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut content = Vec::new();
    file.read_to_end(&mut content)?;
    Ok(content)
}

pub fn load_config() -> Result<Config> {
    let args = Args::parse();
    let content = read_file_content(&args.config)?;
    Ok(serde_yaml::from_slice(&content)?)
}

pub fn read_input_data(config: &Config) -> Result<Inputs> {
    let mut pages = Vec::new();
    for i in 1..config.num_pages + 1 {
        let filename = format!("{}{}", config.file_prefix, i);
        let path = Path::new(&config.directory).join(filename);
        let data = read_file_content(path)?;
        pages.push(Page { data });
    }
    let root_hash_path = Path::new(&config.directory).join(&config.root_hash_file);
    let merkle_root = read_file_content(root_hash_path)?;
    Ok(Inputs { pages, merkle_root })
}
