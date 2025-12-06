use crate::models::admin::MarketMetadata;
use anyhow::Result;
use aws_sdk_s3::{primitives::ByteStream, types::ObjectCannedAcl};


pub async fn upload_market_metadata_to_do(
    s3_client: &aws_sdk_s3::Client,
    bucket_name: &str,
    market_id: u64,
    metadata: &MarketMetadata
) -> Result<String> {
    let key = format!("market_metadata/{}.json", market_id);
    let metadata_json = serde_json::to_vec(metadata)?;

    let url = s3_client
        .put_object()
        .bucket(bucket_name)
        .key(&key)
        .body(ByteStream::from(metadata_json))
        .content_type("application/json")
        .acl(ObjectCannedAcl::PublicRead)
        .send()
        .await?;
    dbg!("Uploaded metadata to S3 with key:", &key);
    dbg!("S3 PutObject output:", &url);
    let metadata_url = format!("https://{}.{}.digitaloceanspaces.com/{}/{}", bucket_name, std::env::var("DO_SPACES_REGION")?, bucket_name, key);
    Ok(metadata_url)
}