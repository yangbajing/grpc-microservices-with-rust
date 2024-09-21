use tonic::transport::Server;
use tonic_getting::grpc::make_grpc_routes;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  tracing_subscriber::fmt::init();

  let grpc_addr = "0.0.0.0:9999".parse()?;
  println!("The gRPC Server listening to {}", grpc_addr);

  Server::builder()
    .layer(TraceLayer::new_for_grpc())
    .accept_http1(true)
    .layer(tonic_web::GrpcWebLayer::new())
    .add_routes(make_grpc_routes())
    .serve(grpc_addr)
    .await?;

  Ok(())
}
