use std::fs::File;

use ipfs_api_backend_actix::{IpfsClient, IpfsApi, Error, response::AddResponse};

pub async fn upload_file() -> Result<String, Error> {
    let client = IpfsClient::default();
    let file = File::open("./file.json").expect("could not read source file");

    let AddResponse {name, ..} = client.add(file).await?;
    Ok(name)
}