
use std:: {
    {fs, fs::File},
    io::{Result, Write},
};

use corelib::{
    Editable,
    menu::{MenuItemContent, Menu, SETTINGS_IDX, FLIGHT_MENU_IDX, full},
};

use phf::phf_map;

static REPLMNTS: phf::Map<&'static str, &'static str> = phf_map! {
    "Flight Menu" => "\\nameref{flight_menu}",
    "Settings" => "\\nameref{settings}",
    "Views" => "\\nameref{views}",
    "Advanced" => "\\nameref{advanced}",
    "Polar Settings" => "\\nameref{polar-settings}",
    "Sensorbox" => "\\nameref{sensor-box}",
};


struct Menus {
    template: String,
}

impl Menus {
    fn new() -> Self {
        let template = fs::read_to_string("menus_template.tex").unwrap();
        Self { template }
    }

    fn add(&mut self, replace: &str, menu: &Menu, menu_list: &[Menu]) {
        let mut level = 1;
        let mut r_str = String::new();

        let m_name = if REPLMNTS.contains_key(menu.name) {
            REPLMNTS.get(menu.name).unwrap()
        } else {
            menu.name
        };
        r_str.push_str(&format!("{} \\\\\n", m_name));
        self.add_sub(&mut level, menu, menu_list, &mut r_str);
        self.template = self.template.replace(replace, &r_str);
    }

    fn save(&self, path: &str) -> Result<()> {
        let mut output = File::create(path)?;
        write!(output, "{}", &self.template)
    }

    fn add_sub(
        &self,
        level: &mut u32,
        d_menu: &Menu, 
        menu_list: &[Menu],
        r_str: &mut String,
    ) {
        let mut line = String::new();
        for _ in 1..*level {
            line += "│\\hspace{2em}";
        }

        for menu_entry in d_menu.items {
            match menu_entry.content {
                MenuItemContent::EditItem(item) => {
                    if item == Editable::Return {
                        let line = format!("{}└── {} \\\\\n", line, item.name());
                        r_str.push_str(&line);
                    } else {
                        let line = format!("{}├── {} \\\\\n", line, item.name());
                        r_str.push_str(&line);
                    }
                }
                MenuItemContent::MenuItem() => {
                    let sub_menu = menu_list[menu_entry.next_menu_idx];
                    let m_name = if REPLMNTS.contains_key(sub_menu.name) {
                        REPLMNTS.get(sub_menu.name).unwrap()
                    } else {
                        sub_menu.name
                    };

                    let line = format!("{}├── {} \\\\\n", line, m_name);
                    r_str.push_str(&line);
                    *level += 1;
                    self.add_sub(level, &sub_menu, menu_list, r_str);
                }
            }
        }
        if &line != "" {
            let line = format!("{} \\\\\n", line);
            r_str.push_str(&line);
        }
        *level -= 1;
    }
}


fn main() {
    let mut menus = Menus::new();
    menus.add(
        "@@flight-menu@@",
        &full::MENU_LIST[FLIGHT_MENU_IDX], 
        full::MENU_LIST
    );
    menus.add(
        "@@settings-menu@@",
        &full::MENU_LIST[SETTINGS_IDX],
        full::MENU_LIST,
    );
    menus.save("../../doc/tex/menus.tex").expect("Could not write file");
}
    

