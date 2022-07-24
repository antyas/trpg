use crate::system::*;

pub fn races() -> [Creature; 1] {
    [
        Creature {
            creature_name: String::from("Человек"),
            creature_description: String::from("Описание человека"),
            attributes: Attributes::from_tuple((2, 2, 2, 2, 2, 2)),
            max_health: 10,
            max_mana: 10,
            speed: 3,
            defense: 3,
            ..Creature::default()
        }
    ]
}