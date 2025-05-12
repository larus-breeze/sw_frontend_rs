use crate::{
    model::{CoreModel, TypeOfInfo},
    tformat,
    utils::Colors,
    view::viewable::circle_area::draw_info,
    CoreError, DrawImage,
};

use embedded_graphics::draw_target::DrawTarget;

use super::viewable::circle_area::draw_alarm_info;

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
        match self.type_of_info {
            TypeOfInfo::WaterBallast => {
                let value = tformat!(20, "{:.0} kg", cm.glider_data.water_ballast.to_kg()).unwrap();
                draw_info(display, cm, "Water Ballast", value.as_str())?;
            }
            TypeOfInfo::GearAlarm => {
                draw_alarm_info(display, cm, "Landing Gear", cm.device_const.images.gear)?
            }
            TypeOfInfo::None => (),
        };
        Ok(())
    }
}
