#[derive(Clone, Copy, Debug)]
pub enum Modes {
    Manual(SMonitors),
    Track
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SMonitors {
    HDMI,
    Main
}

impl SMonitors {
    pub fn new() -> Self {
        SMonitors::Main
    }

    fn switch(&mut self) {
        let newv = match self {
            SMonitors::Main => SMonitors::HDMI,
            SMonitors::HDMI => SMonitors::Main
        };
        println!("{newv:?}");
        *self = newv
    }
}


impl Modes {
    pub fn new(sm: &mut SMonitors) -> Self {
        Modes::Manual(*sm)
    }
}

impl Modes {
    pub fn eq_mode(self, tocmp: String) -> bool {
        if self.cmp_mode(tocmp) {
            return true
        }
        false
    }
    pub fn eq_mon(self, tocmp: String) -> bool {
        if self.cmp_mon(tocmp) {
            return true
        }
        false
    }

    pub fn switch_mode(&mut self) {
        match self {
            Modes::Manual(_) => Modes::Track,
            Modes::Track => Modes::Manual(SMonitors::Main)
        };
    }

    pub fn switch_mon(&mut self) {
        match self {
            Modes::Manual(sm) => {
                sm.switch()
            },
            _ => {}
        }  
    }

    fn cmp_mode(self,tocmp: String) -> bool {
        if tocmp.contains("manual") {
            return match self {
                Modes::Track => false,
                Modes::Manual(_) => true
            };
        }
        if tocmp.contains("track") {
            return match self {
                Modes::Track => true,
                Modes::Manual(_) => false
            };
        }
        false
    }

    fn cmp_mon(self,tocmp: String) -> bool {
        if tocmp.contains("hdmi") {
            return match self {
                Modes::Manual(m) => {
                    if m == SMonitors::HDMI {
                        true
                    } else {
                        false
                    }
                },
                _ => false
            };
        }
        if tocmp.contains("main") {
            return match self {
                Modes::Manual(m) => {
                    if m == SMonitors::Main{
                        true
                    } else {
                        false
                    }
                },
                _ => false
            };
        }
        false
    }

}
