struct WidgetProps {
    is_selected: bool,
    layer_no: u8
}

trait Widget {
    fn is_selected(&self) -> bool;
    fn layer_no(&self) -> u8;
}

impl Widget for WidgetProps {
    fn is_selected(&self) -> bool {
        return self.is_selected;
    }
    fn layer_no(&self) -> u8 {
        return self.layer_no;
    }
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

trait HasWidget {
    fn widget(&self) -> &Widget;
}

impl HasWidget for Note {
    fn widget(&self) -> &Widget {
        return &self.widget;
    }
}

impl HasWidget for Tempo {
    fn widget(&self) -> &Widget {
        return &self.widget;
    }
}

fn main() {
}
