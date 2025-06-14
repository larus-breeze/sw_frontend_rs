use std::{
    fs, io::Write, path::PathBuf, str::FromStr, 
};
use serde::{Deserialize, Serialize};
use homedir::my_home;

use crate::{adapt, Error};

#[derive(PartialEq)]
pub enum ConfigItem {
    None,
    Adapt(adapt::Settings),
    Filter(adapt::FilterSettings),
}


#[derive(Deserialize, Serialize)]
#[serde(default)]
struct ConfigInner {
    adapt: adapt::Settings,
    filter: adapt::FilterSettings,
}

impl Default for ConfigInner {
    fn default() -> Self {
        ConfigInner { 
            adapt: adapt::Settings::default(),
            filter: adapt::FilterSettings::default(),
        }
    }
}

impl ConfigInner {
    fn new() -> ConfigInner {
        let path = match ConfigInner::get_config_dir() {
            Ok(path) => path,
            Err(_) => PathBuf::from_str("~~~").unwrap(),
        };
        let result = match fs::read_to_string(path) {
            Ok(s) => toml::from_str(&s).map_err(|_| Error::NoSettingsData),
            Err(_) => Err(Error::NoSettingsData),
        };
        match result {
            Ok(content) => content,
            Err(_) => {
                eprintln!("Could not read settings data using default");
                ConfigInner::default()
            }
        }
    }

    fn set(&mut self, data: ConfigItem) {
        match data {
            ConfigItem::Adapt(settings) => self.adapt = settings,
            ConfigItem::Filter(settings) => self.filter = settings,
            ConfigItem::None => (), 
        }
    }

    fn get(&self, kind: &str) -> ConfigItem {
        match kind {
            "adapt" => ConfigItem::Adapt(self.adapt.clone()),
            "filter" => ConfigItem::Filter(self.filter.clone()),
            _ => ConfigItem::None,
        }
    }

    fn get_config_dir() -> Result<PathBuf, Error> {
        let opt_path = my_home().map_err(|_| Error::FileIo)?;
        let mut home_path = opt_path.ok_or(Error::FileIo)?;
        home_path.push(".config/simrc");
        Ok(home_path)
    }

    fn store(&self) {
        fn create(content: &str) -> Result<(), Error> {
            let path = ConfigInner::get_config_dir()?;
            let mut file = fs::File::create(path)?;
            file.write_all(content.as_bytes())?;
            Ok(())
        }

        match create(&toml::to_string(self).unwrap()) {
            Ok(_) => (),
            Err(_) => eprintln!("Could not write to '~/.config/simrc'"),
        }
    }
}


pub struct Config {
    inner: ConfigInner,
}

impl Config {
    pub fn new() -> Config {
        Config { inner: ConfigInner::new() }
    }

    pub fn set(&mut self, data: ConfigItem) {
        self.inner.set(data);
    }

    pub fn get(&self, kind: &str) -> ConfigItem {
        self.inner.get(kind)
    }
}

impl Drop for Config {
    fn drop(&mut self) {
        self.inner.store();
    }
}