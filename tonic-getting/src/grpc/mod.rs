use tonic::service::{Routes, RoutesBuilder};

use crate::pb::getting::v1::{auth_server::AuthServer, user_server::UserServer};

mod auth;
mod user;

pub use auth::AuthService;
pub use user::UserService;

static SESSION_TOKEN: &str = "L1AhTRgFMiTkQMuGf8PnY6yHAmaV72ESQsEzo0cVWmiodIEx";

pub fn make_grpc_routes() -> Routes {
  let mut rb = RoutesBuilder::default();
  rb.add_service(AuthServer::new(AuthService));
  rb.add_service(UserServer::with_interceptor(UserService, auth_interceptor));
  rb.routes()
}

pub fn auth_interceptor(request: tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status> {
  let authorization = request
    .metadata()
    .get("authorization")
    .ok_or_else(|| tonic::Status::unauthenticated("No auth token provided"))?
    .to_str()
    .map_err(|e| tonic::Status::unauthenticated(e.to_string()))?;
  let token = &authorization["Berere ".len()..];
  if token != SESSION_TOKEN {
    return Err(tonic::Status::unauthenticated("Invalid auth token"));
  }
  Ok(request)
}
