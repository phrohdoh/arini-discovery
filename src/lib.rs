use std::fmt;

enum ServiceType {
    BebopDrone,
}

impl ServiceType {
    pub fn get_wifi_service_type(&self) -> &'static str {
        match *self {
            ServiceType::BebopDrone => "._arsdk-0901._udp",
        }
    }
}

impl fmt::Debug for ServiceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", (*self).get_wifi_service_type())
    }
}

impl fmt::Display for ServiceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}",
               match *self {
                   ServiceType::BebopDrone => "Bebop Drone",
               })
    }
}

#[cfg(test)]
mod tests {
    use super::ServiceType;

    #[test]
    fn service_type_display() {
        assert_eq!(format!("{}", ServiceType::BebopDrone), "Bebop Drone");
    }

    #[test]
    fn service_type_debug() {
        assert_eq!(format!("{:?}", ServiceType::BebopDrone), "._arsdk-0901._udp");
    }
}
