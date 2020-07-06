pub mod terrain_selector {
    use crate::die_roll;
    use crate::Terrain;

    pub fn choose_terrain(terrain_collection: &mut Vec<Terrain>) {
        for _i in 0..terrain_collection.capacity() {
            terrain_collection.push(generate_terrain_piece())
        }
    }

    fn generate_terrain_piece() -> Terrain {
        match die_roll(2, 6) {
            // Match a single value
            1 | 2 | 3 | 4 => {
                return Terrain {
                    name: String::from("Hill"),
                    rule_set: String::from("Blocks arrows"),
                    radius: 2,
                }
            }
            // Match several values
            5 | 6 | 7 | 8 | 9 => {
                return Terrain {
                    name: String::from("Fountain"),
                    rule_set: String::from("Wet"),
                    radius: 1,
                }
            }
            // Match an inclusive range
            10 | 11 | 12 => {
                return Terrain {
                    name: String::from("Magic?"),
                    rule_set: String::from("Magic!"),
                    radius: 1,
                }
            }

            _ => {
                return Terrain::new();
            }
        }
    }
}
