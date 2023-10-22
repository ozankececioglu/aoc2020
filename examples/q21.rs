use std::collections::{HashMap, HashSet};
use std::f32::consts::E;
use std::str::CharIndices;
use std::{fs::File};
use std::io::{self, prelude::*, BufReader};
use std::rc::Rc;
use std::cell::RefCell;

use itertools::Itertools;


fn main() -> io::Result<()> {
    let file = File::open("data/q21")?;
    let reader = BufReader::new(file);
    
    let mut founds: Vec<(Rc<String>, Rc<String>)> = Vec::new();
    let mut all_intregidents: HashMap<Rc<String>, i32> = HashMap::new();
    let mut all_allergens: HashSet<Rc<String>> = HashSet::new();
    let mut allergen_map: HashMap<Rc<String>, (u32, HashMap<Rc<String>, u32>)> = HashMap::new();
    reader.lines()
        .map(|l| l.unwrap())
        .for_each(|l| {
            let mut fields = l.trim_end_matches(')').split(" (contains ");
            let ingredients = fields.next().unwrap().split(' ')
                .map(|s| Rc::new(String::from(s.trim()))).collect::<Vec<_>>();
            ingredients.iter().for_each(|x| {
                let mut e = all_intregidents.entry(x.clone()).or_insert(0);
                *e += 1;
            });
            
            let allergens = fields.next().unwrap().split(",")
                .map(|s| Rc::new(String::from(s.trim()))).collect::<Vec<_>>();
            for allergen in allergens.iter() {
                let entry = allergen_map.entry(allergen.clone()).or_insert((0, HashMap::new()));
                for ingredient in ingredients.iter() {
                    let count = entry.1.entry(ingredient.clone()).or_insert(0);
                    *count += 1;
                }
                entry.0 += 1;
                all_allergens.insert(allergen.clone());
            }
        });
    
    // println!("all_intregidents: {:?}", all_intregidents.len());
    // println!("allergen_map: {:?}", allergen_map.len());
    // println!("allergen_map: {:#?}", allergen_map.keys());
    // println!("before: {}", all_intregidents.values().sum::<i32>());

    loop {
        println!("round starts!");
        let mut change = false;
        for (allergen, (count, intregidents)) in allergen_map.iter() {
            if !all_allergens.contains(allergen) {
                continue;
            }
            let mut candidate: Option<Rc<String>> = None;
            for (ingredient, ingredient_count) in intregidents.iter() {
                if *ingredient_count == *count && all_intregidents.contains_key(ingredient) {
                    if candidate.is_none() {
                        candidate = Some(ingredient.clone());
                    }
                    else {
                        println!("multiple candidates for allergen {} ({} {})", allergen, candidate.unwrap(), ingredient);
                        candidate = None;
                        break;
                    }
                }
            }
            if let Some(a) = candidate {
                println!("assign {} to allergen {} with count {count}/{}", a, allergen, all_intregidents[&a]);
                founds.push((a.clone(), allergen.clone()));
                all_allergens.remove(allergen);
                all_intregidents.remove(&a).or_else(|| panic!("no such ingredient {}", a));
                change = true;
            } else {
                println!("no candidate for allergen {}", allergen);
            }
        }
        if !change {
            break;
        }
    }

    // println!("{:#?}", allergen_map);
    // println!("allergens: {:?}", all_allergens.len());
    // println!("all_intregidents: {:?}", all_intregidents);
    println!("result: {}", all_intregidents.values().sum::<i32>());
    founds.sort_by(|x, y| x.1.cmp(&y.1));
    println!("{}", founds.iter().map(|(x, _)| x).join(","));

    Ok(())
}
