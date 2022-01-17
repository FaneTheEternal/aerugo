#![allow(dead_code)]

use crate::widgets::base::{_Widget, BuildContext};
use crate::shorts::utility::Rect;
use sdl2::render::{WindowCanvas};
use sdl2::ttf::{FontStyle};
use sdl2::pixels::Color;
use sdl2::surface::Surface;

struct TextCache {
    /// source rect
    base_rect: Rect,
    /// result rect
    self_rect: Rect,
    surface: Surface<'static>,
}

pub struct TextWidget {
    data: String,
    font: String,
    font_size: u16,
    font_style: FontStyle,
    font_color: Color,

    context: Option<BuildContext>,

    cache: Option<TextCache>,

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
            None => 10,
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
            cache: None,
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
                max_width: u32,
) -> Result<Surface<'static>, String> {
    let context = context;
    let abs_rect = context.abs_rect;
    let abs = (abs_rect.width() + abs_rect.height()) / 2;
    let font_size = abs as u16 * font_size / 100;
    let mut font = context.ttf_context.load_font(font, font_size)?;
    font.set_style(font_style);

    let surface = font
        .render(text)
        .blended_wrapped(font_color, max_width);
    match surface {
        Ok(surface) => { Ok(surface) }
        Err(e) => { Err(e.to_string()) }
    }
}

impl _Widget for TextWidget {
    fn update(self: &mut Self, context: BuildContext) -> Result<Rect, String> {
        let context = context;

        {  // try pass with cache
            if self.cache.is_some() {
                let cache = self.cache.as_ref().unwrap();
                if cache.base_rect == context.rect {
                    return Ok(cache.self_rect);
                }
            }
        }

        let surface = text_surface(
            context.clone(), self.data.as_str(), self.font.clone(),
            self.font_size, self.font_style, self.font_color, context.rect.width())?;
        let mut rect = surface.rect();
        rect.reposition(context.rect.top_left());
        if rect.height() > context.rect.height() {
            rect.set_height(context.rect.height());
        }
        self.context.replace(context.with_rect(rect));

        self.cache.replace(TextCache {
            base_rect: context.rect,
            self_rect: rect,
            surface,
        });
        Ok(rect)
    }

    fn render(self: &mut Self, canvas: &mut WindowCanvas) -> Result<(), String> {
        // return Ok(());
        let context = self.context.as_ref().unwrap().clone();
        let surface = self.cache.as_ref().unwrap().surface.as_ref();
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