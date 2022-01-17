#![allow(dead_code)]

use crate::widgets::base::{BuildContext, _Widget, Widget};
use crate::shorts::utility::Rect;
use sdl2::render::WindowCanvas;
use sdl2::mouse::MouseButton;
use crate::widgets::container::ContainerWidget;
use crate::widgets::text::TextWidget;
use crate::widgets::svg::{SVG, SVGWidget};
use std::collections::{HashSet, HashMap};
use sdl2::pixels::Color;

pub type ActionClosure = Box<dyn FnMut(BuildContext)>;

pub struct ActionWidget {
    child: Widget,
    closure: ActionClosure,

    context: Option<BuildContext>,
}

impl ActionWidget {
    pub fn new(child: Widget, closure: ActionClosure) -> Box<ActionWidget> {
        Box::new(ActionWidget {
            child,
            closure,
            context: None,
        })
    }
}

impl _Widget for ActionWidget {
    fn update(self: &mut Self, context: BuildContext) -> Result<Rect, String> {
        let context = context;
        self.child.update(context)
    }

    fn render(self: &mut Self, canvas: &mut WindowCanvas) -> Result<(), String> {
        self.child.render(canvas)
    }

    fn touch(self: &mut Self) {
        (self.closure)(self.context.as_ref().unwrap().clone());
        self.child.touch()
    }

    fn rect(&self) -> Rect {
        self.child.rect()
    }

    fn flex(&self) -> u8 {
        self.child.flex()
    }

    fn str(&self) -> String {
        format!("ActionWidget")
    }

    fn fmt(&self) -> String {
        format!("ActionWidget({})", self.child.str())
    }
}

pub type ButtonClosure = Box<dyn FnMut(BuildContext)>;

pub struct ButtonWidget {
    child: Widget,

    on_hover: ButtonClosure,
    be_hovered: bool,
    is_hover: bool,
    on_free: ButtonClosure,
    is_free: bool,

    on_click: ButtonClosure,
    is_click: bool,
    on_clicked: ButtonClosure,
    is_clicked: bool,

    context: Option<BuildContext>,
}

impl ButtonWidget {
    pub fn new(child: Widget,
               on_hover: ButtonClosure,
               on_click: ButtonClosure,
               on_clicked: ButtonClosure,
               on_free: ButtonClosure,
    ) -> Box<ButtonWidget> {
        Box::new(ButtonWidget {
            child,
            on_hover,
            on_click,
            is_click: false,
            on_clicked,
            on_free,
            context: None,
            is_hover: false,
            is_free: false,
            is_clicked: false,
            be_hovered: false,
        })
    }

    fn stub() -> ButtonClosure { Box::new(|_context| {}) }

    pub fn simple(child: Widget,
                  on_clicked: ButtonClosure,
    ) -> Box<ButtonWidget> {
        ButtonWidget::new(
            child,
            ButtonWidget::stub(),
            ButtonWidget::stub(),
            on_clicked,
            ButtonWidget::stub(),
        )
    }

    pub fn text_button_simple(text: String,
                              on_clicked: ButtonClosure,
    ) -> Box<ButtonWidget> {
        ButtonWidget::simple(
            ContainerWidget::center(
                TextWidget::simple(text)
            ),
            on_clicked,
        )
    }

    pub fn svg_simple(svg: SVG,
                      on_clicked: ButtonClosure,
    ) -> Widget {
        ButtonWidget::simple(
            SVGWidget::new(svg),
            on_clicked,
        )
    }
}

impl _Widget for ButtonWidget {
    fn update(self: &mut Self, context: BuildContext) -> Result<Rect, String> {
        let context = context;
        let interactions = context.interactions.as_ref().unwrap();
        let now_hovered = interactions.mouse.is_in(context.rect);
        if now_hovered && !self.be_hovered {
            self.is_hover = true;
        } else if !now_hovered && self.be_hovered {
            self.is_free = true;
        }
        if now_hovered {
            match interactions.mouse.is_pushed(MouseButton::Left) {
                None => {}
                Some(_) => { self.is_click = true }
            }
            match interactions.mouse.is_released(MouseButton::Left) {
                None => {}
                Some(((x1, y1), (x2, y2))) => {
                    if context.rect.contains_point((x1, y1)) &&
                        context.rect.contains_point((x2, y2)) {
                        self.is_clicked = true;
                    }
                }
            }
        }
        let child_rect = self.child.update(context.clone())?;
        self.context.replace(context.with_rect(child_rect));
        Ok(child_rect)
    }

