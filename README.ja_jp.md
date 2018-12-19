# `tcrank`
`tcrank`はアイドルマスターミリオンライブ シアターデイズとゲーム内投票イベント`THE@TER CHALLENGE!!`用のツールです  
`tcrank`は[Princess](https://api.matsurihi.me/docs/)を使用しています

## インストール

**`tcrank`は`Rust`製のCLIアプリケーションのため、事前に最新のRustツールチェーンをインストールしてください**  
**refs. [rustup](https://rustup.rs/)**

### `cargo install`でインストール

```
$ cargo install -f tcrank
```

### ソースコードからビルド

```
$ git clone https://github.com/sadaie/tcrank
$ cd tcrank
$ cargo build --release
$ ls target/release/
build       deps        examples    incremental native      tcrank      tcrank.d
```

## 使い方

### アイドルと役の一覧を表示する

```
# アイドルと役の両方の一覧を表示する
$ tcrank list

# アイドルの一覧を表示する
$ tcrank list -i

# 役の一覧を表示する
$ tcrank list -r
```

### 指定したアイドルの順位を表示する

```
# IDを指定してアイドルの順位を表示する
$ tcrank show -i 21
Name        Role            Score  Rank
徳川まつり  少女            80     9
徳川まつり  魔法使い        10857  1
徳川まつり  ファイナルデイ  36     7

# アイドルと役の両方のIDを指定して順位を表示する
$ tcrank show -i 21 -r 23
Name        Role            Score  Rank
徳川まつり  魔法使い        10857  1

# IDではなく名前での指定も可能です
$ tcrank show -i "徳川まつり" -r "魔法使い"
Name        Role            Score  Rank
徳川まつり  魔法使い        10857  1
```

#### 追加オプション

- `--json`オプションは結果を`JSON`形式で出力します
- `--json-pretty`オプションは結果を整形された`JSON`形式で出力します

## License

MIT lincense.  