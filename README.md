# Rump

**Rump** is a command line tool for quick storage and retrival of text snippets. It is a port of [Bang](https://github.com/jimmycuadra/bang)/[Gong](https://github.com/jimmycuadra/gong) to Rust.

## Synopsis

```
Usage: rump [OPTIONS] [KEY] [VALUE]

Options:
    -h --help           output usage information
    -v --version        output the version number
    -d --delete KEY     delete the specified key
```

## Usage

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

## Storage

Data is serialized to JSON and stored in a file at `~/.rump`.

## License

[MIT](http://opensource.org/licenses/MIT)
