# 設計

## 目標

<img src="https://i.stack.imgur.com/afWDt.jpg" />

この右半分を作る。  
具体的には、
 - EVMバイトコードを実行できるインタプリタ
 - EVMバイトコード実行の結果変化していくEVMのstate

の2つを作る  
ネットワーク部分やpow部分は作らない

## モジュール構成
- インタプリタ interpreter
    - parser バイトコードをパースする
    - evaluator パースしたバイトコードを実行し必要に応じてstateを変更する
- ステート state  
ステートを表す構造体を持ち、ステート取得や変更のAPIを提供する