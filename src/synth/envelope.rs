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
    release_alpha: f64,
    pub finished: bool,
}

impl ADSR {
    pub fn default() -> Self {
        ADSR {
            attack: 0.0,
            decay: 0.0,
            sustain: 0.0,
            release: 0.0,
            release_alpha: 0.0,
            finished: false,
        }
    }

    pub fn update(&mut self, attack: f32, decay: f32, sustain: f32, release: f32) {
        self.attack = attack.into();
        self.decay = decay.into();
        self.sustain = sustain.into();
        self.release = release.into();
    }
}

impl Envelope for ADSR {
    fn process(&mut self, time: f64, time_off: f64) -> f64 {
        let mut alpha = 0.0;

        if time_off == 0.0 {
            if time < self.attack {
                alpha = time * (1.0 / self.attack);
            } else if time < self.attack + self.decay {
                // this will always be from 1, since we have passed the full attack time
                alpha = 1.0 - (time - self.attack) * ((1.0 - self.sustain) / self.decay);
            } else {
                alpha = self.sustain;
            }
            self.release_alpha = alpha;
        } else {
            // if the key is released before the sustain level has been reached,
            // we should release from the max_alpha, not from the sustain level.
            let sustain = self.release_alpha;

            let time_since_off = time - time_off;
            if time_since_off < self.release {
                alpha = sustain - (time_since_off * (sustain / self.release))
            } else {
                self.finished = true;
            }
        }
        alpha
    }
}
