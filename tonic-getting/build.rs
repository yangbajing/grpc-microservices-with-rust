use std::{env, path::PathBuf};

fn main() {
  let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

  tonic_build::configure()
    // 显示启用当 .proto 文件变化时自动重编译
    .emit_rerun_if_changed(true)
    .build_server(true)
    .build_client(true)
    // 生成描述符文件，当使用 gRPC Reflection 功能时可以从这个文件中获取服务描述信息来返回给调用方
    .file_descriptor_set_path(out_dir.join("getting_descriptor.bin"))
    // 设置代码生成器输出目录，默认为 OUT_DIR 环境变量指定目录
    .out_dir(out_dir)
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
