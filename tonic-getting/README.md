# tonic-getting

## 测试 gRPC 服务

```sh
cargo run -q
```

### 使用 `grpcurl` 测试 gRPC 服务

```sh
grpcurl -plaintext -import-path ./proto \
  -proto getting/v1/auth.proto \
  -d '{
    "email": "yangbajing@gmail.com",
    "password": "Password.2024"
  }' \
  localhost:9999 getting.v1.Auth/Signin
```

```sh
grpcurl -plaintext -import-path ./proto \
  -proto getting/v1/user.proto \
  -d '{
    "id": 1,
    "name": "yangbajing"
  }' \
  localhost:9999 getting.v1.User/Update
```
