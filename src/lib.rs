// Copyright 2026 Jelly Terra <jellyterra@proton.me>
// Use of this source code form is governed under the MIT license.

flatlude::mods!();

use std::{collections::HashMap, error::Error, fs::read_to_string, path::Path};

pub struct McuProject {
    pub mcu: ioc::Mcu,
    pub map: HashMap<String, String>,

    pub cpn: String,
    pub family: String,
    pub name: String,

    pub io_pins: Vec<IoPin>,
}

pub struct IoPin {
    pub position: String,
    pub name: String,
    pub gpio_name: String,
    pub user_defined_name: Option<String>,

    pub gpio_cluster: char,

    pub signal: String,

    pub gpio: Gpio,
}

pub struct Gpio {
    pub mux_mode: GpioMux,
    pub pull_mode: GpioPullMode,
}

pub enum GpioMux {
    Analog,
    Input,
    Output(GpioDriveMode),
}

pub enum GpioDriveMode {
    PushPull,
    OpenDrain,
}

pub enum GpioPullMode {
    Floating,
    Up,
    Down,
}

pub fn parse_ioc(s: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut map = HashMap::new();

    for line in s.lines() {
        if line.starts_with('#') || line.is_empty() {
            continue;
        }

        let (k, v) = line.split_once('=').ok_or("missing =")?;
        map.insert(k.to_string(), v.to_string());
    }

    Ok(map)
}

pub fn parse_mcu_description(s: &str) -> Result<ioc::Mcu, Box<dyn Error>> {
    Ok(quick_xml::de::from_str(s)?)
}

pub fn parse_project(map: HashMap<String, String>, mcu: ioc::Mcu) -> Result<McuProject, Box<dyn Error>> {
    let cpn = map.get("Mcu.CPN").ok_or("missing MCU CPN")?.clone();
    let family = map.get("Mcu.Family").ok_or("missing MCU family")?.clone();
    let name = map.get("Mcu.UserName").ok_or("missing MCU name")?.clone();

    let mut io_pins = vec![];

    for pin in &mcu.pins {
        if pin.pin_type != "I/O" {
            continue;
        }

        let signal = match map.get(&format!("{}.Signal", pin.name)) {
            Some(v) => v.clone(),
            None => continue,
        };

        let mux_mode = match signal.as_str() {
            "GPIO_Input" => GpioMux::Input,
            "GPIO_Output" => match map.get(&format!("{}.GPIO_ModeDefaultOutputPP", pin.name)).ok_or("missing .GPIO_ModeDefaultOutputPP")?.as_str() {
                "GPIO_MODE_OUTPUT_PP" => GpioMux::Output(GpioDriveMode::PushPull),
                "GPIO_MODE_OUTPUT_OD" => GpioMux::Output(GpioDriveMode::OpenDrain),
                _ => continue,
            },
            _ => continue,
        };

        let pull_mode = match map.get(&format!("{}.GPIO_PuPd", pin.name)).ok_or("missing .GPIO_PuPd")?.as_str() {
            "GPIO_PULLUP" => GpioPullMode::Up,
            "GPIO_PULLDOWN" => GpioPullMode::Down,
            _ => GpioPullMode::Floating,
        };

        let gpio_name = pin.name.clone().split_once('-').unwrap_or((&pin.name, "")).0.to_string();

        let gpio_cluster = gpio_name.chars().nth(1).unwrap();

        let user_defined_name = match map.get(&format!("{}.GPIOLabel", gpio_name)) {
            Some(v) => Some(v.clone()),
            None => None,
        };

        io_pins.push(IoPin {
            position: pin.position.clone(),
            name: pin.name.clone(),
            user_defined_name,
            gpio_cluster,
            gpio_name,
            signal,

            gpio: Gpio { mux_mode, pull_mode },
        });
    }

    Ok(McuProject { map, mcu, cpn, family, name, io_pins })
}

pub fn ioc_project_from_file(ioc_path: &Path, mcu_db_dir: &Path) -> Result<McuProject, Box<dyn Error>> {
    let map = parse_ioc(read_to_string(ioc_path)?.as_str())?;
    let mcu_name = map.get("Mcu.Name").ok_or("missing Mcu.Name")?;
    let mcu = parse_mcu_description(read_to_string(mcu_db_dir.join(format!("{}.xml", mcu_name)))?.as_str())?;
    parse_project(map, mcu)
}
