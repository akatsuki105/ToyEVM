# 🚗 ToyEVM

簡易的なEVM(Ethereum Virtual Machine)を実装することを目標としたRust製のレポジトリです。

## ⛓ Ethereum Virtual Machine

EVMについての説明は[こちら](./guide.md)にまとめてあります。

## 🎮 Usage

### 💡 起動

```sh
make build   # require make
./toyevm run # toyevm.exe on Windows
```

#### トランザクションの実行

#### コントラクトのデプロイ

### 📚 その他

#### 初期ステートの変更

EVMの初期状態は、`config/config.json`に記述されています。

これを変更することでEVMの初期状態を変更することが可能です。

#### 逆アセンブル

EVMバイトコードを逆アセンブルする機能も備えています。

```sh
./toyevm disasm "6005600401"
```

Result

```
PUSH
PUSH
ADD
```