<img src="logo.png" width="1024px" height="180px" />

# 🚗 ToyEVM

![Rust](https://github.com/Akatsuki-py/ToyEVM/workflows/Rust/badge.svg)

簡易的な EVM(Ethereum Virtual Machine)を実装することを目標とした Rust 製のレポジトリです。

## ⛓ Ethereum Virtual Machine

EVM についての説明は[こちら](./guide.md)にまとめてあります。

## 🎮 Usage

```sh
# ビルド & 起動
$ make build   # require make
$ ./toyevm run # toyevm.exe on Windows

# トランザクションの実行
$ ./toyevm run

world state: 8023458051e2611dea07a6f9d2dbbcfb1863079eac68f15e01c6c3853aeaec3c

select next action: transaction(1) or deploy(2) => 1
contract address    > 899C5C9bf8396Ba2c14f819C6D807b96990F86EE
sender address      > 9C2b303267DcFc6F247E777f1e412a2b08E57998

# コントラクトのデプロイ
$ ./toyevm run

world state: c52dc54dfb0dabdeddac0f609b330404178d216e5091f938c4577268ac21027e

select next action: transaction(1) or deploy(2) => 2
contract code      > 61010161010201

109361e7a47f44bd357611501efd6eaa35d252a2 is deployed!
```

EVM バイトコードを逆アセンブルする機能も備えています。

```sh
$ ./toyevm disasm "6005600401"
PUSH 05
PUSH 04
ADD
```

## 📄 Source Code

[こちら](./code.md)にソースコードについての解説が載っています。

## 📚 Other

#### 初期ステートの変更

EVM の初期状態は、`config/config.json`に記述されています。

これを変更することで EVM の初期状態を変更することが可能です。

#### 未実装のオペコード

Toy なのでいくつか実装していないオペコードがあります。

また Gas の正確な実装もまだまだです。

今後実装予定です。
