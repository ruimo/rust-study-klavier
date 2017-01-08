struct WidgetProps {
    is_selected: bool,
    layer_no: u8
}

trait Widget {
    fn is_selected(&self) -> bool;
    fn layer_no(&self) -> u8;
}

struct Note {
    start_tick: u32,
    velocity: u8,
    widget: WidgetProps
}

struct Tempo {
    start_tick: u32,
    value: u16,
    widget: WidgetProps
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

impl Widget for Note {
    fn is_selected(&self) -> bool {
        return self.widget.is_selected;
    }
    fn layer_no(&self) -> u8 {
        return self.widget.layer_no;
    }
}

impl Widget for Tempo {
    fn is_selected(&self) -> bool {
        return self.widget.is_selected;
    }
    fn layer_no(&self) -> u8 {
        return self.widget.layer_no;
    }
}

fn main() {
}
