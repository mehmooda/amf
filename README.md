# AMF

A crate that will parse the action message format (AMF0) and possibly AMF3 in the future.

## Features

- Parse AMF Packets
- Serde serialization and deserialization

## Example

- Raw Parsing

```rust
use amf::AMFData;

let bytes: &[u8] = include_bytes!(...)
let data = AMFData::new(bytes);

for x in data {
    // Handle each amf::AMF object
    dbg!(x);
}
```

## License

GPLv3 -- if you need another license let me know