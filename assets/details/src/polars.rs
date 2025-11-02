use corelib::flight_physics::{polar_store::POLARS, polar_store_idx::TO_RAW};

use rust_xlsxwriter::{Format, Workbook, XlsxError, FormatAlign};

pub fn add_polars(workbook: &mut Workbook) -> Result<(), XlsxError> {
    let worksheet = workbook.add_worksheet().set_name("Polar Store")?;

    let header_format = Format::new().set_font_size(14).set_bold();
    let _ = worksheet.write_with_format(0, 0, "Polar Store", &header_format)?;

    let th_format = Format::new().set_bold().set_align(FormatAlign::Center);
    let _ = worksheet.write_with_format(2, 0, "Name", &th_format)?;
    let _ = worksheet.write_with_format(2, 1, "Wing Area", &th_format)?;
    let _ = worksheet.write_with_format(2, 2, "Max Speed", &th_format)?;
    let _ = worksheet.write_with_format(2, 3, "Empty Mass", &th_format)?;
    let _ = worksheet.write_with_format(2, 4, "Max Ballast", &th_format)?;
    let _ = worksheet.write_with_format(2, 5, "Reference Weight", &th_format)?;
    let _ = worksheet.write_with_format(2, 6, "Handicap", &th_format)?;
    let _ = worksheet.write_with_format(2, 7, "v1", &th_format)?;
    let _ = worksheet.write_with_format(2, 8, "si1", &th_format)?;
    let _ = worksheet.write_with_format(2, 9, "v2", &th_format)?;
    let _ = worksheet.write_with_format(2, 10, "si2", &th_format)?;
    let _ = worksheet.write_with_format(2, 11, "v3", &th_format)?;
    let _ = worksheet.write_with_format(2, 12, "si3", &th_format)?;

    let _ = worksheet.set_column_width(0, 15.0)?;
    let _ = worksheet.set_column_width(1, 12.0)?;
    let _ = worksheet.set_column_width(2, 12.0)?;
    let _ = worksheet.set_column_width(3, 12.0)?;
    let _ = worksheet.set_column_width(4, 12.0)?;
    let _ = worksheet.set_column_width(5, 12.0)?;
    let _ = worksheet.set_column_width(6, 12.0)?;
    let _ = worksheet.set_column_width(7, 7.0)?;
    let _ = worksheet.set_column_width(8, 7.0)?;
    let _ = worksheet.set_column_width(9, 7.0)?;
    let _ = worksheet.set_column_width(10, 7.0)?;
    let _ = worksheet.set_column_width(11, 7.0)?;
    let _ = worksheet.set_column_width(12, 7.0)?;


    let format_0 = Format::new().set_num_format("0");
    let format_1 = Format::new().set_num_format("0.0");
    let format_3 = Format::new().set_num_format("0.000");

    let mut row = 3_u32;
    for idx in TO_RAW {
        let polar = POLARS[*idx as usize];
        let _ = worksheet.write(row, 0, polar.name);
        let _ = worksheet.write_with_format(row, 1, polar.wing_area, &format_1);
        let _ = worksheet.write_with_format(row, 2, polar.max_speed, &format_0);
        let _ = worksheet.write_with_format(row, 3, polar.empty_mass, &format_0);
        let _ = worksheet.write_with_format(row, 4, polar.max_ballast, &format_0);
        let _ = worksheet.write_with_format(row, 5, polar.reference_weight, &format_0);
        let _ = worksheet.write_with_format(row, 6, polar.handicap, &format_0);
        let _ = worksheet.write_with_format(row, 7, polar.polar_values[0][0], &format_1);
        let _ = worksheet.write_with_format(row, 8, polar.polar_values[0][1], &format_3);
        let _ = worksheet.write_with_format(row, 9, polar.polar_values[1][0], &format_1);
        let _ = worksheet.write_with_format(row, 10, polar.polar_values[1][1], &format_3);
        let _ = worksheet.write_with_format(row, 11, polar.polar_values[2][0], &format_1);
        let _ = worksheet.write_with_format(row, 12, polar.polar_values[2][1], &format_3);
        row += 1;
    }
    Ok(())
}
