# `arini-discovery`

### A pure-rust implementation of the ARDiscovery protocol

## LICENSE

MIT

## Usage

```rust,no_run
extern crate arini_discovery;
extern crate mdns;

use arini_discovery::ServiceType;
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

## Planned Usage

```rust,ignore
extern crate arini_discovery;
use arini_discovery::ServiceType;
use std::time::Duration;

fn main() {
    let search_duration = Some(Duration::from_secs(5));
    let discoveries = arini_discovery::discover(ServiceType::Bebop2, search_duration);

    for discovery in discoveries {
        println!("{:#?}", discovery);
    }
}
```

## Supporting this project

If you would like to financially support this project please do the following:
* [Become a patron](https://www.patreon.com/Phrohdoh)
* [Tip me on gratipay](https://gratipay.com/~Phrohdoh/)
* [E-mail me](mailto:taryn@phrohdoh.com) for one-time donation information

## Other

This library is _not currently functional_ but, hopefully with the help of
the Parrot developers, it will reach a useable state soon!