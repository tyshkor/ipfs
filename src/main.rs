use std::path::PathBuf;

use clap::Parser;

mod ipfs_interactions;
mod eth_interactions;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {

    #[clap(long, parse(from_os_str))]
    file_path: PathBuf,
}

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let file_cid = ipfs_interactions::upload_file(args.file_path).await?;
    eth_interactions::deploy_contract(file_cid).await?;
    Ok(())
}
