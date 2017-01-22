pub mod model {
    pub mod sharp_flat {
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

    pub mod solfa {
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

    pub trait Model {
        fn start_tick(&self) -> u32;
    }

    pub struct Note {
        start_tick: u32,
        velocity: u8,
        solfa: solfa::Solfa,
        sharp_flat: sharp_flat::SharpFlat
    }

    impl Note {
        pub fn new(
            start_tick: u32,
            velocity: u8,
            solfa: solfa::Solfa,
            sharp_flat: sharp_flat::SharpFlat
        ) -> Note {
            Note {
                start_tick: start_tick,
                velocity: velocity,
                solfa: solfa,
                sharp_flat: sharp_flat
            }
        }
    }

    impl Model for Note {
        fn start_tick(&self) -> u32 {
            return self.start_tick;
        }
    }

    pub struct Tempo {
        start_tick: u32,
        value: u16
    }

    impl Model for Tempo {
        fn start_tick(&self) -> u32 {
            return self.start_tick;
        }
    }

    impl Note {
        pub fn start_tick(&self) -> u32 {
            return self.start_tick;
        }
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

    trait HasModel {
        fn model(&self) -> &Self;
    }

    impl HasModel for Note {
        fn model(&self) -> &Self {
            return &self.model();
        }
    }

    impl HasModel for Tempo {
        fn model(&self) -> &Self {
            return &self.model();
        }
    }
}

pub mod view {
    use model::Note;
    use model::Tempo;

    pub struct ViewProps {
        is_selected: bool,
        layer_no: u8
    }

    impl ViewProps {
        pub fn new(is_selected: bool, layer_no: u8) -> ViewProps {
            return ViewProps {
                is_selected: is_selected,
                layer_no: layer_no
            };
        }
    }

    pub trait View {
        fn is_selected(&self) -> bool;
        fn layer_no(&self) -> u8;
    }

    impl View for ViewProps {
        fn is_selected(&self) -> bool {
            return self.is_selected;
        }
        fn layer_no(&self) -> u8 {
            return self.layer_no;
        }
    }

    pub trait HasView {
        fn view(&self) -> &View;
    }
}

pub mod modelview {
    use view::HasView;
    use view::View;
    use view::ViewProps;
    use model::Model;
    use model::Tempo;
    use model::Note;

    pub struct ModelView<T: Model> {
        model: T,
        view: ViewProps
    }

    impl<T> ModelView<T> where T: Model {
        pub fn new(model: T, view: ViewProps) -> ModelView<T> {
            ModelView::<T> {
                model: model, view: view
            }
        }
        pub fn model(&self) -> &T {
            return &self.model;
        }
    }

    impl<T: Model> HasView for ModelView<T> {
        fn view(&self) -> &View {
            return &self.view;
        }
    }

    impl<T> View for T where T: HasView {
        fn is_selected(&self) -> bool {
            return self.view().is_selected()
        }
        fn layer_no(&self) -> u8 {
            return self.view().layer_no()
        }
    }
}

pub mod repo {
    use std::collections::BTreeMap;
    use std::rc::Rc;
    use model::Note;
    use model::Tempo;
    use modelview::ModelView;

    struct NoteRepo {
        in_start_tick_order: BTreeMap<u32, Rc<ModelView<Note>>>,
        in_end_tick_order: BTreeMap<u32, Rc<ModelView<Note>>>
    }

    struct TempoRepo {
        in_start_tick_order: BTreeMap<u32, Rc<ModelView<Tempo>>>,
        in_end_tick_order: BTreeMap<u32, Rc<ModelView<Tempo>>>
    }

    pub struct Repo {
        noteRepo: NoteRepo,
        tempoRepo: TempoRepo
    }

    impl Repo {
        pub fn new() -> Repo {
            return Repo {
                noteRepo: NoteRepo {
                    in_start_tick_order: BTreeMap::new(),
                    in_end_tick_order: BTreeMap::new()
                },
                tempoRepo: TempoRepo {
                    in_start_tick_order: BTreeMap::new(),
                    in_end_tick_order: BTreeMap::new()
                }
            };
        }

        pub fn addNote(&mut self, nv: ModelView<Note>) {
            let start_tick = nv.model().start_tick();
            let rc = Rc::new(nv);
            self.noteRepo.in_start_tick_order.insert(start_tick, rc.clone());
            self.noteRepo.in_end_tick_order.insert(!(start_tick), rc);
        }
    }
}

fn main() {
    let note: modelview::ModelView<model::Note> = modelview::ModelView::<model::Note>::new(
        model::Note::new(
            0,
            0,
            model::solfa::C,
            model::sharp_flat::NATURAL
        ),
        view::ViewProps::new(
            false, 0
        )
    );
    let mut repo = repo::Repo::new();
    repo.addNote(note);
}
