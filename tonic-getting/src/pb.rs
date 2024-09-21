pub mod getting {
  tonic::include_proto!("getting");
  pub mod common {
    tonic::include_proto!("getting.common");
  }
  pub mod v1 {
    tonic::include_proto!("getting.v1");
  }
}

// `cargo test pb::tests -q`
#[cfg(test)]
mod tests {
  use super::getting::common::*;
  use super::getting::v1::*;

  #[test]
  fn test_user() {
    let pagination = Pagination { page: 1, page_size: 20, ..Default::default() };
    let page_user_request = PageUserRequest { pagination: Some(pagination) };
    println!("Page user request is {:?}", page_user_request);
  }
}
