use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::vizia::vg;

use std::sync::{Arc, Mutex};

pub struct WaveGraph {
    samples: Arc<Mutex<Vec<f32>>>,
}

impl WaveGraph {
    pub fn new<LVec>(cx: &mut Context, samples: LVec) -> Handle<Self>
    where
        LVec: Lens<Target = Arc<Mutex<Vec<f32>>>>,
    {
        Self {
            samples: samples.get(cx),
        }
        .build(cx, |_cx| ())
    }
}

impl View for WaveGraph {
    fn element(&self) -> Option<&'static str> {
        Some("wave-graph")
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        let bounds = cx.bounds();

        if bounds.w == 0.0 || bounds.h == 0.0 {
            return;
        }

        let amplitude = bounds.h / 2.0;
        let middle_offset = bounds.y + amplitude;

        let samples = self.samples.lock().unwrap();

        if samples.len() == 0 {
            return;
        }

        let line_width = cx.style.dpi_factor as f32 * 1.5;
        let paint = vg::Paint::color(cx.font_color().cloned().unwrap_or_default().into())
            .with_line_width(line_width);

        let mut path = vg::Path::new();

        let sample_width = bounds.w / samples.len() as f32;

        // x,y is top left
        // start with the first sample
        path.move_to(bounds.x, middle_offset - (samples[0] * amplitude));

        for n in 1..samples.len() {
            let x_offset = sample_width * n as f32; 
            let y_offset = amplitude * samples[n]; 

            path.line_to(bounds.x + x_offset, middle_offset - y_offset);
        }

        canvas.stroke_path(&mut path, &paint);
    }
}
