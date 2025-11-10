mod table_loader;
use rand::{self, Rng};

fn main() {
    let json = table_loader::read_from_file("tables/test_table.json").unwrap();

    let mut rng = rand::rng();
    let die = rng.random_range(1..=6);
    for item in json.items.iter() {
        if item.range.len() == 2 {
            if die >= item.range[0] && die <= item.range[1] {
                println!("{} {}", json.outcome_prefix, item.value);
            }
        } else {
            if die == item.range[0] {
                println!("{} {}", json.outcome_prefix, item.value);
            }
        }
    }
}
