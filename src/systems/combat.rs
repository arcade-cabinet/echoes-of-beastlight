```rust
use bevy::prelude::*;
use rand::Rng;

struct Character {
    hp: i32,
    attack: i32,
    defense: i32,
    crit_chance: f32,
    status: Status,
}

enum Status {
    Normal,
    Poisoned,
    Stunned,
}

struct AttackEvent {
    attacker: Entity,
    defender: Entity,
}

fn main() {
    App::build()
        .add_event::<AttackEvent>()
        .add_startup_system(setup.system())
        .add_system(attack.system())
        .add_system(calculate_damage.system())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn().insert(Character {
        hp: 100,
        attack: 10,
        defense: 5,
        crit_chance: 0.1,
        status: Status::Normal,
    });
    commands.spawn().insert(Character {
        hp: 50,
        attack: 8,
        defense: 2,
        crit_chance: 0.2,
        status: Status::Normal,
    });
}

fn attack(mut event_writer: EventWriter<AttackEvent>) {
    let attacker = Entity::new(0);
    let defender = Entity::new(1);
    event_writer.send(AttackEvent { attacker, defender });
}

fn calculate_damage(
    mut event_reader: EventReader<AttackEvent>,
    mut characters: Query<&mut Character>,
) {
    for event in event_reader.iter() {
        let mut attacker = characters.get_mut(event.attacker).unwrap();
        let mut defender = characters.get_mut(event.defender).unwrap();

        let mut damage = attacker.attack - defender.defense;
        if damage < 0 {
            damage = 0;
        }

        let mut rng = rand::thread_rng();
        if rng.gen::<f32>() < attacker.crit_chance {
            damage *= 2;
        }

        match defender.status {
            Status::Poisoned => damage += 5,
            Status::Stunned => damage += 10,
            _ => (),
        }

        defender.hp -= damage;
    }
}
```

This code sets up a basic turn-based combat system for a JRPG using the Bevy ECS game engine. It creates two characters with different stats and a system to handle attack events. The damage calculation system takes into account the attacker's attack stat, the defender's defense stat, the attacker's critical hit chance, and the defender's status effect. The damage is then applied to the defender's HP.