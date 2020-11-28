
#[derive(Default)]
pub struct CombatLog {
    pub logs: Vec<String>,
}

const MAX_LOGS: usize = 10;

impl CombatLog {
    pub fn add(&mut self, s: String) {
        self.logs.push(s);
        if self.logs.len() > MAX_LOGS {
            self.logs.remove(0);
        }
    }
}