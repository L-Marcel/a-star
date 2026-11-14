use eframe::egui::{Align, Layout, Slider, Ui, vec2};

use crate::{
  graph::matrix::Step,
  render::theme::{
    CURRENT_STEP_TEXT_WIDTH, FONT_SIZE, FOOTER_HEIGHT,
    STEP_TEXT_WIDTH,
  },
};

pub fn render_footer(
  ui: &mut Ui,
  history: &[Step],
  current_step: &mut usize,
) {
  ui.allocate_ui(
    vec2(ui.available_width(), FOOTER_HEIGHT),
    |ui| {
      ui.horizontal(|ui| {
        ui.allocate_ui(
          vec2(CURRENT_STEP_TEXT_WIDTH, FOOTER_HEIGHT),
          |ui| {
            ui.centered_and_justified(|ui| {
              ui.label(format!(
                "Nó visitado {:?}",
                history[*current_step].0
              ));
            });
          },
        );

        ui.with_layout(
          Layout::right_to_left(Align::Center),
          |ui| {
            ui.allocate_ui(
              vec2(STEP_TEXT_WIDTH, FOOTER_HEIGHT),
              |ui| {
                ui.centered_and_justified(|ui| {
                  ui.label(format!(
                    "{:?}",
                    history[*current_step].1
                  ));
                });
              },
            );

            let font_width = FONT_SIZE * 4.5;
            let value_box_width = font_width.max(55.0);
            let gap = ui.spacing().item_spacing.x;

            let slider_width =
              ui.available_width() - value_box_width - gap;

            ui.spacing_mut().slider_width =
              slider_width.max(10.0);

            ui.add(Slider::new(
              current_step,
              0..=(history.len().saturating_sub(1)),
            ));
          },
        );
      });
    },
  );
}
