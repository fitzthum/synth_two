// A voice roughly corresponds to a note

pub struct Voice {
    // this represents the note
    // maybe it should be in a separate struct?
    note: u8,
    velocity: f32,
    time_since_on: f64,
    time_off: f64,
    pub finished: bool,

    // we will also need to store the oscillators for this voice

}

impl Voice {
    pub fn from_midi(note: u8, velocity: f32) -> Self {
        Self {
            note,
            velocity,
            time_since_on: 0.0,
            time_off: 0.0,
            finished: false,
        }
    }

    pub fn voice_off(&mut self) {
        self.time_off = self.time_since_on;

        // change this once we have envelopes
        self.finished = true;
    }
}
