# Iron Talk Demo

This is the demo code from Rust Melbourne's second event on 2016-09-20.

http://www.meetup.com/Rust-Melbourne/events/233345740/

I've only had this successfully running against `rustc` version `nightly-2016-08-18`. It's a little bit hard to find a working version that works with recent version of both `serde` and `diesel` at the moment.

To run this...

```
./make_database.sh
rustup override set nightly-2016-08-18
cargo run
```
