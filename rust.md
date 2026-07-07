
# 1. Rust 学习笔记

记忆力逐渐丧失，只有笔记才能避免原地踏步。

## 1.1. 环境配置

### 1.1.1. 安装配置 Rust

* 地址：<https://www.rust-lang.org/zh-CN/tools/install>
* 升级：`$ rustup update`
* 查看：`$ rustup show`
* 可能需要额外安装格式化工具：`$ rustup component add rustfmt`
* 卸载：`$ rustup self uninstall`

  * rustup 国内镜像
  
    ```bash
    export RUSTUP_UPDATE_ROOT=https://mirrors.tuna.tsinghua.edu.cn/rustup/rustup
    export RUSTUP_DIST_SERVER=https://mirrors.tuna.tsinghua.edu.cn/rustup
    ```

### 1.1.2. VSCode 插件

* 插件安装
  > rust-analyzer、CodeLLDB

### 1.1.3. cargo 启用清华源

* 编辑 `~/.cargo/config.toml`

```toml
[source.crates-io]
replace-with = 'mirror'

[source.mirror]
registry = "sparse+https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/"

[registries.mirror]
index = "sparse+https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/"
```

### 1.1.4. 清理`~/.cargo`缓存

  > `cargo install cargo-cache`

```shell
cargo cache -a #推荐
cargo-cache --remove-dir all
cargo-cache -r all
```

## 1.2. Rust 平台依赖库

### 1.2.1. macOS 平台

* 安装Xcode命令行工具（含git）

```shell
xcode-select --install #安装
xcode-select -p #验证
```

* 安装brew

```shell
/bin/zsh -c "$(curl -fsSL https://gitee.com/cunkai/HomebrewCN/raw/master/Homebrew.sh)"
```

* 可能缺失的依赖
  > pkg-config cmake ninja openssl

### 1.2.2. Linux 平台

* 可能缺失的依赖
  > gcc glibc llvm-libs libssl-dev

### 1.2.3. Android 平台

* 编译目标
  > rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android

* 安装cargo-ndk
  > cargo install cargo-ndk

### 1.2.4. iOS 平台

* 编译目标

  ```shell
  # 64 bit targets (real device & simulator):
  rustup target add aarch64-apple-ios x86_64-apple-ios

  # New simulator target for Xcode 12 and later
  rustup target add aarch64-apple-ios-sim
  ```

* macOS 与原生 iOS 代码交互
  > sudo gem install cocoapods

## 1.3. 工程配置

* 应用程序
  > cargo new rust-app
* 动态库
  > cargo new rust-lib --lib

## 1.4. 科学上网

### 1.4.1. HTTPS 从 github 拉流

* ssh改走https协议，走443端口，修改 .ssh/config

```shell
Host github.com
    Hostname ssh.github.com
    Port 443
    User git
```

### 1.4.2. Watt Toolkit 访问 github

* 下载地址：<https://github.com/BeyondDimension/SteamTools>
* Windows查看代理端口号：Windows设置-网络-代理，可以查看到端口号26501
* 开启pac代理模式，并一键加速
* 支持https克隆源码库

```shell
git config --global http.https://github.com.proxy 127.0.0.1:26501
git config --global http.https://github.com.sslverify false
git config --global --unset remote.origin.proxy
```

* 核实配置，可以

```shell
git config --list --global
```

* 如需重置，可以

```shell
git config --global --unset http.https://github.com.proxy
git config --global --unset http.https://github.com.sslverify
```
