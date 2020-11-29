#[derive(Debug, Copy, Clone)]
pub enum Event {
    // Draft a die by its position in available_dice of the DicePool
    DraftDie(usize),
}

// global event queue
// RW: in future, it might make sense to separate this out into separate queues based on different event types?
#[derive(Default)]
pub struct EventQueue {
    pub events: Vec<Event>, // possibly read many times by different systems
    pub new_events: Vec<Event>, // possibly written many times by different systems
                            // at end of each frame, `events` is cleared and `new_events` replaces it
}
