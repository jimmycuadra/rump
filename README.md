# Rump

**Rump** is a command line tool for quick storage and retrival of text snippets. It is a port of [Bang](https://github.com/jimmycuadra/bang)/[Gong](https://github.com/jimmycuadra/gong)/[Pork](https://github.com/jimmycuadra/pork) to Rust. Rump provides an executable, `rump`, as well as programmatic access to the text snippets via a Rust library.

## Synopsis

```
Usage: rump [OPTIONS] [KEY] [VALUE]

Options:
    -h --help           output usage information
    -v --version        output the version number
    -d --delete KEY     delete the specified key
```

## CLI usage

Get a key:

``` bash
rump my_key
```

Set a key:

```
rump my_key my_value
```

Delete a key:

```
rump -d my_key
```

## Library usage

Import the crate and command module:

``` rust
extern crate rump;
use rump::commands;
```

Get a key:

``` rust
let value: Option<String> = commands::get("my_key");
```

Set a key:

``` rust
commands::set("my_key", "my_value");
```

Delete a key:

``` rust
commands::delete("my_key");
```

## Storage

Data is serialized to JSON and stored in a file at `~/.rump`.

## License

[MIT](http://opensource.org/licenses/MIT)
