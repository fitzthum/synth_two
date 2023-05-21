use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::vizia::vg;

use std::sync::{Arc, Mutex};

use crate::params::ENVELOPE_TIME_MAX;

pub struct EnvelopeGraph {
    envelope: Arc<Mutex<Vec<f32>>>,
}

impl EnvelopeGraph {
    pub fn new<LVec>(cx: &mut Context, envelope: LVec) -> Handle<Self>
    where
        LVec: Lens<Target = Arc<Mutex<Vec<f32>>>>,
    {
        Self {
            envelope: envelope.get(cx),
        }
        .build(cx, |_cx| ())
    }
}

impl View for EnvelopeGraph {
    fn element(&self) -> Option<&'static str> {
        Some("envelope-graph")
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        let bounds = cx.bounds();

        if bounds.w == 0.0 || bounds.h == 0.0 {
            return;
        }

        let env = self.envelope.lock().unwrap();
        let a = env[0];
        let d = env[1];
        let s = env[2];
        let r = env[3];

        let line_width = cx.style.dpi_factor as f32 * 1.5;
        let paint = vg::Paint::color(cx.font_color().cloned().unwrap_or_default().into())
            .with_line_width(line_width);

        let mut path = vg::Path::new();

        // x,y is top left
        // start in the bottom right
        path.move_to(bounds.x, bounds.y + bounds.h);

        // each section gets the equivalent of 5 seconds at most
        // so we split the box into 4
        // even though that doesn't really make sense for the sustain part
        let section_width = bounds.w / 4.0;

        // the attack line will go to the top of the box
        // and some horizontal offset
        let a_offset = (a / ENVELOPE_TIME_MAX) * section_width;
        path.line_to(bounds.x + a_offset, bounds.y);

        // now go over by some delay offset
        // factor in the attack offset because we've already moved that far
        let d_offset_horizontal = a_offset + (d / ENVELOPE_TIME_MAX) * section_width;
        let d_offset_vertical = bounds.h * s;
        path.line_to(bounds.x + d_offset_horizontal, bounds.y + d_offset_vertical);

        // now just draw a straight line at the sustain level
        path.line_to(
            bounds.x + d_offset_horizontal + section_width,
            bounds.y + d_offset_vertical,
        );

        // now add on the release time and go to the bottom
        let r_offset = d_offset_horizontal + section_width + (r / ENVELOPE_TIME_MAX) * section_width;
        path.line_to(bounds.x + r_offset, bounds.y + bounds.h);

        canvas.stroke_path(&mut path, &paint);
    }
}
