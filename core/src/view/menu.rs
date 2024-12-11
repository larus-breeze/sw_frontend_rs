use crate::{
    model::{menu::Menu, CoreModel},
    utils::Colors,
    CoreError, DrawImage, TString,
};
use u8g2_fonts::types::{FontColor, HorizontalAlignment, VerticalPosition};

use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::Point,
    prelude::Size,
    primitives::{Line, Primitive, PrimitiveStyle, Rectangle},
    Drawable,
};

pub struct MenuView {}

impl MenuView {
    pub fn new() -> MenuView {
        MenuView {}
    }

    pub fn draw<D>(&self, display: &mut D, cm: &CoreModel) -> Result<(), CoreError>
    where
        D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
    {
        fn get_str(
            menu: &Menu,
            pos: usize,
            menu_idx: isize,
            cm: &CoreModel,
        ) -> (TString<16>, Colors) {
            let idx_max = (menu.items.len() - 1) as isize;
            let item_idx = pos as isize + menu_idx - 3;
            if item_idx < 0 || item_idx > idx_max {
                (TString::<16>::from_str(""), cm.palette().background)
            } else {
                let menu_item = menu.items[item_idx as usize];
                let color = if menu_idx == 3 {
                    if menu_item.is_menu() {
                        cm.palette().text2_bold
                    } else {
                        cm.palette().text1_bold
                    }
                } else if menu_item.is_menu() {
                    cm.palette().text2
                } else {
                    cm.palette().text1
                };
                (menu_item.name(), color)
            }
        }

        let menu = cm.control.menu_control.menu;
        let pos = cm.control.menu_control.pos[menu.level];
        let d_sizes = &cm.device_const.sizes.display;

        display.clear(cm.palette().background)?;

        cm.device_const.big_font.render_aligned(
            menu.name,
            Point::new(d_sizes.width as i32 / 2, d_sizes.height as i32 / 14),
            VerticalPosition::Top,
            HorizontalAlignment::Center,
            FontColor::Transparent(cm.palette().scale),
            display,
        )?;

        let y = d_sizes.height as i32 / 5;

        Line::new(Point::new(0, y), Point::new(d_sizes.width as i32, y))
            .into_styled(PrimitiveStyle::with_stroke(cm.palette().scale, 1))
            .draw(display)?;

        let mut y_pos = d_sizes.height as i32 / 4;
        let delta_y = d_sizes.height as i32 / 10;

        let x_margin = (5 * d_sizes.width / 100) as i32;
        let y_delta = 285 * delta_y / 100;
        let y_size = 110 * delta_y as u32 / 100;
        let style = PrimitiveStyle::with_stroke(cm.palette().scale, 1);

        Rectangle::new(
            Point::new(x_margin, y_pos + y_delta),
            Size::new(d_sizes.width - 2 * x_margin as u32, y_size),
        )
        .into_styled(style)
        .draw(display)?;

        for menu_idx in 0..7 {
            let (s, color) = get_str(menu, pos, menu_idx, cm);
            cm.device_const.big_font.render_aligned(
                s.as_str(),
                Point::new(d_sizes.width as i32 / 2, y_pos),
                VerticalPosition::Top,
                HorizontalAlignment::Center,
                FontColor::Transparent(color),
                display,
            )?;
            y_pos += delta_y;
        }
        Ok(())
    }
}
