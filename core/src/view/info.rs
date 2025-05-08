use crate::{
    model::{CoreModel, TypeOfInfo}, 
    view::viewable::circle_area::draw_info,
    tformat, utils::Colors, CoreError, DrawImage,
};

use embedded_graphics::draw_target::DrawTarget;

#[derive(PartialEq)]
pub struct InfoView {
    type_of_info: TypeOfInfo,
}

impl InfoView {
    pub fn new(type_of_info: TypeOfInfo) -> Self {
        Self { type_of_info }
    }

    pub fn draw<D>(&self, display: &mut D, cm: &CoreModel) -> Result<(), CoreError>
    where
        D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
    {
        let (header, value) = match self.type_of_info {
            TypeOfInfo::WaterBallast => (
                "Water Ballast", 
                tformat!(20, "{:.0} kg", cm.glider_data.water_ballast.to_kg()).unwrap()
            ),
            TypeOfInfo::GearAlarm => (
                "Landing Gear Alarm", 
                tformat!(20, "").unwrap()
            ),
        };
        draw_info(display, cm, header, value.as_str())
    }
}
