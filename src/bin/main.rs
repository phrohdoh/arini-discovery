extern crate arini_discovery;

use arini_discovery::ServiceType;

fn main() {
    let bebop = ServiceType::BebopDrone;
    println!("The {} can be discovered with Service Type {}",
             bebop,
             bebop.get_wifi_service_type());
}