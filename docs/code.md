# 📄 Source Code

ここではToyEVMのソースコードについて解説します。

## 📟 `vm.rs`

EthereumにおけるEVMインスタンスを管理するモジュール

#### Environment

トランザクションの実行に必要な付随情報です。

```rs
pub struct Environment {
    code_owner: H160, // 実行するコントラクトのオーナー
    sender: H160, // トランザクションの送信者
    gas_price: usize, // gasのETHレート
    value: usize, // トランザクションに添付されたEth
    code: Vec<u8>, // 実行されるEVMバイトコード
    input: Vec<u8>, // トランザクションに渡されるデータ(solidityでは引数として渡される)
}
```

#### VM

EVMインスタンス本体です。

```rs
pub struct VM {
    env: Environment, // 環境変数
    pc: usize, // Program Counter
    gas: usize, // gas残量
    sp: usize, // スタックポインタ
    stack: Vec<U256>, // トランザクションのライフサイクルの間保持される一時的なスタック領域
    memory: Vec<u8>, // トランザクションのライフサイクルの間保持される一時的なメモリ領域
    asm: Vec<String>, // 実行した命令を入れておく 逆アセンブルに利用
}
```

#### 実行の流れ

基本的には普通のCPUやVM同様に

1. PCが示すバイトコードをfetch
2. 対応するオペコードをexec

を繰り返すのみです。

## 🌏 `state.rs`

Ethereumにおけるステートを表現するモジュール

#### World State

今回の実装では、アドレスとアカウントステートのマッピングおよび、ワールドステートを表すハッシュを含めています。

```rs
pub struct WorldState {
    addresses: HashMap<H160, AccountState>,
    hash: String,
}
```

#### Account State

今回の実装では、EOAとコントラクトアカウントで構造体を分けたりはしていません。

```rs
pub struct AccountState {
    nonce: usize, // ナンス
    balance: U256, // 残高(wei)
    storage: HashMap<U256, U256>, // storage
    code: String, // コントラクトコード
}
```
