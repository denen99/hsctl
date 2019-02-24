extern crate serde_json;

use crate::device::*;

#[derive(Deserialize, Debug, Clone)]
pub struct DeviceControlJson {
    #[serde(rename = "Ref")]
    pub id: u32,
    #[serde(rename = "Label")]
    pub label: String,
    #[serde(rename = "ControlValue")]
    pub control_value: f32,
    #[serde(rename = "ControlType")]
    pub control_type: u32,
}

#[derive(Deserialize, Debug)]
pub struct DeviceControlPairJson {
    #[serde(rename = "ControlPairs")]
    control_pairs: Vec<DeviceControlJson>,
    #[serde(rename = "ref")]
    pub id: u32,
}

#[derive(Deserialize, Debug)]
pub struct DeviceControlResponse {
    #[serde(rename = "Devices")]
    pub devices: Vec<DeviceControlPairJson>,
}

impl DeviceControlResponse {
    pub fn find_by_id(&self, id: u32) -> Vec<DeviceControlJson> {
         self
            .devices
            .iter()
            .filter(|d| d.id == id)
            .map(|d| d.control_pairs.clone())
            .flatten()
            .collect::<Vec<DeviceControlJson>>()
    }
}

impl DeviceControlJson {
    pub fn get_control_type_value(&self) -> DeviceControlTypeValue {
        match self.control_type {
            1 => DeviceControlTypeValue::NotSpecified,
            2 => DeviceControlTypeValue::Values,
            3 => DeviceControlTypeValue::SingleTextFromList,
            4 => DeviceControlTypeValue::ListTextFromList,
            5 => DeviceControlTypeValue::Button,
            6 => DeviceControlTypeValue::ValuesRange,
            7 => DeviceControlTypeValue::ValuesRangeSlider,
            8 => DeviceControlTypeValue::TextList,
            9 => DeviceControlTypeValue::TextBoxNumber,
            10 => DeviceControlTypeValue::TextBoxString,
            11 => DeviceControlTypeValue::RadioOption,
            12 => DeviceControlTypeValue::ButtonScript,
            13 => DeviceControlTypeValue::ColorPicker,
            _ => DeviceControlTypeValue::Values,
        }
    }

    //    pub fn add_control_pairs_to_device(self, d: &mut Device) {
    //        let d_ctrl = self.to_device_control();
    //
    //        d.control_pairs.push(d_ctrl);
    //    }

    pub fn to_device_control(&self) -> DeviceControl {
        DeviceControl {
            label: self.label.clone(),
            control_value: self.control_value,
            control_type: self.get_control_type_value(),
        }
    }
}
