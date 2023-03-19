// going postal

pub trait Envelope {
    // *time* is the time since note on.
    // *on* is whether or not the note is on
    // *time_off* is the time stamp that the note was turned off
    fn process(&mut self, time: f64, time_off: f64) -> f64;
}

pub struct ADSR {
    attack: f64,
    decay: f64,
    sustain: f64,
    release: f64,
    max_alpha: f64,
    pub finished: bool,
}

impl ADSR {
    pub fn new(attack: f32, decay: f32, sustain: f32, release: f32) -> Self {
        ADSR {
            attack: attack.into(),
            decay: decay.into(),
            sustain: sustain.into(),
            release: release.into(),
            max_alpha: 1.0,
            finished: false,
        }
    }
}

impl Envelope for ADSR {
    fn process(&mut self, time: f64, time_off: f64) -> f64 {
        let mut alpha = 0.0;

        if time_off == 0.0 {
            if time < self.attack {
                alpha = time * (1.0 / self.attack);
                self.max_alpha = alpha;
            } else if time < self.attack + self.decay {
                alpha = 1.0 - (time - self.attack) * ((1.0 - self.sustain) / self.decay);
            } else {
                alpha = self.sustain;
            }
        } else {
            // if the key is released before the sustain level has been reached,
            // we should release from the max_alpha, not from the sustain level.
            let sustain = f64::min(self.max_alpha, self.sustain);

            let time_since_off = time - time_off;
            if time_since_off < self.release {
                alpha = sustain - (time_since_off * (self.sustain / self.release))
            } else {
                self.finished = true;
            }
        }
        alpha
    }
}
