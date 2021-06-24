#![allow(dead_code)]

use crate::widgets::base::{Widget, BuildContext};
use crate::shorts::utility::Rect;
use sdl2::render::{WindowCanvas};
use sdl2::ttf::{FontStyle};
use sdl2::pixels::Color;
use sdl2::surface::Surface;

pub struct TextWidget {
    data: String,
    font: String,
    font_size: u16,
    font_style: FontStyle,
    font_color: Color,

    context: Option<BuildContext>,

    is_init: bool,

}

impl TextWidget {
    pub fn new<Font, FontSize, FontStyle, FontColor>(data: String,
                                                     font: Font,
                                                     font_size: FontSize,
                                                     font_style: FontStyle,
                                                     font_color: FontColor) -> Box<TextWidget>
        where
            Font: Into<Option<String>>,
            FontSize: Into<Option<u16>>,
            FontStyle: Into<Option<sdl2::ttf::FontStyle>>,
            FontColor: Into<Option<Color>>,
    {
        let font = match font.into() {
            Some(f) => format!("./ttf/{}.ttf", f),
            None => "./ttf/TimesNewRoman.ttf".to_string()
        };
        let font_size = match font_size.into() {
            Some(s) => s,
            None => 20,
        };
        let font_style = match font_style.into() {
            Some(s) => s,
            None => sdl2::ttf::FontStyle::BOLD
        };
        let font_color = match font_color.into() {
            Some(c) => c,
            None => Color::RGBA(0, 0, 0, 255),
        };
        Box::new(TextWidget {
            data,
            font,
            font_size,
            font_style,
            font_color,
            context: None,
            is_init: false,
        })
    }

    pub fn simple(data: String) -> Box<TextWidget> {
        TextWidget::new(data, None, None, None, None)
    }
}

fn text_surface(context: BuildContext,
                text: &str,
                font: String,
                font_size: u16,
                font_style: FontStyle,
                font_color: Color,
) -> Result<Surface<'static>, String> {
    let context = context;
    let abs_rect = context.abs_rect;
    let abs = (abs_rect.width() + abs_rect.height()) / 2;
    let font_size = abs as u16 * font_size / 1000;
    let mut font = context.ttf_context.load_font(font, font_size)?;
    font.set_style(font_style);

    let surface = font
        .render(text)
        .blended(font_color);
    match surface {
        Ok(surface) => { Ok(surface) }
        Err(e) => { Err(e.to_string()) }
    }
}

impl Widget for TextWidget {
    fn update(self: &mut Self, context: BuildContext) -> Result<Rect, String> {
        let mut context = context;
        let surface = text_surface(
            context.clone(), self.data.as_str(), self.font.clone(),
            self.font_size, self.font_style, self.font_color)?;
        let rect = surface.rect();
        if context.rect.width() > rect.width() {
            context.rect.set_width(rect.width());
        }
        if context.rect.height() > rect.height() {
            context.rect.set_height(rect.height());
        }
        self.context.replace(context.clone());
        Ok(context.rect)
    }

    fn render(self: &mut Self, canvas: &mut WindowCanvas) -> Result<(), String> {
        let context = self.context.as_ref().unwrap().clone();
        let surface = text_surface(
            context.clone(), self.data.as_str(), self.font.clone(),
            self.font_size, self.font_style, self.font_color)?;
        let texture = match context.creator.create_texture_from_surface(surface) {
            Ok(t) => { t }
            Err(e) => { return Err(e.to_string()); }
        };
        let mut src = context.rect;
        src.reposition((0, 0));
        canvas.copy(&texture, src, context.rect)
    }

    fn touch(self: &mut Self) {
        ()
    }

    fn rect(&self) -> Rect {
        self.context.as_ref().unwrap().rect
    }

    fn flex(&self) -> u8 { 0 }

    fn str(&self) -> String {
        String::from("TextWidget")
    }

    fn fmt(&self) -> String {
        format!("TextWidget()")
    }
}