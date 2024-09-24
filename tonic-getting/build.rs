fn main() {
  tonic_build::configure()
    // 显示启用当 .proto 文件变化时自动重编译
    .emit_rerun_if_changed(true)
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
