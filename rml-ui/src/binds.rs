#[derive(Debug, Clone)]
pub struct Bind {
    pub CTRL: bool,
    pub ALT: bool,
    pub SHIFT: bool,
    pub CAPSLOCK: bool,

    pub symbol: char
}

macro_rules! match_key {
    ($key:expr, $e:expr, $copy:expr) => {
        match $key {
            Keys::CTRL => $copy.CTRL = $e,
            Keys::SHIFT => $copy.SHIFT = $e,
            Keys::ALT => $copy.ALT = $e,
            Keys::CAPSLOCK => $copy.CAPSLOCK = $e,
        }
    };
}

pub enum Keys {
    CTRL,
    SHIFT,
    ALT,
    CAPSLOCK,
}
impl Bind {
    pub fn new(symbol: char) -> Bind {
        Bind {
            CTRL: false,
            SHIFT: false,
            ALT: false,
            CAPSLOCK: false,
            symbol: symbol
        }
    }
    pub fn activate(&self, key: Keys) ->  Bind {
        let mut copy: Bind = self.clone();
        match_key!(key, true, copy);
        copy
    }
    pub fn deactivate(&self, key: Keys) ->  Bind {
        let mut copy = self.clone();
        match_key!(key, false, copy);

        copy
    }
}

impl Default for Bind {
    fn default() -> Self {
        Self { CTRL: Default::default(), ALT: Default::default(), SHIFT: Default::default(), CAPSLOCK: Default::default(), symbol: 'c' }
    }
}


impl ToString for Bind {
    fn to_string(&self) -> String {
        let mut s = String::new();

        s.push(unsafe { char::from_u32_unchecked(self.CTRL as u32) });
        s.push(unsafe { char::from_u32_unchecked(self.SHIFT as u32) });
        s.push(unsafe { char::from_u32_unchecked(self.ALT as u32) });
        s.push(unsafe { char::from_u32_unchecked(self.CAPSLOCK as u32) });
        s.push(self.symbol);
        
        s
    }
}

