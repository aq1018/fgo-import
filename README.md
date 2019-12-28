# fgo-import

Imports fgo.json to a postgres db.

## Build

```
cargo build --release
```

## Usage

```
fgo-import 0.1
Aaron Q.
Load FGO data from a JSON file and export to PostgreSQL database.

USAGE:
    fgo-import --dburl <URL> --input <FILE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --dburl <URL>     specify postgresql database URL
    -i, --input <FILE>    specify JSON file that contains FGO data.
```

Exmaple:

```
./target/release/fgo-import -d postgres://user:password@localhost:5432/mydb -i ./data/fgo.json
```