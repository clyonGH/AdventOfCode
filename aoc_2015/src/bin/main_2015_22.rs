use std::fs::File;
use std::io::prelude::*;

const DEBUG: bool = false;
const PART_2: bool = true;

const LOWEST_SPELL_COST: i16 = 53;
const MAX_TOTAL_SPELL: i16 = 1400;

const NO_EFFECT: Effect = Effect {
    turns: 0,
    damage: 0,
    armor: 0,
    mana: 0,
};

const SHIELD_EFFECT: Effect = Effect {
    turns: 6,
    damage: 0,
    armor: 7,
    mana: 0,
};

const POISON_EFFECT: Effect = Effect {
    turns: 6,
    damage: 3,
    armor: 0,
    mana: 0,
};

const RECHARGE_EFFECT: Effect = Effect {
    turns: 5,
    damage: 0,
    armor: 0,
    mana: 101,
};

#[derive(Clone, Debug)]
struct Character {
    hit_points: i16,
    damage: i16,
    armor: i16,
    mana: i16,
    spells: [Spell; 5],
}

#[derive(Clone, Debug)]
struct Effect {
    turns: i16,
    damage: i16,
    armor: i16,
    mana: i16,
}

#[derive(Clone, Debug)]
struct Spell {
    name: String,
    mana: i16,
    active: bool,
    damage: i16,
    heal: i16,
    effect: Effect,
}

