extern crate arini_discovery;
use arini_discovery::ServiceType;

extern crate mdns;

use std::time::Duration;

fn main() {
    let bebop = ServiceType::Bebop2;
    let discover_service_name = bebop.get_wifi_service_type();
    println!("The {} can be discovered with Service Type {}",
             bebop,
             discover_service_name);

    let search_duration = Duration::from_secs(5);
    mdns::discover(discover_service_name, Some(search_duration), |response| {
            let addresses = response.records().filter_map(|record| {
                if let mdns::RecordKind::A(addr) = record.kind {
                    Some(addr)
                } else {
                    None
                }
            });

            for addr in addresses {
                println!("Found {} on {}", bebop, addr);
            }
        })
        .expect(&format!("Failed to discover {}", discover_service_name));
}