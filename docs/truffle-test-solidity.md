# WRITING TESTS IN SOLIDITY

**これは[WRITING TESTS IN SOLIDITY](https://www.trufflesuite.com/docs/truffle/testing/writing-tests-in-solidity)を和訳したものです。**

Solidity によるテストコントラクトは .sol ファイルとして Javascript テストと並んで動作します。`truffle test`を実行すると、テストコントラクトはそれぞれ個別のテストスイートに含まれます。

これらのコントラクトは、Javascript テストが持っているメリットと同様のメリットを持っています。  
つまりテストスイートごとに隔離された環境、デプロイされたコントラクトへのダイレクトなアクセス、コントラクトの依存関係をインポートする機能をちゃんとそなえています。これらの機能に加えて、Truffle の Solidity テストフレームワークは、以下の問題を念頭に置いて開発されています。

- Solidity のテストは、(テストコントラクトのような)どのコントラクトからも拡張されるべきではありません。これにより、テストする範囲を可能な限り小さくし、書くコントラクト内容をしっかり把握できるようになります。
- Solidity のテストは、アサーション・ライブラリに依存すべきではありません。Truffle はデフォルトのアサーション・ライブラリを提供していますが、ニーズに合わせていつでもこのライブラリを変更できます。
- Solidity のテストを任意の Ethereum クライアントで実行できるようにする必要があります。

## 例

それでは、本題に入る前に、`truffle unbox metacoin`にある Solidity のテストコントラクトのサンプルを見てみましょう。

```js
pragma solidity >=0.4.25 <0.6.0;

import "truffle/Assert.sol";
import "truffle/DeployedAddresses.sol";
import "../contracts/MetaCoin.sol";

contract TestMetaCoin {
  function testInitialBalanceUsingDeployedContract() {
    MetaCoin meta = MetaCoin(DeployedAddresses.MetaCoin());

    uint expected = 10000;

    Assert.equal(meta.getBalance(tx.origin), expected, "Owner should have 10000 MetaCoin initially");
  }

  function testInitialBalanceWithNewMetaCoin() {
    MetaCoin meta = new MetaCoin();

    uint expected = 10000;

    Assert.equal(meta.getBalance(tx.origin), expected, "Owner should have 10000 MetaCoin initially");
  }
}
```

実行すると次のような結果になります。

```sh
$ truffle test

Compiling your contracts...
===========================
> Compiling ./contracts/ConvertLib.sol
> Compiling ./contracts/MetaCoin.sol
> Compiling ./contracts/Migrations.sol
> Compiling ./test/TestMetaCoin.sol



  TestMetaCoin
    ✓ testInitialBalanceUsingDeployedContract (79ms)
    ✓ testInitialBalanceWithNewMetaCoin (65ms)

  Contract: MetaCoin
    ✓ should put 10000 MetaCoin in the first account (38ms)
    ✓ should call a function that depends on a linked library (42ms)
    ✓ should send coin correctly (120ms)


  5 passing (7s)
```

結果を見ると、2 つのファイル(1 つは Javascript、もう 1 つは Solidity)から出力が得られていることがわかります。

このドキュメントでは、ここで取り上げている Solidity のテストについてだけ説明していきます。

## Test structure

何が起きているかより理解したいなら、詳細に議論をしましょう。

## Assertions

`Assert.equal()`のようなアサーション関数は、`truffle/Assert.sol`ライブラリによって提供されます。

これはデフォルトのアサーションライブラリであり、もし Truffle のテストランナーと結合可能なアサーションのためのイベントを正しく呼び出せるライブラリがあればそれを使うことも可能です。

利用可能なアサーション関数を知りたい場合は、`Assert.sol`を見てください。

## Deployed addresses

デプロイされたコントラクトのアドレスは`truffle/DeployedAddresses.sol`ライブラリを通してアクセスできます。

これは Truffle によって提供されていて、各テストスイートが各々の環境でテストを実行する前に、再コンパイルと再リンクされます。

このライブラリは以下の形式でデプロイされたコントラクトのすべての関数を提供してくれます。

```js
DeployedAddresses.<contract name>();
```

上であげたテストコードからもわかるように、このコードからアドレスが得られるのでコントラクトへのアクセスが可能になります。

デプロイされたコントラクトを使うためには、コントラクトをテストスイート内にインポートする必要があります。

## テストコントラクトの命名規則

テストコントラクトの名前は`Test`で始まっている必要がある。

## テスト関数の名前

テスト関数の名前は`test`で始まっている必要がある。

## before / after hooks

下の例のように、たくさんの hooks がテストでは利用可能です。

これらの hook は Mocha で提供される hooks と同様に`beforeAll`, `beforeEach`, `afterAll`, `afterEach`の 4 つです。(Mocha の hooks については[こちら](https://note.kiriukun.com/entry/testing-with-mocha-and-chai--basic-2))

```js
import "truffle/Assert.sol";

contract TestHooks {
  uint someValue;

  function beforeEach() {
    someValue = 5;
  }

  function beforeEachAgain() {
    someValue += 1;
  }

  function testSomeValueIsSix() {
    uint expected = 6;

    Assert.equal(someValue, expected, "someValue should have been 6");
  }
}
```
