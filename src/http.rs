extern crate dirs;
extern crate reqwest;

use base64::encode;
use reqwest::Client;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

use crate::api::device_control::DeviceControlResponse;
use crate::api::device_status;
use crate::device;
use crate::output;

const LOGIN_PATH: &str = "/JSON?request=hsversion";
const GET_STATUS_PATH: &str = "/JSON?request=getstatus";
const GET_CONTROL_PATH: &str = "/JSON?request=getcontrol";
const CACHED_TOKEN_FILE: &str = "encoded_pass";
const SET_STATUS_LABEL: &str = "/JSON?request=controldevicebylabel";
const SET_STATUS_VALUE: &str = "/JSON?request=controldevicebyvalue";

#[derive(Debug, Clone)]
pub struct APIRequest {
    auth_token: String,
    base_url: String,
    output_type: String,
}

impl APIRequest {
    // ----------------------------------
    // Public API
    // ----------------------------------
    pub fn new(url: String, output_type: &str) -> APIRequest {
        APIRequest {
            base_url: url,
            auth_token: get_cached_password(),
            output_type: String::from(output_type),
        }
    }

    pub fn login(&mut self, username: &str, password: &str) {
        let encoded = encode_credentials(username, password);
        self.auth_token = encoded.clone();
        let ro = self.clone();
        match APIRequest::make_request(&ro, LOGIN_PATH) {
            Ok(_) => {
                println!("Login successful");
                save_cached_password(encoded);
            }
            Err(e) => println!("Error logging in, {}", e),
        }
    }

    pub fn login_with_saved_token(&mut self) {
        let ro = self.clone();
        if self.auth_token == "" {
            println!("No auth token, returning...");
            return;
        }
        match APIRequest::make_request(&ro, LOGIN_PATH) {
            Ok(_) => {
                println!("Cached token validated successfully.");
            }
            Err(e) => println!("Error logging in, {}", e),
        }
    }

    // -------------------------------------
    // Get Device Status
    // -------------------------------------
    pub fn get_status(&self, ref_id: u32, location1: &str, location2: &str) {
        let mut params: String = String::from("&");

        // Filter by reference ID, otherwise ALL
        match ref_id {
            0 => params.push_str("ref=all"),
            x => params.push_str(&format!("ref={}", x.to_string())),
        }

        params.push_str(&format!("&location1={}&location2={}", location1, location2));

        let uri = GET_STATUS_PATH.to_string() + &params;

        match APIRequest::make_request(self, &uri) {
            Ok(response) => {
                //dbg!(&response);
                let j: device_status::DeviceResponse = serde_json::from_str(&response).unwrap();
                let mut devices: Vec<device::Device> =
                    j.devices.iter().map(|e| e.to_device()).collect();

                let k: DeviceControlResponse = self.get_control_status(ref_id);

                devices.iter_mut().for_each(|d: &mut device::Device| {
                    let cps = k.find_by_id(d.id);
                    if !cps.is_empty() {
                        let dcp = cps.iter().map(|x| x.to_device_control()).collect();
                        d.add_control_pairs(dcp);
                    }
                });

                output::print_status(&self.output_type, &devices);
            }
            _ => println!("Status request failed"),
        }
    }

    //-----------------------------------
    // Set Device Status
    //-----------------------------------
    pub fn set_status(&self, ref_id: u32, label: String, value: f32) {
        if value.abs() < 0 as f32 { //value == -1 as f32 {
            self.set_status_by_label(ref_id, label);
        } else {
            self.set_status_by_value(ref_id, value);
        }
    }

    fn set_status_by_label(&self, ref_id: u32, label: String) {
        let mut params: String = String::from("&");
        params.push_str(&format!("&ref={}&label={}", ref_id, label));

        let uri = SET_STATUS_LABEL.to_string() + &params;

        match APIRequest::make_request(self, &uri) {
            Ok(_response) => println!("Success setting status by label"),
            Err(x) => println!("Error setting status by label: {}", x),
        }
    }

