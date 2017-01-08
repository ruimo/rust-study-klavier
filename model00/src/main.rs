struct Note {
    start_tick: u32,
    velocity: u8
}

struct Tempo {
    start_tick: u32,
    value: u16
}

trait HasStartTick {
    fn start_tick(&self) -> u32;
}

impl HasStartTick for Note {
    fn start_tick(&self) -> u32 {
        return self.start_tick;
    }
}

impl HasStartTick for Tempo {
    fn start_tick(&self) -> u32 {
        return self.start_tick;
    }
}

fn main() {
}
