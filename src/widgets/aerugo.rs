use super::prelude::*;
use sdl2::ttf::FontStyle;
use sdl2::pixels::Color;

use crate::shorts::utility::*;
use crate::{str, svg, rect};
use crate::widgets::snippets::SVG_BUTTON;

fn logo_layer() -> Widget {
    ContainerWidget::colored(
        TextWidget::new(
            str!("Aerugo"),
            str!("JetBrainsMono"),
            20,
            FontStyle::ITALIC | FontStyle::BOLD,
            Color::WHITE,
        ),
        Color::BLACK,
    )
}

fn load_rows() -> Vec<Widget> {
    let mut rows: Vec<Widget> = Vec::new();
    {  // svg
        let svg: _SFClosure = move |context, key| {
            let context = context;
            let check_svg: Closure = Box::new(move |context| {
                let context = context;
                if context.aerugo.borrow().svg().is_some() {
                    context.widgets_states.borrow_mut().update(key.clone());
                }
            });
            let icon: Widget = if context.aerugo.borrow().svg().is_none() {
                SVGWidget::eternal_colored(svg!("solid", "spinner.svg"), Color::WHITE)
            } else {
                SVGWidget::eternal_colored(svg!("solid", "check.svg"), Color::WHITE)
            };
            ClosureWidget::new(
                icon,
                check_svg,
            )
        };
        rows.push(ContainerWidget::expand_wrap(
            RowWidget::new(vec![
                BoundWidget::new(
                    Some(SFClosureWidget::new(Box::new(svg))),
                    rect!(20, 20),
                ),
                TextWidget::new(
                    str!("svg resource"),
                    str!("JetBrainsMono"),
                    3,
                    FontStyle::ITALIC,
                    Color::WHITE,
                )
            ])
        ))
    }
    rows
}

fn load_layer() -> Widget {
    ContainerWidget::center(
        RowWidget::new(
            vec![
                ContainerWidget::expand_filler(2),
                ContainerWidget::expand_wrap(
                    ColumnWidget::new(
                        vec![
                            ContainerWidget::expand_filler(3),
                            ContainerWidget::expand_wrap(
                                ColumnWidget::new(load_rows())
                            )
                        ]
                    )
                ),
            ]
        )
    )
}

pub const AERUGO: _SFClosure = |context, key| {
    let context = context;
    let closure: Closure = Box::new(move |context| {
        let context = context;
        if context.aerugo.borrow().resources_loaded() {
            context.widgets_states.borrow_mut().update(key.clone());
        }
    });

    if !context.aerugo.borrow().resources_loaded() {
        ClosureWidget::new(
            StackWidget::new(
                vec![
                    logo_layer(),
                    load_layer(),
                ],
                None,
            ),
            closure,
        )
    } else {
        ContainerWidget::center(
            SLClosureWidget::new(Box::new(SVG_BUTTON))
            // TextWidget::new(
            //     str!("Some Game"),
            //     None,
            //     None,
            //     None,
            //     Color::WHITE,
            // )
        )
    }
};
