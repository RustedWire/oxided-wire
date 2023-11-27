use rocket::{
    serde::{Deserialize, Serialize},
    tokio::{fs::OpenOptions, io, io::AsyncReadExt, io::AsyncWriteExt},
};

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub enum DataType {
    PUBKEY = 0,
    MESSAGE = 1,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Data {
    pub data_type: DataType,
    pub data: Vec<u8>,
}

pub async fn save_to_file(name: &str, data: &[u8]) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(false)
        .create(true)
        .open(name)
        .await
        .unwrap();

    file.write_all(data).await.unwrap();
}

pub async fn get_data_file(name: &str) -> Result<Vec<u8>, io::Error> {
    let file = OpenOptions::new().read(true).open(name).await;

    let mut file = match file {
        Ok(file) => file,
        Err(error) => return Err(error),
    };
    let mut buffer = Vec::<u8>::new();

    match file.read_to_end(&mut buffer).await {
        Ok(_) => (),
        Err(error) => return Err(error),
    }

    Ok(buffer)
}
