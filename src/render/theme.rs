use eframe::egui::{Color32, Context, FontId, TextStyle};

use crate::graph::nodes::NodeType;

pub const FONT_SIZE: f32 = 16.0;
pub const SLIDER_HEIGHT: f32 = 28.0;
pub const SLIDER_RAIL_HEIGHT: f32 = 10.0;
pub const FOOTER_HEIGHT: f32 = 60.0;
pub const STEP_TEXT_WIDTH: f32 = 140.0;
pub const CURRENT_STEP_TEXT_WIDTH: f32 = 160.0;

pub const VISITED_COLOR: Color32 =
  Color32::from_rgb(241, 196, 15);

pub const RESULT_COLOR: Color32 =
  Color32::from_rgb(155, 89, 182);

pub fn node_color(node: NodeType) -> Color32 {
  match node {
    NodeType::START => Color32::from_rgb(46, 204, 113),
    NodeType::END => Color32::from_rgb(231, 76, 60),
    NodeType::PATH => Color32::from_rgb(220, 224, 230),
    NodeType::WATER => Color32::from_rgb(52, 152, 219),
    NodeType::WALL => Color32::from_rgb(30, 32, 36),
  }
}

pub fn configure_styles(ctx: &Context) {
  ctx.all_styles_mut(|style| {
    style.text_styles.insert(
      TextStyle::Body,
      FontId::proportional(FONT_SIZE),
    );
    style.text_styles.insert(
      TextStyle::Button,
      FontId::proportional(FONT_SIZE),
    );

    style.spacing.interact_size.y = SLIDER_HEIGHT;
    style.spacing.slider_rail_height = SLIDER_RAIL_HEIGHT;
  });
}
