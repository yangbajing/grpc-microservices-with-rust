PROTO_DIR="./proto"
OUT_DIR="./src/pb"

# 创建输出目录
mkdir -p $OUT_DIR

protoc \
    --plugin=./node_modules/.bin/protoc-gen-ts_proto \
    --ts_proto_out=${OUT_DIR} \
    --ts_proto_opt=outputServices=nice-grpc,outputServices=generic-definitions,useExactTypes=false \
    -I ${PROTO_DIR} \
    ${PROTO_DIR}/getting/*.proto \
    ${PROTO_DIR}/getting/common/*.proto \
    ${PROTO_DIR}/getting/v1/*.proto
