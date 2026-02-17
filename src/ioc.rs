// Copyright 2026 Jelly Terra <jellyterra@proton.me>
// Use of this source code form is governed under the MIT license.

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Mcu {
    #[serde(rename = "@ClockTree")]
    pub clock_tree: String,
    #[serde(rename = "@DBVersion")]
    pub db_version: String,
    #[serde(rename = "@Family")]
    pub family: String,
    #[serde(rename = "@HasPowerPad")]
    pub has_power_pad: bool,
    #[serde(rename = "@IOType")]
    pub io_type: String,
    #[serde(rename = "@Line")]
    pub line: String,
    #[serde(rename = "@Package")]
    pub package: String,
    #[serde(rename = "@RefName")]
    pub ref_name: String,

    #[serde(rename = "IP")]
    pub ips: Vec<Ip>,
    #[serde(rename = "Pin")]
    pub pins: Vec<Pin>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Voltage {
    #[serde(rename = "@Max")]
    pub max: f32,
    #[serde(rename = "@Min")]
    pub min: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Temperature {
    #[serde(rename = "@Max")]
    pub max: i32,
    #[serde(rename = "@Min")]
    pub min: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ip {
    #[serde(rename = "@InstanceName")]
    pub instance_name: String,
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Version")]
    pub version: String,
    #[serde(rename = "@ConfigFile")]
    pub config_file: Option<String>,
    #[serde(rename = "@ClockEnableMode")]
    pub clock_enable_mode: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pin {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Position")]
    pub position: String,
    #[serde(rename = "@Type")]
    pub pin_type: String,
    #[serde(rename = "@Variant")]
    pub variant: Option<String>,

    #[serde(rename = "Signal", default)]
    pub signals: Vec<Signal>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Signal {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@IOModes")]
    pub io_modes: Option<String>,
}
