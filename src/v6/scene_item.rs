use self::{group::Group, line::Line, text::Text};

pub mod group;
pub mod line;
mod point;
pub mod text;

#[derive(Debug, Clone)]
pub enum SceneItem {
    Group(Group),
    Line(Line),
    Text(Text),
    GlyphRange,
}
