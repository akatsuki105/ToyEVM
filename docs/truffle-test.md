# WRITING TESTS IN JAVASCRIPT

**これは[WRITING TESTS IN JAVASCRIPT](https://www.trufflesuite.com/docs/truffle/testing/writing-tests-in-javascript)を和訳したものです。**

Truffle は Mocha と Chai をテストとアサーションに使うことで JavaScript テストを作成するための強固なフレームワークを提供します。

Truffle が Mocha を使ってどのようにテストを快適に行えるようにしているか見ていきましょう。

注意: Mocha に詳しくない方は、先に進む前に[Mocha のドキュメント](https://mochajs.org/)を読みましょう。([ここ](https://numb86-tech.hatenablog.com/entry/2016/06/08/155834)もオススメです)

## describe()ではなく contract()

テストの内容は Mocha のそれと対して変わりません。つまり`./test`ディレクトリに`.js`拡張子で Mocha が自動テストと認識する形になります。

Mocha との違いは`contract()`です。これは Truffle の[clean-room features](https://www.trufflesuite.com/docs/truffle/testing/testing-your-contracts#clean-room-environment)を実現するために用いられるという点を除いて、Mocha の`describe()`とまったく同じものです。

`contract()`の役割

- `contract()`が実行される前に、コントラクトは実行中の Ethereum クライアントに再デプロイされる。つまりテスト環境は毎回初期化されている。
- `contract()`はテストを書くのに使った Ethereum クライアントのアカウントのリストを提供します。

Truffle のテストは Mocha のラッパーなので、`contract()`を使う必要がないなら`describe()`を使うことも可能です。

## contract abstractions を使おう

Contract abstractions は Javascript を使ってコントラクトと交信するための基礎です。

Truffle はあなたがテスト内で交信したいコントラクトがどのコントラクトかわからないので、どのコントラクトがそうであるかを明示的にする必要があります。

そのためには`artifacts.require()`を使います。Truffle によって提供されるこのメソッドのおかげで Solidity コントラクトを特定するために利用可能な contract abstraction をリクエストできます。

下の例を見ると、コントラクトが正しく機能することを確認するためにこの abstraction を使っていることがわかります。

contract abstractions の利用についてより情報が欲しい場合は、『the Interacting With Your Contracts』セクションを見てください。

## artifacts.require()の使用について

`artifacts.require()`のテスト内での使用は、migration でそれを使用するのと同じように機能します。つまりコントラクトの名前を渡してやる必要があります。 詳細な使用方法が知りたいなら Migration 内の`artifacts.require()`のドキュメントを見てください。

## Web3 の使用について

正しく設定されていれば各テストファイルで Web3 のインスタンスが利用可能です。つまり`web3.eth.getBalance`が利用可能です。

## 例

### `.then`を使う場合

これは the MetaCoin Truffle Box 内で利用されるテストの例です。

`contract()`を使うことで利用可能な Ethereum アカウントを特定するためのアカウントのリストが手に入り、`artifacts.require()`を使用することでコントラクトと直接交信が可能になります。

```js
// ./test/metacoin.js
const MetaCoin = artifacts.require("MetaCoin"); // これでMetaCoinコントラクトとの交信が可能に

contract("MetaCoin", (accounts) => {
  // accounts: 利用可能なEthereumアカウントのリスト
  it("should put 10000 MetaCoin in the first account", () =>
    MetaCoin.deployed()
      .then((instance) => instance.getBalance.call(accounts[0]))
      .then((balance) => {
        assert.equal(
          balance.valueOf(),
          10000,
          "10000 wasn't in the first account"
        );
      }));

  it("should call a function that depends on a linked library", () => {
    let meta;
    let metaCoinBalance;
    let metaCoinEthBalance;

    return MetaCoin.deployed()
      .then((instance) => {
        meta = instance;
        return meta.getBalance.call(accounts[0]);
      })
      .then((outCoinBalance) => {
        metaCoinBalance = outCoinBalance.toNumber();
        return meta.getBalanceInEth.call(accounts[0]);
      })
      .then((outCoinBalanceEth) => {
        metaCoinEthBalance = outCoinBalanceEth.toNumber();
      })
      .then(() => {
        assert.equal(
          metaCoinEthBalance,
          2 * metaCoinBalance,
          "Library function returned unexpected function, linkage may be broken"
        );
      });
  });

  it("should send coin correctly", () => {
    let meta;

    // Get initial balances of first and second account.
    const account_one = accounts[0];
    const account_two = accounts[1];

    let account_one_starting_balance;
    let account_two_starting_balance;
    let account_one_ending_balance;
    let account_two_ending_balance;

    const amount = 10;

    return MetaCoin.deployed()
      .then((instance) => {
        meta = instance;
        return meta.getBalance.call(account_one);
      })
      .then((balance) => {
        account_one_starting_balance = balance.toNumber();
        return meta.getBalance.call(account_two);
      })
      .then((balance) => {
        account_two_starting_balance = balance.toNumber();
        return meta.sendCoin(account_two, amount, { from: account_one });
      })
      .then(() => meta.getBalance.call(account_one))
      .then((balance) => {
        account_one_ending_balance = balance.toNumber();
        return meta.getBalance.call(account_two);
      })
      .then((balance) => {
        account_two_ending_balance = balance.toNumber();

        assert.equal(
          account_one_ending_balance,
          account_one_starting_balance - amount,
          "Amount wasn't correctly taken from the sender"
        );
        assert.equal(
          account_two_ending_balance,
          account_two_starting_balance + amount,
          "Amount wasn't correctly sent to the receiver"
        );
      });
  });
});
```

このテストは以下のような結果になります。

```
  Contract: MetaCoin
    √ should put 10000 MetaCoin in the first account (83ms)
    √ should call a function that depends on a linked library (43ms)
    √ should send coin correctly (122ms)


  3 passing (293ms)
```

### async/await を使う場合

```js
const MetaCoin = artifacts.require("MetaCoin");

contract("2nd MetaCoin test", async (accounts) => {
  it("should put 10000 MetaCoin in the first account", async () => {
    let instance = await MetaCoin.deployed();
    let balance = await instance.getBalance.call(accounts[0]);
    assert.equal(balance.valueOf(), 10000);
  });

  it("should call a function that depends on a linked library", async () => {
    let meta = await MetaCoin.deployed();
    let outCoinBalance = await meta.getBalance.call(accounts[0]);
    let metaCoinBalance = outCoinBalance.toNumber();
    let outCoinBalanceEth = await meta.getBalanceInEth.call(accounts[0]);
    let metaCoinEthBalance = outCoinBalanceEth.toNumber();
    assert.equal(metaCoinEthBalance, 2 * metaCoinBalance);
  });

  it("should send coin correctly", async () => {
    // Get initial balances of first and second account.
    let account_one = accounts[0];
    let account_two = accounts[1];

    let amount = 10;

    let instance = await MetaCoin.deployed();
    let meta = instance;

    let balance = await meta.getBalance.call(account_one);
    let account_one_starting_balance = balance.toNumber();

    balance = await meta.getBalance.call(account_two);
    let account_two_starting_balance = balance.toNumber();
    await meta.sendCoin(account_two, amount, { from: account_one });

    balance = await meta.getBalance.call(account_one);
    let account_one_ending_balance = balance.toNumber();

    balance = await meta.getBalance.call(account_two);
    let account_two_ending_balance = balance.toNumber();

    assert.equal(
      account_one_ending_balance,
      account_one_starting_balance - amount,
      "Amount wasn't correctly taken from the sender"
    );
    assert.equal(
      account_two_ending_balance,
      account_two_starting_balance + amount,
      "Amount wasn't correctly sent to the receiver"
    );
  });
});
```

結果は上と同じになります。

## テストの指定

以下のようにファイルをしてすることで特定のテストのみ実行できます。

```sh
truffle test ./test/metacoin.js
```

もっと知りたい場合は[command reference](https://www.trufflesuite.com/docs/truffle/reference/truffle-commands#test)を見てください。

## もっと先へ

Truffle は Mocha の設定ファイルも提供しているため、Mocha の挙動も設定可能です。もっと知りたい場合は『the project configuration』セクションを見てください。

## TypeScript のサポート

Truffle は TypeScript によるテストもサポートしています。もっと知りたい場合は『the Writing Tests in JavaScript guide』を見てください。

## 参考

[WRITING TESTS IN JAVASCRIPT](https://www.trufflesuite.com/docs/truffle/testing/writing-tests-in-javascript)
