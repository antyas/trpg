#[derive(Clone)]
pub enum Attribute {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
    None,
}

#[derive(Default, Clone)]
pub struct Attributes {
    pub strength: i8,
    pub dexterity: i8,
    pub constitution: i8,
    pub intelligence: i8,
    pub wisdom: i8,
    pub charisma: i8,
}

impl Attributes {
    pub fn from_tuple(tuple: (i8, i8, i8, i8, i8, i8)) -> Attributes {
        Attributes {
            strength: tuple.0,
            dexterity: tuple.1,
            constitution: tuple.2,
            intelligence: tuple.3,
            wisdom: tuple.4,
            charisma: tuple.5,
        }
    }

    pub fn add(&mut self, other: Attributes) {
        self.strength += other.strength;
        self.dexterity += other.dexterity;
        self.constitution += other.constitution;
        self.intelligence += other.intelligence;
        self.wisdom += other.wisdom;
        self.charisma += other.charisma;
    }

    pub fn add_tuple(&mut self, tuple: (i8, i8, i8, i8, i8, i8)) {
        self.strength += tuple.0;
        self.dexterity += tuple.1;
        self.constitution += tuple.2;
        self.intelligence += tuple.3;
        self.wisdom += tuple.4;
        self.charisma += tuple.5;
    }

    pub fn get(&self, attribute: Attribute) -> i8 {
        match attribute {
            Attribute::Strength => self.strength,
            Attribute::Dexterity => self.dexterity,
            Attribute::Constitution => self.constitution,
            Attribute::Intelligence => self.intelligence,
            Attribute::Wisdom => self.wisdom,
            Attribute::Charisma => self.charisma,
            Attribute::None => 0,
        }
    }
}

#[derive(Clone)]
pub enum ItemKind {
    MeleeWeapon,
    RangedWeapon,
    MagicWeapon,
    Ammunition,
    Armor,
    Shield,
    Clothes,
    Ring,
    Amulet,
    Potion,
    Scroll,
    Food,
    Book,
    Recipe,
    Ingredient,
    Material,
    Misc,
}

#[derive(Clone)]
pub enum EffectValue {
    None,
    Health(i8),
    MaxHealth(i8),
    Mana(i8),
    MaxMana(i8),
    Speed(i8),
    Attributes(Attributes),
    Defense(i8),
}

#[derive(Clone)]
pub enum EffectTarget {
    OnSelf,
    Other,
}

#[derive(Clone)]
pub struct Effect {
    pub value: EffectValue,
    pub duration: u16,
    pub is_permanent: bool,
    pub target: EffectTarget,
}

#[derive(Clone)]
pub struct Item {
    pub name: String,
    pub description: String,
    pub price: isize,
    pub kind: ItemKind,
    pub attributes: Attributes,
    pub attribute: Attribute,
    pub effects: Vec<Effect>,
}

#[derive(Default, Clone)]
pub struct Creature {
    pub name: String,
    pub description: String,
    pub creature_name: String, // race
    pub creature_description: String, // race
    pub level: usize,
    pub experience: usize,
    pub health: usize,
    pub max_health: usize, // race
    pub mana: usize,
    pub max_mana: usize, // race
    pub speed: usize, // race
    pub defense: usize, // race
    pub coins: usize,
    pub inventory: Vec<Item>,
    pub equipped: Vec<Item>,
    pub attributes: Attributes, // race
    pub upgrade_points: usize,
}

impl Creature {
    pub fn add_attributes(&mut self, attributes: Attributes) {
        self.attributes.add(attributes);
    }
    
    pub fn add_experience(&mut self, experience: usize) {
        self.experience += experience;
        let level_up_threshold = 10 + (self.level * 5);

        if self.experience >= level_up_threshold {
            self.level += 1;
            self.upgrade_points += 2;
            self.experience -= level_up_threshold;
        }
    }
}