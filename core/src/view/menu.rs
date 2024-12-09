use super::helpers::themes::{Palette, FONT_BIG};
use crate::{
    basic_config::{DISPLAY_HEIGHT, DISPLAY_WIDTH},
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
                (TString::<16>::from_str(""), cm.color(Palette::Background))
            } else {
                let menu_item = menu.items[item_idx as usize];
                let color = if menu_idx == 3 {
                    if menu_item.is_menu() {
                        cm.color(Palette::Text2Bold)
                    } else {
                        cm.color(Palette::Text1Bold)
                    }
                } else if menu_item.is_menu() {
                        cm.color(Palette::Text2)
                } else {
                    cm.color(Palette::Text1)
                };
                (menu_item.name(), color)
            }
        }

        let menu = cm.control.menu_control.menu;
        let pos = cm.control.menu_control.pos[menu.level];

        display.clear(cm.color(Palette::Background))?;

        FONT_BIG.render_aligned(
            menu.name,
            Point::new(DISPLAY_WIDTH as i32 / 2, DISPLAY_HEIGHT as i32 / 14),
            VerticalPosition::Top,
            HorizontalAlignment::Center,
            FontColor::Transparent(cm.color(Palette::Scale)),
            display,
        )?;

        let y = DISPLAY_HEIGHT as i32 / 5;

        Line::new(Point::new(0, y), Point::new(DISPLAY_WIDTH as i32, y))
            .into_styled(PrimitiveStyle::with_stroke(cm.color(Palette::Scale), 1))
            .draw(display)?;

        let mut y_pos = DISPLAY_HEIGHT as i32 / 4;
        const DELTA_Y: i32 = DISPLAY_HEIGHT as i32 / 10;

        const X_MARGIN: i32 = (5 * DISPLAY_WIDTH / 100) as i32;
        const Y_DELTA: i32 = 285 * DELTA_Y / 100;
        const Y_SIZE: u32 = 110 * DELTA_Y as u32 / 100;
        let style = PrimitiveStyle::with_stroke(cm.color(Palette::Scale), 1);

        Rectangle::new(
            Point::new(X_MARGIN, y_pos + Y_DELTA),
            Size::new(DISPLAY_WIDTH - 2 * X_MARGIN as u32, Y_SIZE),
        )
        .into_styled(style)
        .draw(display)?;

        for menu_idx in 0..7 {
            let (s, color) = get_str(menu, pos, menu_idx, cm);
            FONT_BIG.render_aligned(
                s.as_str(),
                Point::new(DISPLAY_WIDTH as i32 / 2, y_pos),
                VerticalPosition::Top,
                HorizontalAlignment::Center,
                FontColor::Transparent(color),
                display,
            )?;
            y_pos += DELTA_Y;
        }
        Ok(())
    }
}
