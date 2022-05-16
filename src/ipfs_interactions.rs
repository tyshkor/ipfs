use std::{fs::File, path::PathBuf};

use ipfs_api_backend_actix::{IpfsClient, IpfsApi, Error, response::AddResponse};

pub async fn upload_file(path: PathBuf) -> Result<String, Error> {
    let client = IpfsClient::default();
    let file = File::open(path).expect("could not read source file");

    let AddResponse {name, ..} = client.add(file).await?;
    Ok(name)
}