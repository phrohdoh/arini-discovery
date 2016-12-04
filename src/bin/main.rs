extern crate arini_discovery;
use arini_discovery::ServiceType;

extern crate mdns;

use std::time::Duration;

fn main() {
    let bebop = ServiceType::Bebop2;
    let discover_service_name = bebop.get_wifi_service_type();
    let search_duration = Some(Duration::from_secs(5));

    mdns::discover(discover_service_name, search_duration, |response| {
            let records = response.records().filter_map(|record| {
                if let mdns::RecordKind::A(_) = record.kind {
                    Some(record)
                } else {
                    None
                }
            });

            for record in records {
                println!("Found {:?} with addr {:?}", record.name, record.kind);
            }
        })
        .expect(&format!("Failed to discover {}", discover_service_name));
}