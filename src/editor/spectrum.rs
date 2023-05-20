use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::vizia::vg;

use std::sync::{Arc, Mutex};

// idk...
const SCALE_FACTOR: f32 = 10.0;

pub struct SpectrumGraph {
    bins: Arc<Mutex<Vec<f32>>>,
}

impl SpectrumGraph {
    pub fn new<LVec>(cx: &mut Context, bins: LVec) -> Handle<Self>
    where
        LVec: Lens<Target = Arc<Mutex<Vec<f32>>>>,
    {
        Self {
            bins: bins.get(cx),
        }
        .build(cx, |_cx| ())
    }
}

impl View for SpectrumGraph {
    fn element(&self) -> Option<&'static str> {
        Some("wave-graph")
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        let bounds = cx.bounds();

        if bounds.w == 0.0 || bounds.h == 0.0 {
            return;
        }

        let bins = self.bins.lock().unwrap();

        let line_width = cx.style.dpi_factor as f32 * 1.5;
        let paint = vg::Paint::color(cx.font_color().cloned().unwrap_or_default().into())
            .with_line_width(line_width);

        let mut path = vg::Path::new();

        let w = bounds.w / bins.len() as f32;

        // x,y is top left
        // start in the bottom left
        //path.move_to(bounds.x, bounds.y + bounds.h);

        for n in 0..bins.len() {
            // draw rect from upper left
            let h = (SCALE_FACTOR * bounds.h * bins[n]).min(bounds.h);
            let x = bounds.x + w * n as f32; 
            let y = bounds.y + bounds.h - h;

            path.rect(x,y,w,h);
        }

        canvas.stroke_path(&mut path, &paint);
    }
}
