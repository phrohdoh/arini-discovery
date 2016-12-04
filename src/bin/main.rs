extern crate arini_discovery;
use arini_discovery::ServiceType;

extern crate mdns;

use std::time::Duration;

fn main() {
    let bebop = ServiceType::Bebop2;
    let discover_service_name = bebop.get_wifi_service_type();
    let search_duration = Some(Duration::from_secs(5));

    println!("{}", discover_service_name);
    mdns::discover(discover_service_name, search_duration, |response| {
            println!("{:?}", response);
            let records = response.records()
                .filter_map(|record| {
                    if let mdns::RecordKind::A(addr) = record.kind {
                        Some(addr)
                    } else {
                        None
                    }
                })
                // Collect so we can get len,
                // otherwise this isn't necessary.
                .collect::<Vec<_>>();

            if records.len() > 0 {
                for record in records {
                    println!("Found {:#?}", record);
                }
            } else {
                println!("No addrs found!");
            }
        })
        .expect(&format!("Failed to discover {}", discover_service_name));
}