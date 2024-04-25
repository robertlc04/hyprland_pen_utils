#[derive(Clone, Copy)]
pub enum Modes {
    Manual,
    Track
}

impl Modes {
    pub fn new() -> Self {
        Modes::Manual
    }
}

impl Modes {
    pub fn eq(self, tocmp: String) -> bool {
        if self.cmp(tocmp) {
            return true
        }
        false
    }
    pub fn change(self) {
        
    }

    fn cmp(self,tocmp: String) -> bool {
        if tocmp.contains("manual") {
            return match self {
                Modes::Track => false,
                Modes::Manual => true
            };
        }
        if tocmp.contains("track") {
            return match self {
                Modes::Track => true,
                Modes::Manual => false
            };
        }
        false
    }
}
