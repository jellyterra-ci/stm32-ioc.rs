// Copyright 2026 Jelly Terra <jellyterra@proton.me>

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

#[derive(Debug)]
pub struct IoPin {
    pub position: String,
    pub gpio_name: String,
    pub user_defined_name: Option<String>,
    pub signal: Option<String>,
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

        let (gpio_name, user_defined_name) = match pin.name.split_once('-') {
            Some(first) => (first.0.to_string(), Some(first.1.to_string())),
            None => (pin.name.clone(), None),
        };

        let signal = match map.get(&format!("{}.Signal", pin.name)) {
            Some(v) => Some(v.clone()),
            None => None,
        };

        io_pins.push(IoPin {
            position: pin.position.clone(),
            user_defined_name,
            gpio_name,
            signal,
        });
    }

    Ok(McuProject { map, mcu, cpn, family, name, io_pins })
}

pub fn ioc_project_from_file(ioc_path: &Path, mcu_db_dir: &Path) -> Result<McuProject, Box<dyn Error>> {
    let map = parse_ioc(read_to_string(ioc_path)?.as_str())?;
    let mcu_name = map.get("Mcu.Name").ok_or("missing MCU name")?;
    let mcu = parse_mcu_description(read_to_string(mcu_db_dir.join(format!("{}.xml", mcu_name)))?.as_str())?;
    parse_project(map, mcu)
}
