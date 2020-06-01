use rusoto_core::{HttpClient, Region};
use rusoto_credential::{ChainProvider, ProfileProvider};
use rusoto_s3::{GetObjectRequest, PutObjectRequest, S3Client, S3};
use std::env;
use tokio::io::AsyncReadExt;

fn client() -> S3Client {
    let mut profile_provider = ProfileProvider::new().unwrap();
    profile_provider.set_profile(String::from("private"));
    let credentials = ChainProvider::with_profile_provider(profile_provider);
    return S3Client::new_with(
        HttpClient::new().expect("failed to create request dispatcher"),
        credentials,
        Region::EuCentral1,
    );
}

pub async fn put_object(key: String, body: String) {
    client()
        .put_object(PutObjectRequest {
            bucket: bucket(),
            key: key,
            body: Some(body.into_bytes().into()),
            ..Default::default()
        })
        .await
        .unwrap();
}

pub async fn get_object(key: String) -> String {
    let result = client()
        .get_object(GetObjectRequest {
            bucket: bucket(),
            key: key,
            ..Default::default()
        })
        .await
        .unwrap();

    let mut stream = result.body.unwrap().into_async_read();
    let mut buffer = String::new();

    stream.read_to_string(&mut buffer).await.unwrap();
    return buffer;
}

pub fn bucket() -> String {
    match env::var("S3_BUCKET") {
        Ok(val) => return val,
        Err(e) => return String::from("bng-rlgql-5s4e0wl2l"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn write_string() {
        put_object(String::from("test_s3.txt"), String::from("test")).await;
    }

    #[tokio::test]
    async fn read_string() {
        let val = get_object(String::from("test_s3.txt")).await;
        assert_eq!(val, String::from("test"));
    }
}
