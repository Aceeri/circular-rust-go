# circular-rust-go

FFI between rust and go.

## Build

```
go install -buildmode=shared -linkshared std
go install -buildmode=shared -linkshared dhl
cargo rustc --lib -- --crate-type=cdylib
go build -buildmode=c-shared dhl
cargo build --features link
```