    fn render(self: &mut Self, canvas: &mut WindowCanvas) -> Result<(), String> {
        self.child.render(canvas)
    }

    fn touch(self: &mut Self) {
        if self.is_hover {
            // println!("is_hover");
            (self.on_hover)(self.context.as_ref().unwrap().clone());
            self.is_hover = false;
        }
        if self.is_free {
            // println!("is_free");
            (self.on_free)(self.context.as_ref().unwrap().clone());
            self.is_free = false;
        }
        if self.is_click {
            // println!("is_click");
            (self.on_click)(self.context.as_ref().unwrap().clone());
            self.is_click = false;
        }
        if self.is_clicked {
            // println!("is_clicked");
            (self.on_clicked)(self.context.as_ref().unwrap().clone());
            self.is_clicked = false;
        }
        self.child.touch();
    }

    fn rect(&self) -> Rect {
        self.child.rect()
    }

    fn flex(&self) -> u8 {
        self.child.flex()
    }

    fn str(&self) -> String {
        format!("ButtonClosure")
    }

    fn fmt(&self) -> String {
        format!("ButtonClosure({})", self.child.str())
    }
}

/// Type to define before "boxing" for [RebuildingButtonClosure]
pub type _RebuildingButtonClosure = fn(BuildContext) -> Option<Widget>;

/// Type for [RebuildingButtonWidget]
pub type RebuildingButtonClosure = Box<dyn FnMut(BuildContext) -> Option<Widget>>;

pub struct RebuildingButtonWidget {
    child: Widget,

    be_hover: bool,
    be_click: bool,

    on_hover: RebuildingButtonClosure,
    is_hover: bool,
    on_free: RebuildingButtonClosure,
    is_free: bool,

    click_buttons: HashSet<MouseButton>,
    on_click: RebuildingButtonClosure,
    is_click: bool,
    on_clicked: RebuildingButtonClosure,
    is_clicked: bool,

    context: Option<BuildContext>,
}

impl RebuildingButtonWidget {
    fn stub() -> RebuildingButtonClosure {
        Box::new(move |_| { None })
    }

    pub fn new<OnHover, OnFree, OnClick, OnClicked>(child: Widget,
                                                    on_hover: OnHover,
                                                    on_free: OnFree,
                                                    click_buttons: HashSet<MouseButton>,
                                                    on_click: OnClick,
                                                    on_clicked: OnClicked,
    ) -> Widget
        where
            OnHover: Into<Option<RebuildingButtonClosure>>,
            OnFree: Into<Option<RebuildingButtonClosure>>,
            OnClick: Into<Option<RebuildingButtonClosure>>,
            OnClicked: Into<Option<RebuildingButtonClosure>>,
    {
        fn unpack_or_stub<F: Into<Option<RebuildingButtonClosure>>>(f: F) -> RebuildingButtonClosure {
            match f.into() {
                None => { RebuildingButtonWidget::stub() }
                Some(f) => { f }
            }
        }
        let on_hover = unpack_or_stub(on_hover);
        let on_free = unpack_or_stub(on_free);
        let on_click = unpack_or_stub(on_click);
        let on_clicked = unpack_or_stub(on_clicked);
        Box::new(RebuildingButtonWidget {
            child,
            be_hover: false,
            be_click: false,
            on_hover,
            is_hover: false,
            on_free,
            is_free: false,
            click_buttons,
            on_click,
            is_click: false,
            on_clicked,
            is_clicked: false,
            context: None,
        })
    }
}

