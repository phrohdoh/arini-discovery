use std::fmt;

enum ServiceType {
    /// ._arsdk-0901._dup
    BebopDrone,

    /// ._arsdk-0902._dup
    JumpingSumo,

    /// ._arsdk-0903._udp
    SkyController,

    /// ._arsdk-0905._udp
    JumpingNight,

    /// ._arsdk-0906._udp
    JumpingRace,
}

impl ServiceType {
    pub fn get_wifi_service_type(&self) -> &'static str {
        match *self {
            ServiceType::BebopDrone => "._arsdk-0901._udp",
            ServiceType::JumpingSumo => "._arsdk-0902._udp",
            ServiceType::SkyController => "._arsdk-0903._udp",
            ServiceType::JumpingNight => "._arsdk-0904._udp",
            ServiceType::JumpingRace => "._arsdk-0905._udp",
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
                   ServiceType::JumpingSumo => "Jumping Sumo",
                   ServiceType::SkyController => "SkyController",
                   ServiceType::JumpingNight => "Jumping Night",
                   ServiceType::JumpingRace => "Jumping Race",
               })
    }
}

#[cfg(test)]
mod tests {
    use super::ServiceType;

    #[test]
    fn service_type_display() {
        assert_eq!(format!("{}", ServiceType::BebopDrone), "Bebop Drone");
        assert_eq!(format!("{}", ServiceType::JumpingSumo), "Jumping Sumo");
        assert_eq!(format!("{}", ServiceType::SkyController), "SkyController");
        assert_eq!(format!("{}", ServiceType::JumpingNight), "Jumping Night");
        assert_eq!(format!("{}", ServiceType::JumpingRace), "Jumping Race");
    }

    #[test]
    fn service_type_debug() {
        assert_eq!(format!("{:?}", ServiceType::BebopDrone),
                   "._arsdk-0901._udp");
        assert_eq!(format!("{:?}", ServiceType::JumpingSumo),
                   "._arsdk-0902._udp");
        assert_eq!(format!("{:?}", ServiceType::SkyController),
                   "._arsdk-0903._udp");
        assert_eq!(format!("{:?}", ServiceType::JumpingNight),
                   "._arsdk-0904._udp");
        assert_eq!(format!("{:?}", ServiceType::JumpingRace),
                   "._arsdk-0905._udp");
    }
}
