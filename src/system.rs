struct Attributes {
    strength: i32,
    dexterity: i32,
    constitution: i32,
    intelligence: i32,
    wisdom: i32,
    charisma: i32,
}

impl Attributes {
    fn new() -> Attributes {
        Attributes {
            strength: 0,
            dexterity: 0,
            constitution: 0,
            intelligence: 0,
            wisdom: 0,
            charisma: 0,
        }
    }

    fn from_tuple(tuple: (i32, i32, i32, i32, i32, i32)) -> Attributes {
        Attributes {
            strength: tuple.0,
            dexterity: tuple.1,
            constitution: tuple.2,
            intelligence: tuple.3,
            wisdom: tuple.4,
            charisma: tuple.5,
        }
    }

    fn add(&mut self, other: Attributes) {
        self.strength += other.strength;
        self.dexterity += other.dexterity;
        self.constitution += other.constitution;
        self.intelligence += other.intelligence;
        self.wisdom += other.wisdom;
        self.charisma += other.charisma;
    }

    fn add_tuple(&mut self, tuple: (i32, i32, i32, i32, i32, i32)) {
        self.strength += tuple.0;
        self.dexterity += tuple.1;
        self.constitution += tuple.2;
        self.intelligence += tuple.3;
        self.wisdom += tuple.4;
        self.charisma += tuple.5;
    }
}

struct Player {
    name: String,
    level: u32,
    experience: u32,
    health: u32,
    max_health: u32,
    mana: u32,
    max_mana: u32,
    coins: u32,
    inventory: Vec<String>,
    equipped: Vec<String>,
    attributes: Attributes,
    upgrade_points: u32,
}

impl Player {
    fn new() -> Player {
        Player {
            name: String::new(),
            level: 0,
            experience: 0,
            health: 0,
            max_health: 0,
            mana: 0,
            max_mana: 0,
            coins: 0,
            inventory: Vec::new(),
            equipped: Vec::new(),
            attributes: Attributes::new(),
            upgrade_points: 0,
        }
    }

    fn add_attributes(&mut self, attributes: Attributes) {
        self.attributes.add(attributes);
    }
    
    fn add_experience(&mut self, experience: u32) {
        self.experience += experience;
        let level_up_threshold = 10 + (self.level * 5);

        if (self.experience >= level_up_threshold) {
            self.level += 1;
            self.upgrade_points += 2;
            self.experience -= level_up_threshold;
        }
    }
}