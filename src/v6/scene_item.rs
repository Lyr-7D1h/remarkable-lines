use self::{glyph_range::GlyphRange, group::Group, line::Line, text::Text};

pub mod glyph_range;
pub mod group;
pub mod line;
pub mod point;
pub mod text;

#[derive(Debug, Clone)]
pub enum SceneItem {
    Group(Group),
    Line(Line),
    Text(Text),
    GlyphRange(GlyphRange),
}
