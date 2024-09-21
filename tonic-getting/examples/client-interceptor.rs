use tonic::{transport::Channel, Code, Request, Status};
use tonic_getting::pb::getting::v1::{user_client::UserClient, UpdateUserRequest};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let channel = Channel::from_static("http://localhost:9999").connect().await?;

  let mut client = UserClient::with_interceptor(channel, |mut request: Request<()>| {
    let token = "Bearer L1AhTRgFMiTkQMuGf8PnY6yHAmaV72ESQsEzo0cVWmiodIEx"
      .parse()
      .map_err(|_| Status::new(Code::Internal, "InvalidMetadataValue"))?;
    request.metadata_mut().insert("authorization", token);
    Ok(request)
  });

  let request = Request::new(UpdateUserRequest { id: 1, name: Some("yangbajing".to_string()), status: Some(100) });

  let response = client.update(request).await?;

  println!("Response: {:?}", response);

  Ok(())
}
