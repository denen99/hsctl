#[derive(Serialize, Debug)]
pub struct Device {
    pub id: u32,
    pub name: String,
    pub location2: String,
    pub location: String,
    pub value: f32,
    pub status: String,
    pub device_api: DeviceApiValue,
    pub device_type: DeviceTypeValue,
    pub control_pairs: Vec<DeviceControl>,
}

impl Device {
    pub fn add_control_pairs(&mut self, control_pairs: Vec<DeviceControl>) {
        control_pairs.iter().for_each(|cp| {
            let c = cp.clone();
            self.control_pairs.push(c);
        });
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct DeviceControl {
    pub label: String,
    pub control_value: f32,
    pub control_type: DeviceControlTypeValue,
}

#[derive(Serialize, Clone, Debug)]
pub enum DeviceTypeValue {
    GenericRoot,
    Media(u32),
    Script(u32),
    Security(u32),
    SourceSwitch(u32),
    Thermostat(u32),
    Root,
    Unknown,
}

impl DeviceTypeValue {
    pub fn to_str(&self) -> &str {
        match self {
            DeviceTypeValue::GenericRoot => "Generic Root",
            DeviceTypeValue::Root => "Root",
            DeviceTypeValue::Media(x) => match x {
                1 => "Player Status",
                2 => "Player Status Additional",
                3 => "Player Control",
                4 => "Player Volume",
                5 => "Player Shuffle",
                6 => "Player Repeat",
                7 => "Music Genre",
                8 => "Music Album",
                9 => "Music Artist",
                10 => "Music Track",
                11 => "Music Playlist",
                12 => "Media Type",
                20 => "Music Selector Control",
                99 => "Root",
                _ => "Unknown Media Type",
            },
            DeviceTypeValue::SourceSwitch(x) => match x {
                0 => "Invalid",
                1 => "System",
                10 => "Source",
                15 => "Source Extended",
                20 => "Zone",
                25 => "Zone Extended",
                99 => "Root",
                _ => "Unknown Source Switch Type",
            },
            DeviceTypeValue::Thermostat(x) => match x {
                1 => "Operating State",
                2 => "Temperature",
                3 => "Mode_Set",
                4 => "Fan Mode Set",
                5 => "Fan Status",
                6 => "Setpoint",
                7 => "Runtime",
                8 => "Hold Mode",
                9 => "Operating Mode",
                10 => "Additional Temperature",
                11 => "Setback",
                12 => "Filter Remind",
                99 => "Root",
                _ => "Unknown Thermostat Type",
            },
            DeviceTypeValue::Script(x) => match x {
                0 => "Disabled",
                1 => "Run On Any Change",
                2 => "Run On Value Change",
                3 => "Run On String Change",
                _ => "Unknown Script Type",
            },
            DeviceTypeValue::Security(x) => match x {
                1 => "Alarm",
                10 => "Arming",
                20 => "Keypad",
                30 => "Zone Perimeter",
                31 => "Zone Perimeter Delay",
                32 => "Zone Interior",
                33 => "Zone Interior Delay",
                34 => "Zone Auxiliary",
                35 => "Zone Other",
                40 => "Zone Safety Smoke",
                41 => "Zone Safety CO",
                42 => "Zone Safety CO2",
                43 => "Zone Safety Other",
                50 => "Output Relay",
                51 => "Output Other",
                60 => "Communicator",
                70 => "Siren",
                99 => "Root",
                _ => "Unknown Security Type",
            },
            _ => "Unknown",
        }
    }
}

#[derive(Serialize, Debug)]
pub enum DeviceApiValue {
    NoAPI,
    Plugin,
    Thermostat,
    Media,
    Security,
    SourceSwitch,
    Script,
    UnKnown,
}

impl DeviceApiValue {
    pub fn to_str(&self) -> &str {
        match self {
            DeviceApiValue::NoAPI => "NO_API",
            DeviceApiValue::Plugin => "Plugin",
            DeviceApiValue::Thermostat => "Thermostat",
            DeviceApiValue::Media => "Media",
            DeviceApiValue::Security => "Security",
            DeviceApiValue::SourceSwitch => "SourceSwitch",
            DeviceApiValue::Script => "Script",
            _ => "Unknown",
        }
    }
}

#[derive(Serialize, Clone, Debug)]
pub enum DeviceControlTypeValue {
    NotSpecified,
    Values,
    SingleTextFromList,
    ListTextFromList,
    Button,
    ValuesRange,
    ValuesRangeSlider,
    TextList,
    TextBoxNumber,
    TextBoxString,
    RadioOption,
    ButtonScript,
    ColorPicker,
}

impl DeviceControlTypeValue {
    pub fn to_str(&self) -> &str {
        match self {
            DeviceControlTypeValue::NotSpecified => "Not Specified",
            DeviceControlTypeValue::Values => "Values",
            DeviceControlTypeValue::SingleTextFromList => "Single Text From List",
            DeviceControlTypeValue::ListTextFromList => "List Text From List",
            DeviceControlTypeValue::Button => "Button",
            DeviceControlTypeValue::ValuesRange => "Values Range",
            DeviceControlTypeValue::ValuesRangeSlider => "ValuesRangeSlider",
            DeviceControlTypeValue::TextList => "TextList",
            DeviceControlTypeValue::TextBoxNumber => "TextBoxNumber",
            DeviceControlTypeValue::TextBoxString => "TextBoxString",
            DeviceControlTypeValue::RadioOption => "RadioOption",
            DeviceControlTypeValue::ButtonScript => "ButtonScript",
            DeviceControlTypeValue::ColorPicker => "Color Picker",
        }
    }
}
