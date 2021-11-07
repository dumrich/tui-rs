use crate::{buffer::Buffer, layout::Rect, widgets::Widget};

/// A widget to clear/reset a certain area to allow overdrawing (e.g. for popups).
///
/// This widget **cannot be used to clear the terminal on the first render** as `zui_widgets` assumes the
/// render area is empty. Use [`crate::Terminal::clear`] instead.
///
/// # Examples
///
/// ```
/// # use zui_widgets::widgets::{Clear, Block, Borders};
/// # use zui_widgets::layout::Rect;
/// # use zui_widgets::Frame;
/// # use zui_widgets::backend::Backend;
/// fn draw_on_clear<B: Backend>(f: &mut Frame<B>, area: Rect) {
///     let block = Block::default().title("Block").borders(Borders::ALL);
///     f.render_widget(Clear, area); // <- this will clear/reset the area first
///     f.render_widget(block, area); // now render the block widget
/// }
/// ```
///
/// # Popup Example
///
/// For a more complete example how to utilize `Clear` to realize popups see
/// the example `examples/popup.rs`
#[derive(Debug, Clone)]
pub struct Clear;

impl Widget for Clear {
    fn render(self, area: Rect, buf: &mut Buffer) {
        for x in area.left()..area.right() {
            for y in area.top()..area.bottom() {
                buf.get_mut(x, y).reset();
            }
        }
    }
}
