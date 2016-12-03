use std::fmt;

enum ServiceType {
    Bebop,
}

impl ServiceType {
    pub fn get_wifi_service_type(&self) -> &'static str {
        match *self {
            ServiceType::Bebop => "._arsdk-0901._udp",
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
                   ServiceType::Bebop => "Bebop Drone",
               })
    }
}

#[cfg(test)]
mod tests {
    use super::ServiceType;

    #[test]
    fn service_type_display() {
        assert_eq!(format!("{}", ServiceType::Bebop), "Bebop Drone");
    }

    #[test]
    fn service_type_debug() {
        assert_eq!(format!("{:?}", ServiceType::Bebop), "._arsdk-0901._udp");
    }
}
