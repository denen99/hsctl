extern crate serde_json;

use crate::device::*;
// HomeSeer JSON is an absolute cluster.  Bizarre naming conventions
// as well as inconsistent casing.  So we need to do a lot of mapping
// here to our snake case structs

#[derive(Deserialize, Debug)]
pub struct DeviceJson {
    #[serde(rename = "ref")]
    pub id: u32,
    pub name: String,
    pub location: String,
    pub location2: String,
    pub value: f32,
    pub status: String,
    pub device_type_string: String,
    #[serde(rename = "device_type")]
    pub device_type_info: DeviceTypeInfo,
}

#[derive(Deserialize, Debug)]
pub struct DeviceResponse {
    #[serde(rename = "Devices")]
    pub devices: Vec<DeviceJson>,
}

#[derive(Deserialize, Debug)]
pub struct DeviceTypeInfo {
    #[serde(rename = "Device_API")]
    pub device_api: u32,
    #[serde(rename = "Device_API_Description")]
    pub device_api_description: String,
    #[serde(rename = "Device_Type")]
    pub device_type: u32,
    #[serde(rename = "Device_Type_Description")]
    pub device_type_description: String,
    #[serde(rename = "Device_SubType")]
    pub device_sub_type: u32,
    #[serde(rename = "Device_SubType_Description")]
    pub device_sub_type_description: String,
}

impl DeviceJson {
    pub fn get_device_api_value(&self) -> DeviceApiValue {
        match self.device_type_info.device_api {
            0 => DeviceApiValue::NoAPI,
            4 => DeviceApiValue::Plugin,
            16 => DeviceApiValue::Thermostat,
            32 => DeviceApiValue::Media,
            8 => DeviceApiValue::Security,
            64 => DeviceApiValue::SourceSwitch,
            128 => DeviceApiValue::Script,
            _ => DeviceApiValue::UnKnown,
        }
    }

    pub fn get_device_type_value(&self) -> DeviceTypeValue {
        if self.device_type_info.device_type == 999 {
            return DeviceTypeValue::GenericRoot;
        }

        match self.get_device_api_value() {
            DeviceApiValue::NoAPI => match self.device_type_info.device_type {
                999 => DeviceTypeValue::GenericRoot,
                _ => DeviceTypeValue::Unknown,
            },
            DeviceApiValue::Plugin => match self.device_type_info.device_type {
                999 => DeviceTypeValue::GenericRoot,
                99 => DeviceTypeValue::Root,
                _ => DeviceTypeValue::Unknown,
            },
            DeviceApiValue::Media => DeviceTypeValue::Media(self.device_type_info.device_type),
            DeviceApiValue::Thermostat => {
                DeviceTypeValue::Thermostat(self.device_type_info.device_type)
            }
            DeviceApiValue::Script => DeviceTypeValue::Script(self.device_type_info.device_type),
            DeviceApiValue::Security => {
                DeviceTypeValue::Security(self.device_type_info.device_type)
            }
            DeviceApiValue::SourceSwitch => {
                DeviceTypeValue::SourceSwitch(self.device_type_info.device_type)
            }
            _ => DeviceTypeValue::GenericRoot,
        }
    }

    pub fn to_device(&self) -> Device {
        Device {
            device_api: self.get_device_api_value(),
            device_type: self.get_device_type_value(),
            id: self.id,
            location: self.location.clone(),
            location2: self.location2.clone(),
            name: self.name.clone(),
            status: self.status.clone(),
            value: self.value,
            control_pairs: Vec::new(),
        }
    }
}
