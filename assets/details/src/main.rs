mod menu_structure;
mod polars;

use menu_structure::add_menu_sturcture;
use polars::add_polars;
use rust_xlsxwriter::{Workbook, XlsxError};


fn main() -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    add_menu_sturcture(&mut workbook)?;
    add_polars(&mut workbook)?;

    let path = "../../doc/details.xlsx";
    workbook.save(path)?;
    println!("File '{}' created", path);

    Ok(())
}
