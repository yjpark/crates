use bevy::prelude::*;
use edger_bevy_shape::bevy_prototype_lyon::prelude::*;

use crate::prelude::{LayoutData, OutlineRectangle, ShapeOp};

use super::theme::ViewShapeDevTheme;

impl ShapeOp<ViewShapeDevTheme, OutlineRectangle> for LayoutData {
    fn get_shape(&self, theme: &ViewShapeDevTheme) -> OutlineRectangle {
        let color = theme.layout.get_view_color();
        let outline_color = theme.layout.border_color;
        OutlineRectangle {
            width: self.size.width,
            height: self.size.height,
            origin: shapes::RectangleOrigin::from(*self),
            color,
            outline_width: theme.layout.border_line_width,
            outline_color,
            offset: Vec3::new(self.offset.x, self.offset.y, 2.0),
        }
    }
}
