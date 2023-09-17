use bitmaps::Bitmap;

#[derive(Default)]
pub struct Abilities {
    pub(crate) bitmap: Bitmap<11>,
}

#[derive(Default)]
pub struct AbilitiesInit {
    pub(crate) battlecry: bool,
    pub(crate) deathrattle: bool,
    pub(crate) taunt: bool,
    pub(crate) shield: bool,
    //TODO: implement functionality
    pub(crate) windfury: bool,
    pub(crate) venomous: bool,
    //TODO: implement functionality
    pub(crate) reborn: bool,
    //TODO: implement functionality
    pub(crate) avenge: bool,
    //TODO: implement functionality
    pub(crate) magnetic: bool,
    //TODO: implement functionality
    pub(crate) spellcraft: bool,
    pub(crate) stealth: bool,
}

impl AbilitiesInit {
    #[allow(clippy::identity_op)]
    pub fn init(self) -> Abilities {
        Abilities {
            bitmap: Bitmap::from_value(
                (self.battlecry as u16) << 0
                    | (self.deathrattle as u16) << 1
                    | (self.taunt as u16) << 2
                    | (self.shield as u16) << 3
                    | (self.windfury as u16) << 4
                    | (self.venomous as u16) << 5
                    | (self.reborn as u16) << 6
                    | (self.avenge as u16) << 7
                    | (self.magnetic as u16) << 8
                    | (self.spellcraft as u16) << 9
                    | (self.stealth as u16) << 10,
            ),
        }
    }
}

impl Abilities {
    pub fn battlecry(&self) -> bool {
        self.bitmap.get(0)
    }

    pub fn set_battlecry(&mut self, value: bool) {
        self.bitmap.set(0, value);
    }

    pub fn deathrattle(&self) -> bool {
        self.bitmap.get(1)
    }

    pub fn set_deathrattle(&mut self, value: bool) {
        self.bitmap.set(1, value);
    }

    pub fn taunt(&self) -> bool {
        self.bitmap.get(2)
    }

    pub fn set_taunt(&mut self, value: bool) {
        self.bitmap.set(2, value);
    }

    pub fn shield(&self) -> bool {
        self.bitmap.get(3)
    }

    pub fn set_shield(&mut self, value: bool) {
        self.bitmap.set(3, value);
    }

    pub fn windfury(&self) -> bool {
        self.bitmap.get(4)
    }

    pub fn set_windfury(&mut self, value: bool) {
        self.bitmap.set(4, value);
    }

    pub fn venomous(&self) -> bool {
        self.bitmap.get(5)
    }

    pub fn set_venomous(&mut self, value: bool) {
        self.bitmap.set(5, value);
    }

    pub fn reborn(&self) -> bool {
        self.bitmap.get(6)
    }

    pub fn set_reborn(&mut self, value: bool) {
        self.bitmap.set(6, value);
    }

    pub fn avenge(&self) -> bool {
        self.bitmap.get(7)
    }

    pub fn set_avenge(&mut self, value: bool) {
        self.bitmap.set(7, value);
    }

    pub fn magnetic(&self) -> bool {
        self.bitmap.get(8)
    }

    pub fn set_magnetic(&mut self, value: bool) {
        self.bitmap.set(8, value);
    }

    pub fn spellcraft(&self) -> bool {
        self.bitmap.get(9)
    }

    pub fn set_spellcraft(&mut self, value: bool) {
        self.bitmap.set(9, value);
    }

    pub fn stealth(&self) -> bool {
        self.bitmap.get(10)
    }

    pub fn set_stealth(&mut self, value: bool) {
        self.bitmap.set(10, value);
    }
}
