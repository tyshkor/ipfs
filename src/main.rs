// use ipfs_api::{IpfsApi, IpfsClient};
use std::{fs::File, time};

use ipfs_api_backend_actix::{IpfsClient, IpfsApi, Error, response::AddResponse};

use web3::contract::{Contract, Options};

mod ipfs_interactions;
mod eth_interactions;

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_cid = ipfs_interactions::upload_file().await?;
    eth_interactions::deploy_contract(file_cid).await?;
    Ok(())
}
