# benchmarking

```sh
$ cargo build --release
# term 1
$ cargo run --release
# term 2
$ wrk -c 1 -d 5 -t 1 --script wrk.bench.lua http://localhost:3030/
```