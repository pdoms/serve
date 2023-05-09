# Just a server that i use to serve build/dist folders.

In case you stumble upon this and want to use it:

Install [rust](https://www.rust-lang.org/tools/install).

Clone this repo.

Run: 
```console
$ cargo build --release
$ ./target/release/serve_app -p 3000 -h localhost -s dist
```
#Usage

Have a ``dist`` or ``build`` directory with an index.html ready.

|FLAG|EXPECTS|EXPLAINATION                                      |
|----|-------|--------------------------------------------------|
| -h | HOST  | define host to serve on                          |
| -p | PORT  | define port via whicht to serve                  |
| -s | PORT  | define the source directory from which to serve  |





