# Optimization Repro.

Run with `cargo +nightly bench`:

```
$ cargo +nightly bench
       Fresh odds v0.2.24
       Fresh nodrop v0.1.8
       Fresh arrayvec v0.3.20
   Compiling optim-repro v0.1.0 (file:///Users/daschl/code/rust/optim-repro)
     Running `rustc benches/allinone.rs --crate-name allinone -C opt-level=3 --test -C metadata=c16420d13a87d70e -C extra-filename=-c16420d13a87d70e --out-dir /Users/daschl/code/rust/optim-repro/target/release --emit=dep-info,link -L dependency=/Users/daschl/code/rust/optim-repro/target/release/deps --extern arrayvec=/Users/daschl/code/rust/optim-repro/target/release/deps/libarrayvec-caf4a6c5f0300bbd.rlib --extern optim_repro=/Users/daschl/code/rust/optim-repro/target/release/deps/liboptim_repro.rlib`
    Finished release [optimized] target(s) in 0.36 secs
     Running `/Users/daschl/code/rust/optim-repro/target/release/allinone-c16420d13a87d70e --bench`

running 1 test
test bench_allinone ... bench:       3,765 ns/iter (+/- 576)

test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured

     Running `/Users/daschl/code/rust/optim-repro/target/release/imported-3b98bc0d36d61cde --bench`

running 1 test
test bench_imported ... bench:       6,686 ns/iter (+/- 3,608)

test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured

     Running `/Users/daschl/code/rust/optim-repro/target/release/deps/optim_repro-cdc56146a007087a --bench`

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```

See how the time on the bench differs but the code is exactly the same.
