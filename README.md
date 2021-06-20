# TeaCuP ☕ 

A simple cross-platform `netcat`-like TCP/UDP listener and client.

## Server

```
Use as a server

USAGE:
    teacup listen [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
        --ipv6       Tell the server to use IPv6
    -V, --version    Prints version information

OPTIONS:
    -p, --port <port>            Port to listen on
        --protocol <protocol>    Protocol: TCP or UDP [default: tcp]
```

## Client

```
Use as a client

USAGE:
    teacup connect [OPTIONS] <addr>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --data <data>            Optionally send data on connection
    -p, --protocol <protocol>    Protocol: TCP or UDP [default: tcp]

ARGS:
    <addr>    Remote address to connect to
```

## Examples

### A simple echo server

```
USAGE:
    teacup example echo

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
```
