use std::{pin::Pin, time::Duration};

use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};

use crate::{
  pb::getting::{
    v1::{user_server::User, GetUserRequest, StreamListRequest, UpdateUserRequest, UserDto},
    Empty,
  },
  utils::now_millis,
};

type UserDtoStream = Pin<Box<dyn Stream<Item = Result<UserDto, tonic::Status>> + Send>>;

pub struct UserService;

#[tonic::async_trait]
impl User for UserService {
  async fn get(&self, request: tonic::Request<GetUserRequest>) -> Result<tonic::Response<UserDto>, tonic::Status> {
    println!("The get user request: {:?}", request);

    Ok(tonic::Response::new(UserDto {
      id: 1,
      email: "test@example.com".to_string(),
      name: Some("Test User".to_string()),
      status: 1,
      ctime: now_millis(),
    }))
  }

  async fn update(&self, request: tonic::Request<UpdateUserRequest>) -> Result<tonic::Response<Empty>, tonic::Status> {
    println!("The update user request: {:?}", request);

    Ok(tonic::Response::new(Empty::default()))
  }

  type StreamListStream = UserDtoStream;

  async fn stream_list(
    &self,
    request: tonic::Request<StreamListRequest>,
  ) -> Result<tonic::Response<Self::StreamListStream>, tonic::Status> {
    println!("UserService::stream_list");
    println!("\tclient connected from: {:?}", request.remote_addr());

    let repeat = std::iter::repeat_with(|| UserDto {
      id: 1,
      email: "yangbajing@gmail.com".to_string(),
      status: 1,
      ctime: now_millis(),
      ..Default::default()
    });
    let mut stream = Box::pin(tokio_stream::iter(repeat).throttle(Duration::from_secs(2)));

    let (tx, rx) = mpsc::channel(16);
    tokio::spawn(async move {
      while let Some(item) = stream.next().await {
        match tx.send(Result::<_, tonic::Status>::Ok(item)).await {
          Ok(_) => {
            // item（服务器响应）已排队等待发送到客户端
          }
          Err(_) => {
            // output_stream 是从 rx 构建的，两者都被丢弃了
            break;
          }
        }
      }
      println!("\tclient disconnected");
    });

    let output_steam = ReceiverStream::new(rx);
    Ok(tonic::Response::new(Box::pin(output_steam)))
  }
}
