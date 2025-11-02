use corelib::menu;
use rust_xlsxwriter::{Format, Workbook, Worksheet, XlsxError};
use menu::{MenuItemContent, Menu};

pub fn add_menu_sturcture(workbook: &mut Workbook) -> Result<(), XlsxError> {
    let worksheet = workbook.add_worksheet().set_name("Menu Structure")?;

    let header_format = Format::new().set_font_size(14).set_bold();
    let _ = worksheet.write_with_format(0, 0, "Menu Structure", &header_format)?;

    let main_menu = menu::MENU_LIST[menu::SETTINGS_IDX];
    let mut row = 2_u32;
    store_menu(worksheet, &mut row, main_menu, 0);

    Ok(())
}

fn store_menu(worksheet: &mut Worksheet, row: &mut u32, d_menu: Menu, level: u16) {
    let s = format!("Menu <{}>", d_menu.name);
    let _ = worksheet.write(*row, level, &s);
    *row += 1;


    for menu_entry in d_menu.items {
        match menu_entry.content {
            MenuItemContent::EditItem(item) => {
                let _ = worksheet.write(*row, level + 1, item.name());
            }
            MenuItemContent::MenuItem() => {
                let sub_menu = menu::MENU_LIST[menu_entry.next_menu_idx];
                store_menu(worksheet, row, sub_menu, level + 1);
            }
        }
        *row += 1;
    }
}