This is an example reproduction for an issue in rust-protobuf fixed by https://github.com/stepancheg/rust-protobuf/pull/725.

After a `cargo run`, you should see:

```
assertion `left == right` failed: Expected to write 10003, actually wrote 3
  left: 10005
 right: 5
```
