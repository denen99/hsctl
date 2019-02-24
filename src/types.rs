// pub enum DeviceApi {
//    NoAPI,
//    Plugin,
//    Thermostat,
//    Media,
//    Security,
//    SourceSwitch,
//    Script,
//    UnKnown
// }
//
// impl DeviceApi {
//    pub fn to_string(&self) -> String {
//        match self {
//            DeviceApi::NoAPI => String::from("NoAPI"),
//            DeviceApi::Plugin => String::from("Plugin"),
//            DeviceApi::Thermostat => String::from("Thermostat"),
//            DeviceApi::Media => String::from("Media"),
//            DeviceApi::Security => String::from("Security"),
//            DeviceApi::SourceSwitch => String::from("SourceSwitch"),
//            DeviceApi::Script => String::from("Script"),
//            DeviceApi::UnKnown => String::from("UnKnown"),
//           // _ => String::from("UnKnown")
//        }
//    }
//
//    pub fn from_int(id: u8) -> DeviceApi {
//        match id {
//            0 => DeviceApi::NoAPI,
//            4 => DeviceApi::Plugin,
//            8 => DeviceApi::Security,
//            16 => DeviceApi::Thermostat,
//            32 => DeviceApi::Media,
//            64 => DeviceApi::SourceSwitch,
//            128 => DeviceApi::Script,
//            _ => DeviceApi::UnKnown
//        }
//    }
// }
//
// pub enum DeviceType {
//    GenericRoot,
//
// }
