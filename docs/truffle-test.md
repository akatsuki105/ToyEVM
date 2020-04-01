# Truffle のテストコード

Truffle は Mocha と Chai をテストとアサーションに使うことで JavaScript テストを作成するための強固なフレームワークを提供します。

Truffle が Mocha を使ってどのようにテストを快適に行えるようにしているか見ていきましょう。

注意: Mocha に詳しくない方は、先に進む前に[Mocha のドキュメント](https://mochajs.org/)を読みましょう。([ここ](https://numb86-tech.hatenablog.com/entry/2016/06/08/155834)もオススメです)

## describe()ではなく contract()

テストの内容は Mocha のそれと対して変わりません。つまり`./test`ディレクトリに`.js`拡張子で Mocha が自動テストと認識する形になります。

Mocha との違いは`contract()`です。これは Truffle の[clean-room features](https://www.trufflesuite.com/docs/truffle/testing/testing-your-contracts#clean-room-environment)を実現するために用いられるという点を除いて、Mocha の`describe()`とまったく同じものです。

`contract()`の役割

- `contract()`が実行される前に、コントラクトは実行中の Ethereum クライアントに再デプロイされる。つまりテスト環境は毎回初期化されている。
- `contract()`はテストを書くのに使った Ethereum クライアントのアカウントを提供します。

Truffle のテストは Mocha のラッパーなので、`contract()`を使う必要がないなら`describe()`を使うことも可能です。

## 参考

[WRITING TESTS IN JAVASCRIPT](https://www.trufflesuite.com/docs/truffle/testing/writing-tests-in-javascript)
