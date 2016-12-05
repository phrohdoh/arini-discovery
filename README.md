# `arini-discovery`

### A pure-rust implementation of the ARDiscovery protocol

## LICENSE

MIT

## Usage

```rust
extern crate arini_discovery;
use arini_discovery::ServiceType;

extern crate mdns;

use std::time::Duration;

fn main() {
    let bebop = ServiceType::Bebop2;
    let discover_service_name = bebop.get_wifi_service_type();
    let search_duration = Some(Duration::from_secs(5));
    mdns::discover(discover_service_name, search_duration, |response| {
        println!("{:?}", response);
    }).expect(&format!("Failed to discover {}", discover_service_name));
}
```

## Other

This library is _not currently functional_ but, hopefully with the help of
the Parrot developers, it will reach a useable state soon!