#[derive(Clone, Debug)]
pub struct Hero {
    health: u16,
    armor: u16,
}

impl Hero {
    pub fn new(health: u16, armor: u16) -> Self {
        Self {
            health,
            armor,
        }
    }
}

impl Default for Hero {
    fn default() -> Self {
        Self {
            health: 30,
            armor: 0,
        }
    }
}
