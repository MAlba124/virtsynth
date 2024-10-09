use std::{cmp::Ordering, ops::RangeInclusive};

use eframe::{
    egui::{Painter, Pos2, Sense, Shape, Stroke, Vec2, Widget,},
    emath,
};

const ARC_ROTATION: f32 = std::f32::consts::PI / 6.0;

// From https://github.com/emilk/egui/blob/master/crates/egui/src/widgets/drag_value.rs#L741
pub fn clamp_value_to_range(x: f32, range: &RangeInclusive<f32>) -> f32 {
    let (mut min, mut max) = (*range.start(), *range.end());

    if min.total_cmp(&max) == Ordering::Greater {
        (min, max) = (max, min);
    }

    match x.total_cmp(&min) {
        Ordering::Less | Ordering::Equal => min,
        Ordering::Greater => match x.total_cmp(&max) {
            Ordering::Greater | Ordering::Equal => max,
            Ordering::Less => x,
        },
    }
}

#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct Knob<'a> {
    value: &'a mut f32,
    range: RangeInclusive<f32>,
    speed: f32,
}

impl<'a> Knob<'a> {
    pub fn new(value: &'a mut f32) -> Self {
        Self {
            value,
            range: 0.0..=1.0,
            speed: 0.01,
        }
    }

    pub fn range(mut self, range: RangeInclusive<f32>) -> Self {
        self.range = range;
        self
    }

    pub fn speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }
}

fn render_arc(
    painter: &Painter,
    center: &Pos2,
    start: f32,
    end: f32,
    radius: f32,
    stroke: &Stroke,
) {
    let segments = 50; // Overkill?
    let angle_step = (end - start) / segments as f32;

    let points: Vec<Pos2> = (0..=segments)
        .map(|i| {
            let angle = ARC_ROTATION - (start + i as f32 * angle_step);
            Pos2 {
                x: center.x + radius * angle.cos(),
                y: center.y + radius * angle.sin(),
            }
        })
        .collect();

    painter.add(Shape::line(points, *stroke));
}

impl<'a> Widget for Knob<'a> {
    fn ui(self, ui: &mut eframe::egui::Ui) -> eframe::egui::Response {
        let id = ui.next_auto_id();

        let old_value = *self.value;

        let size = Vec2::splat(64.0);

        let sense = Sense::drag();

        let (rect, mut response) = ui.allocate_at_least(size, sense);

        if ui.is_rect_visible(rect) {
            let visuals = ui.style().interact(&response);
            let painter = ui.painter();

            let center = rect.center();

            let arc_max = std::f32::consts::PI * 4.0 / 3.0;

            let mut stroke = visuals.bg_stroke;
            stroke.color = visuals.weak_bg_fill;
            stroke.width = 4.0;
            render_arc(&painter, &center, 0.0, arc_max, 28.0 - 4.0, &stroke);

            let progress_start = arc_max
                - arc_max
                    * ((clamp_value_to_range(old_value, &self.range) - self.range.start())
                        / (self.range.end() - self.range.start()));

            let mut stroke = visuals.fg_stroke;
            stroke.width = 4.0;
            render_arc(
                &painter,
                &center,
                progress_start,
                arc_max,
                28.0 - 4.0,
                &stroke,
            );

            painter.circle_filled(rect.center(), 24.0 - 4.0, visuals.fg_stroke.color);
            painter.circle_filled(rect.center(), 20.0 - 4.0, visuals.bg_fill);
            painter.line_segment(
                [
                    center,
                    Pos2 {
                        x: center.x + (20.0 - 4.0) * (ARC_ROTATION - progress_start).cos(),
                        y: center.y + (20.0 - 4.0) * (ARC_ROTATION - progress_start).sin(),
                    },
                ],
                visuals.fg_stroke,
            );

            if response.dragged() {
                // TODO: Change cursor

                let mdelta = response.drag_delta();
                let delta_points = mdelta.x - mdelta.y;

                let delta_value = delta_points * self.speed;

                if delta_value != 0.0 {
                    let precise_value = ui.data_mut(|data| data.get_temp::<f32>(id));
                    let precise_value = precise_value.unwrap_or(*self.value);
                    let precise_value = precise_value + delta_value;

                    let aim_rad = ui.input(|i| i.aim_radius());

                    let aim_delta = aim_rad * self.speed;
                    let rounded_new_value = emath::smart_aim::best_in_range_f64(
                        (precise_value - aim_delta) as f64,
                        (precise_value + aim_delta) as f64,
                    );
                    let rounded_new_value = emath::round_to_decimals(rounded_new_value, 2) as f32;
                    let rounded_new_value = clamp_value_to_range(rounded_new_value, &self.range);

                    *self.value = rounded_new_value;

                    ui.data_mut(|data| data.insert_temp::<f32>(id, precise_value));
                }
            }
        }

        response.changed = *self.value != old_value;

        response
    }
}
