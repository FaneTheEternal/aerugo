#![allow(dead_code)]

use crate::widgets::base::{BuildContext, _Widget, Widget};
use crate::shorts::utility::Rect;
use sdl2::render::{WindowCanvas, BlendMode};
use std::cmp::{min};
use sdl2::pixels::{PixelFormatEnum, Color};
use tiny_skia::Pixmap;

/// (type, name): (String, String)
/// type is ("regular" | "solid")
pub type SVG = (String, String);

struct SVGCache {
    base_rect: Rect,
    self_rect: Rect,
    cache: Pixmap,
}

pub struct SVGWidget {
    svg: SVG,
    color: Color,
    eternal_load: bool,

    context: Option<BuildContext>,
    cache: Option<SVGCache>,
}

impl SVGWidget {
    pub fn make(svg: SVG, color: Color, eternal_load: bool) -> Widget {
        Box::new(SVGWidget {
            svg,
            color,
            eternal_load,
            context: None,
            cache: None,
        })
    }

    pub fn new(svg: SVG) -> Widget {
        SVGWidget::make(svg, Color::BLACK, false)
    }

    pub fn colored(svg: SVG, color: Color) -> Widget {
        SVGWidget::make(svg, color, false)
    }

    pub fn eternal(svg: SVG) -> Widget {
        SVGWidget::make(svg, Color::BLACK, true)
    }

    pub fn eternal_colored(svg: SVG, color: Color) -> Widget {
        SVGWidget::make(svg, color, true)
    }
}

impl _Widget for SVGWidget {
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
        let mut rect = context.rect.clone();
        rect.set_width(min(rect.width(), rect.height()));

        let mut pixmap = Pixmap::new(rect.width(), rect.height()).unwrap();
        let (w, h) = rect.size();
        if self.eternal_load {
            let mut opt = usvg::Options::default();
            opt.fontdb.load_system_fonts();
            let paths = format!("./svgs/{}/{}", self.svg.0, self.svg.1);
            let svg_data = std::fs::read(&paths).unwrap();
            let rtree = usvg::Tree::from_data(&svg_data, &opt).unwrap();
            resvg::render(&rtree, usvg::FitTo::Size(w, h), pixmap.as_mut()).unwrap();
        } else {
            let rtree = match context.svgs.get(&self.svg).clone() {
                None => { return Err(format!("Can't find svg {}|{}", self.svg.0, self.svg.1)) }
                Some(tree) => { tree }
            };
            resvg::render(rtree, usvg::FitTo::Size(w, h), pixmap.as_mut()).unwrap();
        }

        let (r, g, b, a) = self.color.rgba();
        pixmap.data_mut().chunks_mut(4).for_each(|p| {
            if p[3] > 0 {
                p[0] = r;
                p[1] = g;
                p[2] = b;
                p[3] = a;
            }
        });

        self.context.replace(context.with_rect(rect.clone()));
        self.cache.replace(SVGCache {
            base_rect: context.rect,
            self_rect: rect,
            cache: pixmap,
        });
        // println!("{:?}|{:?}", context.rect, rect);
        Ok(rect)
    }

    fn render(self: &mut Self, canvas: &mut WindowCanvas) -> Result<(), String> {
        let context = self.context.as_ref().unwrap().clone();
        let rect = self.rect();
        let mut texture = context.creator.create_texture_streaming(
            PixelFormatEnum::RGBA32,
            rect.width(), rect.height(),
        ).unwrap();
        let cache = self.cache.as_ref().unwrap();
        let data = &cache.cache;
        texture.set_blend_mode(BlendMode::Blend);
        texture.with_lock(None, |buff, _| {
            buff.copy_from_slice(data.data());
        })?;
        // println!("{:?}", rect);
        canvas.copy(&texture, None, self.rect())?;
        Ok(())
    }

    fn touch(self: &mut Self) {}

    fn rect(&self) -> Rect {
        self.context.as_ref().unwrap().rect
    }

    fn flex(&self) -> u8 {
        0
    }

    fn str(&self) -> String {
        format!("SVGWidget")
    }

    fn fmt(&self) -> String {
        format!("SVGWidget({:?})", self.svg)
    }
}