/// SVG implementation
impl RebuildingButtonWidget {
    /// SVG base button
    ///
    /// may change svg on each event (default | hover | click)
    pub fn svg<H, C>(default: SVG,
                     hover: H,
                     click: C,
                     on_clicked: ButtonClosure,
                     click_buttons: HashSet<MouseButton>,
    ) -> Widget
        where
            H: Into<Option<SVG>>,
            C: Into<Option<SVG>>,
    {
        let default = default;
        let mut _on_clicked = on_clicked;
        fn pair_stub_or_svg<_SVG: Into<Option<SVG>>>(
            default: SVG,
            svg: _SVG,
        ) -> (RebuildingButtonClosure, RebuildingButtonClosure)
        {
            match svg.into() {
                None => {
                    let f1: RebuildingButtonClosure = Box::new(move |_context| { None });
                    let f2: RebuildingButtonClosure = Box::new(move |_context| { None });
                    (f1, f2)
                }
                Some(svg) => {
                    let f1: RebuildingButtonClosure = Box::new(
                        move |_context| { Some(SVGWidget::colored(svg.clone(), Color::WHITE)) }
                    );
                    let f2: RebuildingButtonClosure = Box::new(
                        move |_context| { Some(SVGWidget::colored(default.clone(), Color::WHITE)) }
                    );
                    (f1, f2)
                }
            }
        }
        let (on_hover, on_free) =
            pair_stub_or_svg(default.clone(), hover);
        let (on_click, mut on_clicked) =
            pair_stub_or_svg(default.clone(), click);
        let on_clicked: RebuildingButtonClosure = Box::new(move |context| {
            _on_clicked(context.clone());
            on_clicked(context)
        });

        let child = SVGWidget::colored(default, Color::WHITE);
        RebuildingButtonWidget::new(
            child,
            on_hover,
            on_free,
            click_buttons,
            on_click,
            on_clicked,
        )
    }
}

impl _Widget for RebuildingButtonWidget {
    fn update(self: &mut Self, context: BuildContext) -> Result<Rect, String> {
        let context = context;
        let child_rect = self.child.update(context.clone())?;
        self.context.replace(context.with_rect(child_rect));
        Ok(child_rect)
    }

    fn render(self: &mut Self, canvas: &mut WindowCanvas) -> Result<(), String> {
        self.child.render(canvas)
    }

    fn touch(self: &mut Self) {
        let context = self.context.as_ref().unwrap();
        let interactions = context.interactions.as_ref().unwrap();
        let mouse_in = interactions.mouse.is_in(self.rect());
        let try_actions = interactions.mouse.touch(self.rect());
        if try_actions.is_some() {
            let actions: HashMap<MouseButton, bool> = try_actions.unwrap().into_iter().filter(|e| {
                self.click_buttons.contains(&e.0)
            }).collect();
            let mut is_clicked = false;
            actions.iter().for_each(|(_e, r)| {
                is_clicked |= r;
            });
            let is_click = !is_clicked & !actions.is_empty();
            if self.be_click & is_clicked {
                self.is_clicked = true;
                self.be_click = false;
            } else if !self.be_click & is_click {
                self.is_click = true;
            }
            if is_click {
                self.be_click = true;
            }
        }
        if self.be_hover != mouse_in {
            if self.be_hover {
                self.is_free = true;
                self.be_hover = false;
            } else {
                self.is_hover = true;
                self.be_hover = true;
            }
        }

        let mut mute_child = false;
        if self.is_clicked {
            // println!("is_clicked");
            let res: Option<Widget> = (self.on_clicked)(self.context.clone().unwrap());
            if res.is_some() {
                self.child = res.unwrap();
                mute_child = true;
            }
            self.be_hover = false;
        } else if self.is_click {
            // println!("is_click");
            let res: Option<Widget> = (self.on_click)(self.context.clone().unwrap());
            if res.is_some() {
                self.child = res.unwrap();
                mute_child = true;
            }
        }
        self.is_clicked = false;
        self.is_click = false;

        // second: hovers
        if self.is_hover & !mute_child {
            // println!("is_hover");
            let res: Option<Widget> = (self.on_hover)(self.context.clone().unwrap());
            if res.is_some() {
                self.child = res.unwrap();
                mute_child = true;
            };
        } else if self.is_free & !mute_child {
            // println!("is_free");
            let res: Option<Widget> = (self.on_free)(self.context.clone().unwrap());
            if res.is_some() {
                self.child = res.unwrap();
                mute_child = true;
            }
        }
        self.is_hover = false;
        self.is_free = false;

        if !mute_child {
            self.child.touch();
        }
    }

    fn rect(&self) -> Rect {
        self.child.rect()
    }

    fn flex(&self) -> u8 {
        self.child.flex()
    }

    fn str(&self) -> String {
        format!("RebuildingButtonWidget")
    }

    fn fmt(&self) -> String {
        format!("RebuildingButtonWidget({})", self.child.str())
    }
}