    fn set_status_by_value(&self, ref_id: u32, value: f32) {
        let mut params: String = String::from("&");
        params.push_str(&format!("&ref={}&value={}", ref_id, value));

        let uri = SET_STATUS_VALUE.to_string() + &params;

        match APIRequest::make_request(self, &uri) {
            Ok(_response) => println!("Success setting status by value"),
            Err(x) => println!("Error setting status by value: {}", x),
        }
    }

    //-------------------------------------
    // Get Device Control
    //-------------------------------------
    pub fn get_control_status(&self, ref_id: u32) -> DeviceControlResponse {
        let mut params: String = String::from("&");

        match ref_id {
            0 => params.push_str("ref=all"),
            x => params.push_str(&format!("ref={}", x.to_string())),
        }

        let uri = GET_CONTROL_PATH.to_string() + &params;

        match APIRequest::make_request(&self, &uri) {
            Ok(response) => {
                let v: Result<DeviceControlResponse, serde_json::Error> =
                    serde_json::from_str(&response);

                match v {
                    Ok(j) => j,
                    Err(e) => panic!("Error parsing json: {}", e),
                }
            }
            _ => {
                println!("Control request failed");
                DeviceControlResponse {
                    devices: Vec::new(),
                }
            }
        }
    }

    // ----------------------------------------------
    //   PRIVATE APIS
    // ----------------------------------------------
    fn make_request(&self, path: &str) -> Result<String, String> {
        match APIRequest::make_http_request(self, &path) {
            Ok(mut response) => {
                if response.status().is_success() {
                    match response.text() {
                        Ok(s) => Ok(s),
                        Err(e) => Err(format!("Error in API Request, {}", e)),
                    }
                } else {
                    Err(String::from("Non-200 code returned"))
                }
            }
            Err(e) => Err(e),
        }
    }

    fn make_http_request(a: &APIRequest, path: &str) -> Result<reqwest::Response, String> {
        let auth = format!("Basic {}", a.auth_token);
        let url: String = (*a.base_url).to_string() + path;

        let resp = Client::new().get(&url).header("Authorization", auth).send();

        match resp {
            Ok(r) => Ok(r),
            Err(e) => Err(format!("{}", e)),
        }
    }
}

fn encode_credentials(username: &str, password: &str) -> String {
    let s = format!("{}:{}", username, password);
    encode(&s)
}

fn get_cached_password() -> String {
    let cached_token = format!("{}{}", get_cached_token_dir(), CACHED_TOKEN_FILE);
    let input = File::open(cached_token);
    match input {
        Ok(mut file) => {
            let mut contents = String::new();
            match file.read_to_string(&mut contents) {
                Ok(_) => contents,
                Err(e) => {
                    println!("Warning, unable to read credentials file {}", e);
                    String::from("")
                }
            }
        }
        Err(_) => {
            println!("Warning, no cached credentials found, make sure you login!");
            String::from("")
        }
    }
}

fn get_cached_token_dir() -> String {
    match dirs::home_dir() {
        Some(x) => {
            let p = Path::new(x.to_str().unwrap()).join(".hsctl/");
            p.to_str().unwrap().to_string()
        }
        _ => panic!("Error, unable to determine home directory"),
    }
}

fn maybe_create_dir() -> io::Result<()> {
    match fs::read_dir(get_cached_token_dir()) {
        Ok(_) => Ok(()),
        Err(_) => {
            println!(
                "Cached token directory does not exist, creating {}",
                get_cached_token_dir()
            );
            fs::create_dir(get_cached_token_dir())
        }
    }
}

fn save_cached_password(token: String) {
    if let Err(e) = maybe_create_dir() {
        println!("Unable to create temporary directory for token, {}", e);
        return;
    }

    let cached_token = format!("{}{}", get_cached_token_dir(), CACHED_TOKEN_FILE);

    match fs::write(cached_token, token) {
        Ok(_) => println!("Login token cached successfully.."),
        Err(e) => println!("Error saving cached token, {}", e),
    }
}
