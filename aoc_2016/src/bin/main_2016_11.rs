use itertools::Itertools;
use std::{cmp::Ordering, collections::HashSet};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, PartialOrd, Ord)]
enum Type {
    Generator,
    Microchip,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, PartialOrd, Ord)]
enum Element {
    Curium,
    Dilithium,
    Elerium,
    Plutonium,
    Ruthenium,
    Strontium,
    Thulium,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Object {
    obj_type: Type,
    element: Element,
}

impl Ord for Object {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.obj_type.cmp(&other.obj_type) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.element.cmp(&other.element),
        }
    }
}

impl PartialOrd for Object {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Building {
    elevator_floor: u8,
    floors: [Vec<Object>; 4],
}

impl Building {
    fn get_neighbours(&self) -> Vec<u8> {
        match self.elevator_floor {
            0 => vec![1],
            1 => vec![0, 2],
            2 => vec![1, 3],
            3 => vec![2],
            _ => Vec::new(),
        }
    }

    fn is_valid_state(&self) -> bool {
        Building::check_floor(&self.floors[0])
            && Building::check_floor(&self.floors[1])
            && Building::check_floor(&self.floors[2])
            && Building::check_floor(&self.floors[3])
    }

    fn check_floor(floor: &[Object]) -> bool {
        let generators: Vec<Element> = floor
            .iter()
            .filter_map(|&obj| {
                if obj.obj_type == Type::Generator {
                    Some(obj.element)
                } else {
                    None
                }
            })
            .collect();

        if generators.is_empty() {
            return true;
        }

        let chips: Vec<Element> = floor
            .iter()
            .filter_map(|&obj| {
                if obj.obj_type == Type::Microchip {
                    Some(obj.element)
                } else {
                    None
                }
            })
            .collect();

        chips.iter().all(|el| generators.contains(el))
    }

    fn get_elements(&self) -> Vec<Vec<Object>> {
        self.floors[self.elevator_floor as usize]
            .iter()
            .map(|&obj| vec![obj])
            .chain(
                self.floors[self.elevator_floor as usize]
                    .clone()
                    .into_iter()
                    .combinations(2),
            )
            .collect()
    }

    fn sort_floors(&mut self) {
        (0..4).for_each(|index| self.floors[index].sort());
    }

    fn move_comb_els(&self, neighbour: u8, comb_els: Vec<Object>) -> Self {
        let mut new_floors: [Vec<Object>; 4] = self.floors.clone();

        comb_els.iter().for_each(|&new_el| {
            new_floors[neighbour as usize].push(new_el);
            new_floors[neighbour as usize].sort();

            let checked_el: Vec<Object> = new_floors[self.elevator_floor as usize]
                .iter()
                .filter(|&old_el| *old_el != new_el)
                .cloned()
                .collect();
            new_floors[self.elevator_floor as usize] = checked_el;
        });

        let mut new_building: Building = Building {
            elevator_floor: neighbour,
            floors: new_floors,
        };

        new_building.sort_floors();
        new_building
    }

    fn move_all(&self) -> Vec<Self> {
        let neighbours: Vec<u8> = self.get_neighbours();
        let all_comb_els: Vec<Vec<Object>> = self.get_elements();

        neighbours
            .iter()
            .flat_map(|&neighbour| {
                all_comb_els
                    .iter()
                    .map(|comb_els| self.move_comb_els(neighbour, comb_els.to_vec()))
                    .filter(|building| building.is_valid_state())
                    .collect::<Vec<Building>>()
            })
            .collect()
    }
}

fn main() {
    let cur_gen: Object = Object {
        obj_type: Type::Generator,
        element: Element::Curium,
    };
    let ele_gen: Object = Object {
        obj_type: Type::Generator,
        element: Element::Elerium,
    };
    let dil_gen: Object = Object {
        obj_type: Type::Generator,
        element: Element::Dilithium,
    };
    let plu_gen: Object = Object {
        obj_type: Type::Generator,
        element: Element::Plutonium,
    };
    let rut_gen: Object = Object {
        obj_type: Type::Generator,
        element: Element::Ruthenium,
    };
    let str_gen: Object = Object {
        obj_type: Type::Generator,
        element: Element::Strontium,
    };
    let thu_gen: Object = Object {
        obj_type: Type::Generator,
        element: Element::Thulium,
    };

    let cur_chip: Object = Object {
        obj_type: Type::Microchip,
        element: Element::Curium,
    };
    let ele_chip: Object = Object {
        obj_type: Type::Microchip,
        element: Element::Elerium,
    };
    let dil_chip: Object = Object {
        obj_type: Type::Microchip,
        element: Element::Dilithium,
    };
    let plu_chip: Object = Object {
        obj_type: Type::Microchip,
        element: Element::Plutonium,
    };
    let rut_chip: Object = Object {
        obj_type: Type::Microchip,
        element: Element::Ruthenium,
    };
    let str_chip: Object = Object {
        obj_type: Type::Microchip,
        element: Element::Strontium,
    };
    let thu_chip: Object = Object {
        obj_type: Type::Microchip,
        element: Element::Thulium,
    };

    // first part
    let mut start_building: Building = Building {
        elevator_floor: 0,
        floors: [
            vec![str_gen, str_chip, plu_gen, plu_chip],
            vec![thu_gen, rut_gen, rut_chip, cur_gen, cur_chip],
            vec![thu_chip],
            vec![],
        ],
    };

    start_building.sort_floors();
    let mut all_buildings = vec![start_building.clone()];
    let mut steps: u64 = 0;
    let mut final_dest: bool = false;
    let mut all_steps: HashSet<Building> = HashSet::new();
    all_steps.insert(start_building);

    loop {
        all_buildings = all_buildings
            .iter_mut()
            .flat_map(|new_building| new_building.move_all())
            .filter(|new_building| all_steps.insert(new_building.clone()))
            .collect();
        steps += 1;

        all_buildings.iter().for_each(|new_building| {
            if new_building.floors[3].len() == 10 {
                final_dest = true;
            }
        });

        if final_dest {
            println!(
                "{steps} is the minimum of steps to bring all the objects to the fourth floor"
            );
            break;
        }
    }

    // second part
    let mut new_start_building: Building = Building {
        elevator_floor: 0,
        floors: [
            vec![
                str_gen, str_chip, plu_gen, plu_chip, ele_gen, ele_chip, dil_gen, dil_chip,
            ],
            vec![thu_gen, rut_gen, rut_chip, cur_gen, cur_chip],
            vec![thu_chip],
            vec![],
        ],
    };

    new_start_building.sort_floors();
    all_buildings = vec![new_start_building.clone()];
    steps = 0;
    final_dest = false;
    all_steps.clear();
    all_steps.insert(new_start_building);

    loop {
        all_buildings = all_buildings
            .iter_mut()
            .flat_map(|new_building| new_building.move_all())
            .filter(|new_building| all_steps.insert(new_building.clone()))
            .collect();
        steps += 1;

        all_buildings.iter().for_each(|new_building| {
            if new_building.floors[3].len() == 14 {
                final_dest = true;
            }
        });

        if final_dest {
            println!(
                "{steps} is the minimum of steps to bring all the objects to the fourth floor"
            );
            break;
        }
    }
}
