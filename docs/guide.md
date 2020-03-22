# ⛓ Ethereum Virtual Machine について

EVM はスタックベース、ビッグエンディアン形式の 1 ワード 256bit とするバーチャルマシンであり、Ethereum ブロックチェーン上でスマートコントラクトを実行されるために作られたものです。

256bit スタックベースの VM なので uint256 といった一般的な高級言語では定義されていない整数型をよく利用します。
浮動小数点型はサポートされていないため、1ETH は EVM の内部では 1000000000wei という単位で取り扱われます。1wei が ETH の最小単位です。

スマートコントラクトはトランザクションを受け取って計算やさらなるトランザクションを発行するために EVM バイトコードを実行する以外は普通のアカウントと変わりません。

トランザクションは 0 バイト以上の、コントラクトやほかの情報とどのように作用するか決めたりするためのデータを持っています。

コントラクトの実行はバイトコードの先頭から開始されます。

バイトコードはオペランドを持つ PUSH 命令を除いて 1 バイトの固定長です。

ほかのスタックベースの VM と同様に、オペコードはスタックの先頭からオペランドを POP して実行結果をスタックに PUSH します。

## 🔍 Index

- [Ethereum のステートと EVM](#EthereumのステートとEVM)
- [コントラクトのデプロイ](#コントラクトのデプロイ)
- [コントラクトの実行](#コントラクトの実行)
- [Gas](#Gas)
- [Opcode 一覧](#Opcode一覧)
- [その他のリソース](#その他)

## ⛓ Ethereum のステートと EVM

Ethereum のステートは World state と Account State のおおまかに 2 つに分けられます。

EVM はトランザクションの実行を通してこのステートを変更する役割を持ちます。

これらの状態は ToyEVM においては state.rs で実装されています。

#### World State

Ethereum アドレスから Account State へのマッピングです。

#### Account State

Ethereum の残高、nonce、storage(後述するデータ領域)、コントラクトコードから構成されます。

nonce は、EOA ではそのアカウントが実行したトランザクションの総数、コントラクトアカウントでは作成されたコントラクトの数を表します。

#### ステートへの反映

トランザクション実行時には、EVM のインスタンスが作成されその中でコントラクトの EVM バイトコードを実行します。

DB のトランザクションと同様に、Ethereum へのステートの反映は EVM インスタンスでトランザクションを完遂して初めて行われます。

途中で gas 不足などによりトランザクションが中断した場合は Ethereum へのステート反映は行われることはありません。

## 📦 コントラクトのデプロイ

新しいコントラクトを Ethereum 上にデプロイするには、デプロイのための特別なトランザクションが必要になってきます。

デプロイ用のトランザクションでは、トランザクションの to フィールドをアドレス 0x0 に設定し、データフィールドをデプロイ用の EVM バイトコードにする必要があります。

このバイトコードはデプロイする EVM バイトコードとは異なったものとなっています。

例えば、`6005600401`という EVM バイトコードをデプロイするとするとします。

これは逆アセンブルすると以下のようなコードになります。

```
PUSH1  05   // 6005
PUSH1  04   // 6004
ADD         // 01
```

このバイトコードをデプロイする際に実行されるバイトコードは以下のように`600580600b6000396000f36005600401`なり元のバイトコードとは異なったものになっています。

```
// デプロイ用のバイトコード
PUSH1  05   // 6005
DUP1        // 80
PUSH1  0b   // 600b
PUSH1  00   // 6000
CODECOPY    // 39
PUSH1  00   // 6000
RETURN      // f3

// 本体のコード
PUSH1  05   // 6005
PUSH1  04   // 6004
ADD         // 01
```

このデプロイ用のコードでは、最初にデプロイされるバイトコードの長さをスタックに push しています。(PUSH1 05)

そして CODECOPY で 0b-0b+05 つまりデプロイされるバイトコードをメモリの先頭にコピーし、それを return で返しているため、デプロイを行うコード(`600580600b6000396000f36005600401`)の返り値はデプロイされるコード(`6005600401`)となります。

この返り値がコントラクトのコードとして Ethereum 上に登録されます。

コントラクトのアドレスは、トランザクションの sender のアドレスと nonce から決定的に生成されます。

## 🎮 コントラクトの実行

コントラクトはユーザーのためにインターフェースとなる ABI を公開しています。

コントラクトを実行するために、ユーザーは任意の wei(Ethereum の最小単位)と ABI に基づいたデータをもつトランザクションを発行します。トランザクションはコントラクトをどのように実行するかという情報に加えてほかの付随情報も持っています。

トランザクションの実行では主に以下の 4 つのデータ領域を操作します。

- Call Data
- Stack
- Memory
- Storage

### Call Data

この領域に関連するオペコード: CALLDATALOAD, CALLDATASIZE, CALLDATACOPY

トランザクションに付随されるデータ(分かりやすく言えば引数など)を格納したデータ領域です。

スタック上のオペランドを含めると命令全体で 4byte になることもあります。

### Stack

この領域に関連するオペコード: PUSH1, DUP1, SWAP1, POP

EVM は 1 層 1 層が 256bit のスタックを実装しており、一般的なスタックベースのプログラミング言語同様に、ローカル変数、関数の引数、リターン先のアドレスを保持しています。

リターン先のアドレスとほかの変数を区別する方法は少々変わっていますがここでは割愛します。

### Memory

この領域に関連するオペコード: MLOAD, MSTORE, MSTORE8

メモリは 1 バイトの大きさの配列でスマートコントラクトが実行される間、データを保持する揮発性のデータ領域です。

### Storage

この領域に関連するオペコード: SLOAD, SSTORE

ストレージは、HashMap<U256, U256>型の不揮発性のデータ領域です。

コントラクトのインスタンス変数とハッシュマップはストレージに保存され、`web3.eth.getStorageAt(address, key)`でアクセスすることができます。

### その他の命令

LOG 命令はブロック内にログを記録することで軽量クライアントによる検証を効率的に行えるようにします。

CALL, CREATE 命令はコントラクトにほかのコントラクトのアクションを引き起こしたり、新しいコントラクトを作らせる命令です。

RETURN 命令は、メモリからデータのまとまりを返り値として返す命令であり、SUICIDE はコントラクトを消去して、たまっていた資産を特定のアドレスに返す役割を持った命令です。

## ☁️ Gas

[Gas](./gas.md)を参照

## 📑 Opcode 一覧

|    Value     |   Mnemonic   |                         Gas Used                         |    Subset    | Removed from stack | Added to stack |                                             Notes                                              |                                                                          Formula Notes                                                                          |
| :----------: | :----------: | :------------------------------------------------------: | :----------: | :----------------: | :------------: | :--------------------------------------------------------------------------------------------: | :-------------------------------------------------------------------------------------------------------------------------------------------------------------: |
|     0x00     |     STOP     |                            0                             |     zero     |         0          |       0        |                                        Halts execution.                                        |                                                                                                                                                                 |
|     0x01     |     ADD      |                            3                             |   verylow    |         2          |       1        |                                       Addition operation                                       |                                                                                                                                                                 |
|     0x02     |     MUL      |                            5                             |     low      |         2          |       1        |                                   Multiplication operation.                                    |                                                                                                                                                                 |
|     0x03     |     SUB      |                            3                             |   verylow    |         2          |       1        |                                     Subtraction operation.                                     |                                                                                                                                                                 |
|     0x04     |     DIV      |                            5                             |     low      |         2          |       1        |                                  Integer division operation.                                   |                                                                                                                                                                 |
|     0x05     |     SDIV     |                            5                             |     low      |         2          |       1        |                         Signed integer division operation (truncated).                         |                                                                                                                                                                 |
|     0x06     |     MOD      |                            5                             |     low      |         2          |       1        |                                   Modulo remainder operation                                   |                                                                                                                                                                 |
|     0x07     |     SMOD     |                            5                             |     low      |         2          |       1        |                               Signed modulo remainder operation.                               |                                                                                                                                                                 |
|     0x08     |    ADDMOD    |                            8                             |     mid      |         3          |       1        |                                   Modulo addition operation.                                   |                                                                                                                                                                 |
|     0x09     |    MULMOD    |                            8                             |     mid      |         3          |       1        |                                Modulo multiplication operation.                                |                                                                                                                                                                 |
|     0x0a     |     EXP      |     (exp == 0) ? 10 : (10 + 10 \* (1 + log256(exp)))     |              |         2          |       1        |                                     Exponential operation.                                     |                                                                        "If exponent is 0                                                                        | gas used is 10. If exponent is greater than 0 | gas used is 10 plus 10 times a factor related to how large the log of the exponent is." |
|     0x0b     |  SIGNEXTEND  |                            5                             |     low      |         2          |       1        |                       Extend length of two’s complement signed integer.                        |                                                                                                                                                                 |
|     0x10     |      LT      |                            3                             |   verylow    |         2          |       1        |                                     Less-than comparison.                                      |                                                                                                                                                                 |
|     0x11     |      GT      |                            3                             |   verylow    |         2          |       1        |                                    Greater-than comparison.                                    |                                                                                                                                                                 |
|     0x12     |     SLT      |                            3                             |   verylow    |         2          |       1        |                                  Signed less-than comparison.                                  |                                                                                                                                                                 |
|     0x13     |     SGT      |                            3                             |   verylow    |         2          |       1        |                                Signed greater-than comparison.                                 |                                                                                                                                                                 |
|     0x14     |      EQ      |                            3                             |   verylow    |         2          |       1        |                                      Equality comparison.                                      |                                                                                                                                                                 |
|     0x15     |    ISZERO    |                            3                             |   verylow    |         1          |       1        |                                      Simple not operator.                                      |                                                                                                                                                                 |
|     0x16     |     AND      |                            3                             |   verylow    |         2          |       1        |                                     Bitwise AND operation.                                     |                                                                                                                                                                 |
|     0x17     |      OR      |                            3                             |   verylow    |         2          |       1        |                                      Bitwise OR operation                                      |                                                                                                                                                                 |
|     0x18     |     XOR      |                            3                             |   verylow    |         2          |       1        |                                     Bitwise XOR operation.                                     |                                                                                                                                                                 |
|     0x19     |     NOT      |                            3                             |   verylow    |         1          |       1        |                                     Bitwise NOT operation.                                     |                                                                                                                                                                 |
|     0x1a     |     BYTE     |                            3                             |   verylow    |         2          |       1        |                                 Retrieve single byte from word                                 |                                                                                                                                                                 |
|     0x20     |     SHA3     |            30 + 6 \* (size of input in words)            |              |         2          |       1        |                                    Compute Keccak-256 hash.                                    |                                   30 is the paid for the operation plus 6 paid for each word (rounded up) for the input data.                                   |
|     0x30     |   ADDRESS    |                            2                             |     base     |         0          |       1        |                          Get address of currently executing account.                           |                                                                                                                                                                 |
|     0x31     |   BALANCE    |                           400                            |              |         1          |       1        |                               Get balance of the given account.                                |                                                                                                                                                                 |
|     0x32     |    ORIGIN    |                            2                             |     base     |         0          |       1        |                               Get execution origination address.                               |                                                                                                                                                                 |
|     0x33     |    CALLER    |                            2                             |     base     |         0          |       1        |                                      Get caller address.                                       |                                                                                                                                                                 |
|     0x34     |  CALLVALUE   |                            2                             |     base     |         0          |       1        |       Get deposited value by the instruction/transaction responsible for this execution.       |                                                                                                                                                                 |
|     0x35     | CALLDATALOAD |                            3                             |   verylow    |         1          |       1        |                             Get input data of current environment.                             |                                                                                                                                                                 |
|     0x36     | CALLDATASIZE |                            2                             |     base     |         0          |       1        |                         Get size of input data in current environment.                         |                                                                                                                                                                 |
|     0x37     | CALLDATACOPY |            "2 + 3 \* (number of words copied             | rounded up)" |                    |       3        |                                               0                                                |                                                        Copy input data in current environment to memory.                                                        | 2 is paid for the operation plus 3 for each word copied (rounded up). |
|     0x38     |   CODESIZE   |                            2                             |     base     |         0          |       1        |                        Get size of code running in current environment.                        |                                                                                                                                                                 |
|     0x39     |   CODECOPY   |            "2 + 3 \* (number of words copied             | rounded up)" |                    |       3        |                                               0                                                |                                                       Copy code running in current environment to memory.                                                       | 2 is paid for the operation plus 3 for each word copied (rounded up). |
|     0x3a     |   GASPRICE   |                            2                             |     base     |         0          |       1        |                            Get price of gas in current environment.                            |                                                                                                                                                                 |
|     0x3b     | EXTCODESIZE  |                           700                            |   extcode    |         1          |       1        |                                 Get size of an account’s code.                                 |                                                                                                                                                                 |
|     0x3c     | EXTCODECOPY  |           "700 + 3 \* (number of words copied            | rounded up)" |                    |       4        |                                               0                                                |                                                                Copy an account’s code to memory.                                                                | 700 is paid for the operation plus 3 for each word copied (rounded up). |
|     0x40     |  BLOCKHASH   |                            20                            |              |         1          |       1        |                  Get the hash of one of the 256 most recent complete blocks.                   |                                                                                                                                                                 |
|     0x41     |   COINBASE   |                            2                             |     base     |         0          |       1        |                              Get the block’s beneficiary address.                              |                                                                                                                                                                 |
|     0x42     |  TIMESTAMP   |                            2                             |     base     |         0          |       1        |                                   Get the block’s timestamp.                                   |                                                                                                                                                                 |
|     0x43     |    NUMBER    |                            2                             |     base     |         0          |       1        |                                    Get the block’s number.                                     |                                                                                                                                                                 |
|     0x44     |  DIFFICULTY  |                            2                             |     base     |         0          |       1        |                                  Get the block’s difficulty.                                   |                                                                                                                                                                 |
|     0x45     |   GASLIMIT   |                            2                             |     base     |         0          |       1        |                                   Get the block’s gas limit.                                   |                                                                                                                                                                 |
|     0x50     |     POP      |                            2                             |     base     |         1          |       0        |                                    Remove item from stack.                                     |                                                                                                                                                                 |
|     0x51     |    MLOAD     |                            3                             |   verylow    |         1          |       1        |                                     Load word from memory.                                     |                                                                                                                                                                 |
|     0x52     |    MSTORE    |                            3                             |   verylow    |         2          |       0        |                                      Save word to memory                                       |                                                                                                                                                                 |
|     0x53     |   MSTORE8    |                            3                             |   verylow    |         2          |       0        |                                      Save byte to memory.                                      |                                                                                                                                                                 |
|     0x54     |    SLOAD     |                           200                            |              |         1          |       1        |                                     Load word from storage                                     |                                                                                                                                                                 |
|     0x55     |    SSTORE    | ((value != 0) && (storage_location == 0)) ? 20000 : 5000 |              |         1          |       1        |                                     Save word to storage.                                      |       20000 is paid when storage value is set to non-zero from zero. 5000 is paid when the storage value's zeroness remains unchanged or is set to zero.        |
|     0x56     |     JUMP     |                            8                             |     mid      |         1          |       0        |                                   Alter the program counter                                    |                                                                                                                                                                 |
|     0x57     |    JUMPI     |                            10                            |     high     |         2          |       0        |                            Conditionally alter the program counter.                            |                                                                                                                                                                 |
|     0x58     |      PC      |                            2                             |     base     |         0          |       1        | Get the value of the program counter prior to the increment corresponding to this instruction. |                                                                                                                                                                 |
|     0x59     |    MSIZE     |                            2                             |     base     |         0          |       1        |                            Get the size of active memory in bytes.                             |                                                                                                                                                                 |
|     0x5a     |     GAS      |                            2                             |     base     |         0          |       1        |                                "Get the amount of available gas                                |                                            including the corresponding reduction for the cost of this instruction."                                             |  |
|     0x5b     |   JUMPDEST   |                            1                             |              |         0          |       0        |                               Mark a valid destination for jumps                               |                                                                                                                                                                 |
| 0x60 -- 0x7f |    PUSH\*    |                            3                             |   verylow    |         0          |       1        |                            Place _ byte item on stack. 0 < _ <= 32                             |                                                                                                                                                                 |
| 0x80 -- 0x8f |    DUP\*     |                            3                             |   verylow    |         \*         |     \* + 1     |                             Duplicate _th stack item. 0 < _ <= 16                              |                                                                                                                                                                 |
| 0x90 -- 0x9f |    SWAP\*    |                            3                             |   verylow    |       \* + 1       |     \* + 1     |                            Exchange 1st and (\* + 1)th stack items.                            |                                                                                                                                                                 |
|     0xa0     |     LOG0     |         375 + 8 \* (number of bytes in log data)         |              |         2          |       0        |                               Append log record with no topics.                                |                                              375 is paid for operation plus 8 for each byte in data to be logged.                                               |
|     0xa1     |     LOG1     |      375 + 8 \* (number of bytes in log data) + 375      |              |         3          |       0        |                               Append log record with one topic.                                |                           375 is paid for operation plus 8 for each byte in data to be logged plus 375 for the 1 topic to be logged.                            |
|     0xa2     |     LOG2     |    375 + 8 _ (number of bytes in log data) + 2 _ 375     |              |         4          |       0        |                               Append log record with two topics.                               |                        375 is paid for operation plus 8 for each byte in data to be logged plus 2 \* 375 for the 2 topics to be logged.                         |
|     0xa3     |     LOG3     |    375 + 8 _ (number of bytes in log data) + 3 _ 375     |              |         5          |       0        |                              Append log record with three topics.                              |                        375 is paid for operation plus 8 for each byte in data to be logged plus 3 \* 375 for the 3 topics to be logged.                         |
|     0xa4     |     LOG4     |    375 + 8 _ (number of bytes in log data) + 4 _ 375     |              |         6          |       0        |                              Append log record with four topics.                               |                        375 is paid for operation plus 8 for each byte in data to be logged plus 4 \* 375 for the 4 topics to be logged.                         |
|     0xf0     |    CREATE    |                          32000                           |              |         3          |       1        |                           Create a new account with associated code.                           |                                                                                                                                                                 |
|     0xf1     |     CALL     |          Complex -- see yellow paper Appendix H          |              |         7          |       1        |                                 Message-call into an account.                                  |                                                                                                                                                                 |
|     0xf2     |   CALLCODE   |          Complex -- see yellow paper Appendix H          |              |         7          |       1        |               Message-call into this account with an alternative account’s code.               |                                                                                                                                                                 |
|     0xf3     |    RETURN    |                            0                             |     zero     |         2          |       0        |                             Halt execution returning output data.                              |                                                                                                                                                                 |
|     0xf4     | DELEGATECALL |          Complex -- see yellow paper Appendix H          |              |         6          |       1        |               "Message-call into this account with an alternative account’s code               |                                                    but persisting the current values for sender and value."                                                     |  |
|     0xfe     |   INVALID    |                            NA                            |              |         NA         |       NA       |                                Designated invalid instruction.                                 |                                                                                                                                                                 |
|     0xff     | SELFDESTRUCT |        5000 + ((create_new_account) ? 25000 : 0)         |              |         1          |       0        |                     Halt execution and register account for later deletion                     | 5000 for the operation plus 25000 if a new account is also created. A refund of 24000 gas is also added to the refund counter for self-destructing the account. |

※ [opcode-gas-costs_EIP-150_revision-1e18248_2017-04-12.csv](https://github.com/djrtwo/evm-opcode-gas-costs/blob/master/opcode-gas-costs_EIP-150_revision-1e18248_2017-04-12.csv)から移植したものです

## その他

- [EIP について](./eips/eips.md)

## 📚 参考

- [Mastering Ethereum](https://www.oreilly.co.jp/books/9784873118963/)
- [Ethereum Virtual Machine Opcodes](https://ethervm.io/)
- [djrtwo/evm-opcode-gas-costs](https://github.com/djrtwo/evm-opcode-gas-costs/blob/master/opcode-gas-costs_EIP-150_revision-1e18248_2017-04-12.csv)
- [CoinCulture/evm-tools](https://github.com/CoinCulture/evm-tools/blob/master/analysis/guide.md)
- [Ethereum の Contract Creation の仕組み](https://y-nakajo.hatenablog.com/entry/2018/05/17/100752)
