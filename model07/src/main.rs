use std::collections::BTreeMap;

mod solfa {
    pub struct Solfa {
        pitch_offset: u8,
        score_offset: u8
    }

    impl Solfa {
        pub fn pitch_offset(&self) -> u8 {
            return self.pitch_offset;
        }
        pub fn score_offset(&self) -> u8 {
            return self.score_offset;
        }
    }

    pub const C: Solfa = Solfa {pitch_offset: 0, score_offset: 0};
    pub const D: Solfa = Solfa {pitch_offset: 1, score_offset: 2};
    pub const E: Solfa = Solfa {pitch_offset: 2, score_offset: 4};
    pub const F: Solfa = Solfa {pitch_offset: 3, score_offset: 5};
    pub const G: Solfa = Solfa {pitch_offset: 4, score_offset: 7};
    pub const A: Solfa = Solfa {pitch_offset: 5, score_offset: 9};
    pub const B: Solfa = Solfa {pitch_offset: 6, score_offset: 11};
    pub const OCTAVE: u8 = 7;
}

mod sharp_flat {
    pub struct SharpFlat {
        offset: i8
    }
    impl SharpFlat {
        pub fn offset(&self) -> i8 {
            return self.offset;
        }
    }

    pub const SHARP: SharpFlat = SharpFlat {offset: 1};
    pub const DOUBLE_SHARP: SharpFlat = SharpFlat {offset: 2};
    pub const NATURAL: SharpFlat = SharpFlat {offset: 0};
    pub const FLAT: SharpFlat = SharpFlat {offset: -1};
    pub const DOUBLE_FLAT: SharpFlat = SharpFlat {offset: -2};
}

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
    solfa: solfa::Solfa,
    sharp_flat: sharp_flat::SharpFlat,
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

impl<T> Widget for T where T: HasWidget {
    fn is_selected(&self) -> bool {
        return self.widget().is_selected()
    }
    fn layer_no(&self) -> u8 {
        return self.widget().layer_no()
    }
}

struct NoteRepo {
    in_start_tick_order: BTreeMap<u32, Note>,
    in_end_tick_order: BTreeMap<u32, Note>
}

fn main() {
    let noteRepo: NoteRepo = NoteRepo {
        in_start_tick_order: BTreeMap::new(),
        in_end_tick_order: BTreeMap::new()
    };
    noteRepo.in_start_tick_order.insert(
        0, Note {
            start_tick: 0,
            velocity: 0,
            solfa: solfa::C,
            sharp_flat: sharp_flat:: NATURAL,
            widget: WidgetProps {
                is_selected: false,
                layer_no: 0
            }
        }
    );
}
