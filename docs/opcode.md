# EVM Opcodes

※ このドキュメントは[Ethereum Virtual Machine Opcodes](https://ethervm.io/)を和訳したものです。

## 概要
EVMはスタックベース、ビッグエンディアン形式の1ワード256bitとするバーチャルマシンであり、Ethereumブロックチェーン上でスマートコントラクトを実行されるために作られたものです。

スマートコントラクトはトランザクションを受け取って計算やさらなるトランザクションを発行するためにEVMバイトコードを実行する以外は普通のアカウントと変わりません。

トランザクションは0バイト以上の、コントラクトやほかの情報とどのように作用するか決めたりするためのデータを持っています。

コントラクトの実行はバイトコードの先頭から開始されます。

バイトコードはオペランドを持つPUSH命令を除いて1バイトの固定長です。

ほかのスタックベースのVMと同様に、オペコードはスタックの先頭からオペランドをPOPして実行結果をスタックにPUSHします。

## コントラクトの作成

スマートコントラクトを作るのに必要なデータそれ自体も、コントラクトのコンストラクタを実行するEVMバイトコードです。

コンストラクタはコントラクトの状態の初期化を行い、最後にコントラクトのEVMバイトコードを返します。

コンストラクタはコントラクトのデプロイ時にのみ実行されコントラクトデプロイ後にはコントラクト上に残りません。

## コントラクトの実行

コントラクトはユーザーのためにインターフェースとなるABIを公開しています。

コントラクトを実行するために、ユーザーは任意のwei(Ethereumの最小単位)とABIに基づいたデータをもつトランザクションを発行します。トランザクションはコントラクトをどのように実行するかという情報に加えてほかの付随情報も持っています。

トランザクションの実行では主に以下の4つのデータ領域を操作します。

- Call Data
- Stack
- Memory
- Storage

### Call Data

この領域に関連するオペコード: CALLDATALOAD, CALLDATASIZE, CALLDATACOPY

トランザクションに付随されるデータ(分かりやすく言えば引数など)を格納したデータ領域です。

オペランドを含めると命令全体で4byteになることもあります。

### Stack

この領域に関連するオペコード: PUSH1, DUP1, SWAP1, POP

EVMは1層1層が256bitのスタックを実装しており、一般的なスタックベースのプログラミング言語同様に、ローカル変数、関数の引数、リターン先のアドレスを保持しています。

リターン先のアドレスとほかの変数を区別する方法は少々変わっていますがここでは割愛します。

### Memory

この領域に関連するオペコード: MLOAD, MSTORE, MSTORE8

メモリは1バイトの大きさの配列でスマートコントラクトが実行される間、データを保持する揮発性のデータ領域です。

### Storage

この領域に関連するオペコード: SLOAD, SSTORE

ストレージは、HashMap<U256, U256>型の不揮発性のデータ領域です。

コントラクトのインスタンス変数とハッシュマップはストレージに保存され、`web3.eth.getStorageAt(address, key)`でアクセスすることができます。

## Opcodes

|   opcode    |  Mnemonic   |     Gas     |  Stack Input  |  Stack Output  |  Expression  |   Notes   |
| ----------- | ----------- | ----------- |  -----------  |  -----------   | -----------  | --------- |
|  00         |  STOP       |  0          |               |                |  STOP()      |  halts execution of the contract      |
|  01         |  ADD        |  3          |  a\|b\|         |  a+b\|          |  a+b         |  (u)int256 addition modulo 2**256  |
|  02         |  MUL        |  5          |  a\|b\|         |  a\*b\|          |  a\*b         |  (u)int256 multiplication modulo 2**256  |
|  03         |  SUB        |  3          |  a\|b\|         |  a-b\|          |  a-b         |  (u)int256 subtraction modulo 2**256  |
|  04         |  DIV        |  5          |  a\|b\|         |  a//b\|          |  a//b         |  uint256 division  |
|  05         |  SDIV       |  5          |  a\|b\|         |  a//b\|          |  a//b         |  int256 division   |
|  06         |  MOD        |  5          |  a\|b\|         |  a%b\|          |  a%b         |  uint256 modulus   |
|  07         |  SMOD        |  5          |  a\|b\|         |  a%b\|          |  a%b         |  int256 modulus   |
|  08         |  ADDMOD        |  8          |  a\|b\|N\|         |  (a + b) % N\|          |  (a + b) % N         |  (u)int256 addition modulo N   |
|  09         |  MULMOD        |  8          |  a\|b\|N\|         |  (a * b) % N\|          |  (a * b) % N         |  (u)int256 multiplication modulo N   |
|  0a         |  EXP        |  10          |  a\|b\|        |  a\*\*b\|          |  a\*\*b         |  uint256 exponentiation modulo 2**256  |
|  0b         |  SIGNEXTEND        |  5          |  b\|x\|        |  y\|          |  y = SIGNEXTEND(x, b)         |  sign extends x from (b + 1) * 8 bits to 256 bits.  |
## 参考

- https://ethervm.io/