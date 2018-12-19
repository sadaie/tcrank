# `tcrank`
`tcrank` is a tiny utility tool for THE IDOLM@STER MILLION LIVE!! THEATER DAYS and its special election event, THE@TER CHALLENGE!!.  
`tcrank` uses the [Princess](https://api.matsurihi.me/docs/).

## Install

**`tcrank` is written in Rust. Thus you should install the latest Rust ecosystem in advance.**  
**refs. [rustup](https://rustup.rs/)**

### With `cargo install`

```
$ cargo install -f tcrank
```

### Build from source code

```
$ git clone https://github.com/sadaie/tcrank
$ cd tcrank
$ cargo build --release
$ ls target/release/
build       deps        examples    incremental native      tcrank      tcrank.d
```

## Usage

### Listing the idols and/or the roles.

```
# lists both of the idols and roles.
$ tcrank list

# lists the idols.
$ tcrank list -i

# lists the roles.
$ tcrank list -r
```

### Showing the specified idol's rank(s).

```
# shows the idol's rank by ID.
$ tcrank show -i 21
Name        Role            Score  Rank
徳川まつり  少女            80     9
徳川まつり  魔法使い        10857  1
徳川まつり  ファイナルデイ  36     7

# shows the idol's rank by ID and role's ID.
$ tcrank show -i 21 -r 23
Name        Role            Score  Rank
徳川まつり  魔法使い        10857  1

# and you can use both of the idol's name and role's name.
$ tcrank show -i "徳川まつり" -r "魔法使い"
Name        Role            Score  Rank
徳川まつり  魔法使い        10857  1
```

#### Additional options

- `--json` option prints the result as `JSON` style string.
- `--json-pretty` option prints the result as pretty `JSON` style string.

## License

MIT lincense.  