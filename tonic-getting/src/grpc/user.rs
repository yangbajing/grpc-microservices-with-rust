use crate::pb::getting::{
  v1::{user_server::User, GetUserRequest, UpdateUserRequest, UserDto},
  Empty,
};

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
    }))
  }

  async fn update(&self, request: tonic::Request<UpdateUserRequest>) -> Result<tonic::Response<Empty>, tonic::Status> {
    println!("The update user request: {:?}", request);

    Ok(tonic::Response::new(Empty::default()))
  }
}