fn main() {
    let mut file = File::open("inputs/input_2015_22.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let boss_data: Vec<i16> = data
        .lines()
        .map(|line| {
            let stat: Vec<&str> = line.split(": ").collect();
            stat[1].parse::<i16>().unwrap()
        })
        .collect();

    let boss: Character = Character {
        hit_points: boss_data[0],
        damage: boss_data[1],
        armor: 0,
        mana: 0,
        spells: set_spells(),
    };

    let player: Character = Character {
        hit_points: 50,
        damage: 0,
        armor: 0,
        mana: 500,
        spells: set_spells(),
    };

    let all_spells: [Spell; 5] = set_spells();
    let used_spell: Vec<String> = Vec::new();

    let result_found = cast_and_fight(&player, &boss, &all_spells, 0, used_spell).unwrap();
    println!(
        "{:?} is the least amount of mana you can spend and still win the fight\n{:?}",
        result_found.0, result_found.1
    );
}

// cast spell and fight boss
fn cast_and_fight(
    player: &Character,
    boss: &Character,
    spells: &[Spell],
    mana_spent: i16,
    used_spells: Vec<String>,
) -> Option<(i16, Vec<String>)> {
    if boss.hit_points <= 0 {
        return Some((mana_spent, used_spells));
    } else if player.hit_points <= 0
        || player.mana < LOWEST_SPELL_COST
        || mana_spent > MAX_TOTAL_SPELL
    {
        return None;
    } else {
        spells
            .iter()
            .filter(|&spell| !spell_already_active(spell, &player) && player.mana >= spell.mana)
            .flat_map(|spell| {
                let (updated_player, boss_hp) = fight_boss(player, boss, &spell);
                let mut updated_boss = boss.clone();
                updated_boss.hit_points = boss_hp;
                let mut used_spells_clone = used_spells.clone();
                used_spells_clone.push(spell.name.clone());
                if DEBUG {
                    println!("{:?}", used_spells_clone);
                }
                cast_and_fight(
                    &updated_player,
                    &updated_boss,
                    spells,
                    mana_spent + spell.mana,
                    used_spells_clone,
                )
            })
            .min_by(|a, b| a.0.cmp(&b.0))
    }
}

// returning updated player stats and updated boss hp
fn fight_boss(player: &Character, boss: &Character, spell: &Spell) -> (Character, i16) {
    let mut boss_hp = boss.hit_points;
    let mut updated_player: Character = player.clone();

    // player's turn
    updated_player = cast_spell(spell, updated_player);

    // player damage
    boss_hp = boss_hp - updated_player.damage;

    if boss_hp <= 0 {
        return (updated_player, boss_hp);
    }

    // boss's turn
    // second part: player loses 1 hp at the beginning of each turn
    if PART_2 {
        updated_player.hit_points -= 1;
        if updated_player.hit_points <= 0 {
            return (updated_player, boss_hp);
        }
    }

    updated_player = spells_effects(updated_player);

    // spell effects (poison)
    let poison_activity = updated_player.clone().spells[3].active;
    if poison_activity {
        boss_hp = boss_hp - updated_player.damage;

        if boss_hp <= 0 {
            return (updated_player, boss_hp);
        }
    }

    // boss damage
    if boss.damage > updated_player.armor {
        updated_player.hit_points = updated_player.hit_points + updated_player.armor - boss.damage;
    } else {
        updated_player.hit_points = updated_player.hit_points - 1;
    }

    // second part: player loses 1 hp at the beginning of each turn
    if PART_2 {
        updated_player.hit_points -= 1;
        if updated_player.hit_points <= 0 {
            return (updated_player, boss_hp);
        }
    }

    updated_player = spells_effects(updated_player);

    return (updated_player, boss_hp);
}

fn spell_already_active(spell: &Spell, player: &Character) -> bool {
    player
        .spells
        .as_ref()
        .iter()
        .any(|player_spell| spell.name == player_spell.name && player_spell.active)
}

fn cast_spell(spell: &Spell, mut player: Character) -> Character {
    // casting a new spell
    player.mana -= spell.mana;

    if DEBUG {
        println!("\n{:?} time!", spell.name);
    }
    match spell.name.as_str() {
        "Magic Missile" => {
            player.damage += spell.damage;
        }
        "Drain" => {
            player.damage += spell.damage;
            player.hit_points += spell.heal;
        }
        "Shield" => {
            player.spells[2].active = true;
        }
        "Poison" => {
            player.spells[3].active = true;
        }
        "Recharge" => {
            player.spells[4].active = true;
        }
        _ => (),
    }

    player
}

fn spells_effects(mut player: Character) -> Character {
    player.armor = 0;
    player.damage = 0;

    for spell in &mut player.spells {
        if spell.active {
            if spell.effect.turns > 0 {
                match spell.name.as_str() {
                    "Shield" => {
                        player.armor += SHIELD_EFFECT.armor;
                    }
                    "Poison" => {
                        player.damage += POISON_EFFECT.damage;
                    }
                    "Recharge" => {
                        player.mana += RECHARGE_EFFECT.mana;
                    }
                    _ => (),
                }

                spell.effect.turns -= 1;
            }

            if spell.effect.turns == 0 {
                if DEBUG {
                    println!("{:?} ended...", spell.name);
                }
                spell.active = false;
                match spell.name.as_str() {
                    "Shield" => {
                        spell.effect = SHIELD_EFFECT;
                    }
                    "Poison" => {
                        spell.effect = POISON_EFFECT;
                    }
                    "Recharge" => {
                        spell.effect = RECHARGE_EFFECT;
                    }
                    _ => (),
                }
            }
        }
    }

    player
}

fn set_spells() -> [Spell; 5] {
    [
        Spell {
            name: "Magic Missile".to_string(),
            mana: 53,
            active: false,
            damage: 4,
            heal: 0,
            effect: NO_EFFECT,
        },
        Spell {
            name: "Drain".to_string(),
            mana: 73,
            active: false,
            damage: 2,
            heal: 2,
            effect: NO_EFFECT,
        },
        Spell {
            name: "Shield".to_string(),
            mana: 113,
            active: false,
            damage: 0,
            heal: 0,
            effect: SHIELD_EFFECT,
        },
        Spell {
            name: "Poison".to_string(),
            mana: 173,
            active: false,
            damage: 0,
            heal: 0,
            effect: POISON_EFFECT,
        },
        Spell {
            name: "Recharge".to_string(),
            mana: 229,
            active: false,
            damage: 0,
            heal: 0,
            effect: RECHARGE_EFFECT,
        },
    ]
}
