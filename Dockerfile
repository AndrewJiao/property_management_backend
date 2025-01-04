# 使用官方的Rust镜像作为构建基础
FROM rust:1.83.0 AS builder

# 设置工作目录
WORKDIR /usr/src/app

# 复制项目源代码到构建环境中
COPY . .

# 安装 MUSL 工具链以生成静态二进制文件
#RUN rustup target add x86_64-unknown-linux-musl
#
#RUN apt update && apt install -y musl-tools musl-dev
#RUN apt update && apt install -y pkg-config libssl-dev
# 构建项目的所有模块，使用 --workspace 标志
RUN cargo build --release --workspace

# 使用一个轻量级的镜像作为运行基础
FROM debian:latest

# 设置工作目录
WORKDIR /usr/app

# 安装 OpenSSL 运行时库
RUN apt-get update && apt-get install -y libssl3 && rm -rf /var/lib/apt/lists/*

# 从构建阶段复制构建好的二进制文件到运行环境中
COPY --from=builder /usr/src/app/target/release/ /usr/app/

ENV env=prod
# Expose the port your application is running on
EXPOSE 9000

# 运行Rust应用程序
CMD ["./PropertyManagement"]