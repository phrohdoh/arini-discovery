use std::fmt;

pub enum ServiceType {
    /// _arsdk-0901
    Bebop,

    /// _arsdk-090c
    Bebop2,

    /// _arsdk-0902
    JumpingSumo,

    /// _arsdk-0903
    SkyController,

    /// _arsdk-0905
    JumpingNight,

    /// _arsdk-0906
    JumpingRace,
}

impl ServiceType {
    pub fn get_wifi_service_type(&self) -> &'static str {
        match *self {
            ServiceType::Bebop => "_arsdk-0901",
            ServiceType::Bebop2 => "_arsdk-090c",
            ServiceType::JumpingSumo => "_arsdk-0902",
            ServiceType::SkyController => "_arsdk-0903",
            ServiceType::JumpingNight => "_arsdk-0905",
            ServiceType::JumpingRace => "_arsdk-0906",
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
                   ServiceType::Bebop2 => "Bebop 2 Drone",
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
        assert_eq!(format!("{}", ServiceType::Bebop), "Bebop Drone");
        assert_eq!(format!("{}", ServiceType::Bebop2), "Bebop 2 Drone");
        assert_eq!(format!("{}", ServiceType::JumpingSumo), "Jumping Sumo");
        assert_eq!(format!("{}", ServiceType::SkyController), "SkyController");
        assert_eq!(format!("{}", ServiceType::JumpingNight), "Jumping Night");
        assert_eq!(format!("{}", ServiceType::JumpingRace), "Jumping Race");
    }

    #[test]
    fn service_type_debug() {
        assert_eq!(format!("{:?}", ServiceType::Bebop), "_arsdk-0901");
        assert_eq!(format!("{:?}", ServiceType::Bebop2), "_arsdk-090c");
        assert_eq!(format!("{:?}", ServiceType::JumpingSumo),
                   "_arsdk-0902");
        assert_eq!(format!("{:?}", ServiceType::SkyController),
                   "_arsdk-0903");
        assert_eq!(format!("{:?}", ServiceType::JumpingNight),
                   "_arsdk-0905");
        assert_eq!(format!("{:?}", ServiceType::JumpingRace),
                   "_arsdk-0906");
    }
}
