use rocket::{
    serde::{Deserialize, Serialize},
    tokio::{fs::OpenOptions, io::AsyncWriteExt},
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
