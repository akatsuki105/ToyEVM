# 📄 Source Code

ここでは ToyEVM のソースコードについて解説します。

## 📟 `vm.rs`

Ethereum における EVM インスタンスを管理するモジュール

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

EVM インスタンス本体です。

```rs
pub struct VM {
    env: Environment, // 環境変数
    pc: usize, // Program Counter
    gas: usize, // gas残量
    sp: usize, // スタックポインタ
    stack: Vec<U256>, // トランザクションのライフサイクルの間保持される一時的なスタック領域
    memory: Vec<u8>, // トランザクションのライフサイクルの間保持される一時的なメモリ領域
    asm: Vec<String>, // 実行した命令を入れておく 逆アセンブルに利用
    returns: Vec<u8>, // アクションの返り値
}
```

#### 実行の流れ

基本的には普通の CPU や VM 同様に

1. PC が示すバイトコードを fetch
2. 対応するオペコードを exec

を繰り返すのみです。

## 🌏 `state.rs`

Ethereum におけるステートを表現するモジュール

#### World State

今回の実装では、アドレスとアカウントステートのマッピングおよび、ワールドステートを表すハッシュを含めています。

```rs
pub struct WorldState {
    addresses: HashMap<H160, AccountState>,
    hash: String,
}
```

#### Account State

今回の実装では、EOA とコントラクトアカウントで構造体を分けたりはしていません。

```rs
pub struct AccountState {
    nonce: usize, // ナンス
    balance: U256, // 残高(wei)
    storage: HashMap<U256, U256>, // storage
    code: String, // コントラクトコード
}
```
