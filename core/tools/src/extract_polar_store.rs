//use rust_xlsxwriter::{Format, Workbook, XlsxError, FormatAlign};
use corelib::{
    flight_physics::{
        polar_store::{POLARS, BasicGliderData},
        polar_store_idx::TO_RAW,
    }
};
use std:: {
    fs::File,
    io::Write,
};

fn save(path: &str) {
    let mut file = File::create(path).unwrap();
    write_header(&mut file);
    for idx in TO_RAW {
        let gd = POLARS[*idx as usize];
        write_line(&mut file, &gd)
    }
    println!("File {} written", path);
}

fn write_header(file: &mut File) {
    writeln!(
        file, 
        "{}", 
        "Name,Wing Area,Max Speed,Empty Mass,Max Ballast,Reference Weight,Handicap,v1,si1,v2,si2,v3,si3"
    ).unwrap();        
}

fn write_line(file: &mut File, gd: &BasicGliderData) {
    writeln!(
        file, 
        "{},{:.1},{:.0},{:.0},{:.0},{:.0},{:.0},{:.0},{:.3},{:.0},{:.3},{:.0},{:.3}",
        gd.name,
        gd.wing_area,
        gd.max_speed,
        gd.empty_mass,
        gd.max_ballast,
        gd.reference_weight,
        gd.handicap,
        gd.polar_values[0][0],
        gd.polar_values[0][1],
        gd.polar_values[1][0],
        gd.polar_values[1][1],
        gd.polar_values[2][0],
        gd.polar_values[2][1],
    ).unwrap();        
}

fn main() {
    save("../doc/polar_store.csv");
}
