fn main() {
  println!("cargo::rerun-if-changed=proto/**/*.proto");

  tonic_build::configure()
    .compile(
      &[
        "proto/getting/basic.proto",
        "proto/getting/common/page.proto",
        "proto/getting/v1/auth.proto",
        "proto/getting/v1/user.proto",
      ],
      &["proto"],
    )
    .unwrap();
}
