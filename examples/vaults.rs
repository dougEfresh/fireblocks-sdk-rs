use {
    fireblocks_sdk::{apis::vaults_api::GetVaultAccountParams, ClientBuilder},
    std::{fs::File, io::Read, time::Duration},
};

fn load_secret() -> anyhow::Result<Vec<u8>> {
    match std::env::var("FIREBLOCKS_SECRET").ok() {
        Some(secret) => Ok(secret.into_bytes()),
        None => {
            let secret = std::env::var("FIREBLOCKS_SECRET_FILE")
                .expect("failed find secret key in FIREBLOCKS_SECRET or FIREBLOCKS_SECRET_FILE");
            let mut file = File::open(secret).expect("file not found");
            let mut secret: String = String::new();
            file.read_to_string(&mut secret)
                .expect("something went wrong reading the file");
            Ok(secret.into_bytes())
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api_key = std::env::var("FIREBLOCKS_API_KEY")?;
    let secret = load_secret()?;
    let client = ClientBuilder::new(&api_key, &secret)
        .with_sandbox()
        .with_timeout(Duration::from_secs(10))
        .with_connect_timeout(Duration::from_secs(5))
        .build()?;

    let id = String::from("0");
    let params = GetVaultAccountParams {
        vault_account_id: id.clone(),
    };
    let vault_account = client.vaults_api().get_vault_account(params).await?;
    println!("vault account: {vault_account:#?}");
    Ok(())
}
