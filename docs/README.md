# ToyEVM

簡易的なEVM(Ethereum Virtual Machine)を実装することを目標としたRust製のレポジトリです。

## Ethereum Virtual Machine

EVMについての説明は[こちら](./guide.md)にまとめてあります。

## Usage

#### 起動

```sh
make build # require make
./toyevm # toyevm.exe on Windows
```

#### 初期ステートの変更

EVMの初期状態は、`config/config.json`に記述されています。

#### トランザクションの実行

#### コントラクトのデプロイ
