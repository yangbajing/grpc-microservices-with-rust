use crate::{
  grpc::SESSION_TOKEN,
  pb::getting::v1::{auth_server::Auth, SigninRequest, SigninResponse, SignupRequest, SignupResponse, TokenType},
};

pub struct AuthService;

#[tonic::async_trait]
impl Auth for AuthService {
  async fn signin(
    &self,
    request: tonic::Request<SigninRequest>,
  ) -> Result<tonic::Response<SigninResponse>, tonic::Status> {
    let req = request.into_inner();
    println!("The signin request is {:?}", req);

    let resp = SigninResponse { access_token: SESSION_TOKEN.to_string(), token_type: TokenType::Bearer as i32 };
    Ok(tonic::Response::new(resp))
  }

  async fn signup(
    &self,
    request: tonic::Request<SignupRequest>,
  ) -> Result<tonic::Response<SignupResponse>, tonic::Status> {
    let req = request.into_inner();
    println!("Signup request is {:?}", req);

    let resp = SignupResponse { code: 0, ..Default::default() };
    Ok(tonic::Response::new(resp))
  }
}
