# プロセス間通信のサンプル

このプロジェクトは、Rustを使用したプロセス間通信のサンプルです。

## 使い方

### 1. メインのバイナリを実行する場合

```bash
# main.rsを実行する
cargo run --bin guess-the-number-game
```

### 2. パイプを使用したプロセス間通信

```bash
# writeバイナリの出力をreadバイナリの入力として渡す
cargo run --bin write | cargo run --bin read
```

### 3. 標準入力からのデータ受け渡し

```bash
# echoコマンドの出力をreadバイナリの入力として渡す
echo "Yakiniku" | cargo run --bin read
```

## 動作の説明

- `main.rs`: メインのバイナリ（guess-the-number-game）のエントリーポイント
- `write.rs`: 標準出力に"Sukiyaki"という文字列を出力します
- `read.rs`: 標準入力からデータを読み取り、それを処理します

これらのバイナリは、Linuxのパイプ機能を使用して連携することができます。パイプ（`|`）を使用することで、あるプロセスの出力を別のプロセスの入力として渡すことができます。
