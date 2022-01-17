#![allow(dead_code)]

use crate::widgets::prelude::*;

pub const READ_SCREEN: _SFClosure = |context, root_key| {
    ContainerWidget::expand_wrap(
        ColumnWidget::new(vec![
            READ_FILLER(context.clone()),  // TODO: Wrap in [SLClosureWidget]
            READ_FOOTER(context, root_key),
        ])
    )
};

/// Const of size for read grid
///
/// Text with controls be of one to [FILL_SIZE] of screen
const FILL_SIZE: u8 = 5;

const READ_FILLER: _SLClosure = |_context| {
    ContainerWidget::expand_filler(FILL_SIZE)
};

const READ_FOOTER: _SFClosure = |context, root_key| {
    ContainerWidget::expand_indent(
        RowWidget::new(vec![
            LEFT_BUTTON(context.clone(), root_key),
            // TEXT_WIDGET(context, root_key),
            RIGHT_BUTTON(context.clone(), root_key),
        ]),
        Indent::All(10),
    )
};

const LEFT_BUTTON: _SFClosure = |_context, _root_key| {
    todo!()
};

const RIGHT_BUTTON: _SFClosure = |_context, _root_key| {
    todo!()
};
