
use serde::{Deserialize, Serialize};
use std:: {
    collections::BTreeMap,
    {fs, fs::File},
    io::{Result, Write},
};

use corelib::{
    Editable,
    menu::{MenuItemContent, Menu, SETTINGS_IDX, FLIGHT_MENU_IDX, full, club},
};

#[derive(Serialize, Deserialize)]
struct Dict(BTreeMap<String, u32>);

struct Io {
    dict: Dict,
    no: u32,
    json_file: &'static str,
    dest: File,
}

impl Io {
    fn new(json_file: &'static str, dest_file: &'static str) -> Self {
        let dict_str = match fs::read_to_string(json_file) {
            Result::Ok(s) => s,
            Result::Err(_) => String::from("{}"),
        };
        let dict: Dict = serde_json::from_str(&dict_str).unwrap();
        let dest = File::create(dest_file).unwrap();

        let mut no = 0;
        for val in dict.0.values() {
            if *val > no {
                no = *val;
            }
        }

        Self { dict, no, json_file, dest }
    }

    fn no(&mut self, s: &str) -> u32 {
        if self.dict.0.contains_key(s) {
            *self.dict.0.get(s).unwrap()
        } else {
            self.no += 1;
            self.dict.0.insert(String::from(s), self.no);
            self.no
        }

    }

    fn save_to_json(&self) {
        let s = serde_json::to_string_pretty(&self.dict).unwrap();
        let mut f = File::create(self.json_file).unwrap();
        let _ = write!(f, "{}", s);
        println!("File '{}' created", self.json_file);
    }

    fn write(&mut self, s: &String) {
        writeln!(&self.dest, "{}", s).unwrap();
    }
}



fn store_menu(
    header: &str,
    menu: &Menu,
    menu_list: &[Menu],
    json_file: &'static str, 
    dest_file: &'static str,
) {
    let mut level = 1;
    let mut no_item = 0;
    let mut io = Io::new(json_file, dest_file);

    let line = format!("{}\n=\n\n```\n{}", header, menu.name);
    io.write(&line);
    store_sub_menu(&mut no_item, &mut level, menu, menu_list, &mut io);
    let line = format!("```");
    io.write(&line);

    println!("File '{}' created", dest_file);
    io.save_to_json();

    fn store_sub_menu(
        no_item: &mut u32, 
        level: &mut u32,
        d_menu: &Menu, 
        menu_list: &[Menu],
        io: &mut Io) 
    {
        let mut line = String::new();
        for _ in 1..*level {
            line += "│   ";
        }

        for menu_entry in d_menu.items {
            match menu_entry.content {
                MenuItemContent::EditItem(item) => {
                    if item == Editable::Return {
                        let line = format!("{}└── {}", line, item.name());
                        io.write(&line);
                    } else {
                        let line = format!("{}├── {} ({})", line, item.name(), io.no(item.name()));
                        io.write(&line);
                        *no_item += 1;
                    }
                }
                MenuItemContent::MenuItem() => {
                    let sub_menu = menu_list[menu_entry.next_menu_idx];
                    let line = format!("{}├── {}", line, sub_menu.name);
                    io.write(&line);
                    *level += 1;
                    store_sub_menu(no_item, level, &sub_menu, menu_list, io);
                }
            }
        }
        let line = format!("{}", line);
        io.write(&line);
        *level -= 1;
    }
}

fn main() {
    store_menu(
        "Usage Mode Normal or Club",
        &full::MENU_LIST[FLIGHT_MENU_IDX],
        full::MENU_LIST,
        "../doc/menu.json", 
        "../doc/flight_menu.md"
    );
    store_menu(
        "Usage Mode Normal",
        &full::MENU_LIST[SETTINGS_IDX],
        full::MENU_LIST,
        "../doc/menu.json", 
        "../doc/full_settings_menu.md"
    );
    store_menu(
        "Usage Mode Club",
        &club::MENU_LIST[SETTINGS_IDX],
        club::MENU_LIST,
        "../doc/menu.json", 
        "../doc/club_settings_menu.md"
    );
}

    

