use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use thiserror::Error;

mod modifier;
mod wmc;
use modifier::*;
use wmc::*;

#[derive(Error, Debug)]
pub enum CookError {
    #[error("parse error {0}")]
    Parse(#[from] serde_json::Error),
    #[error("io error {0}")]
    File(#[from] std::io::Error),
    #[error("Unknown item {0}")]
    UnknownItem(String),
    #[error("Recipe not found")]
    NotFound,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
enum AVec {
    One(Vec<String>),
    Two(Vec<Vec<String>>),
}
impl AVec {
    fn len(&self) -> usize {
        match self {
            Self::One(v) => v.len(),
            Self::Two(v) => v.len(),
        }
    }
    fn id(&self, i: usize) -> &[String] {
        match self {
            Self::One(v) => &v,
            Self::Two(v) => &v[i],
        }
    }
    fn vec(&self) -> Vec<Vec<String>> {
        match self {
            Self::One(v) => vec![v.clone()],
            Self::Two(v) => v.clone(),
        }
    }
}

fn inter(a: &[String], b: &[String]) -> Vec<String> {
    let mut c = vec![];
    for ai in a {
        if b.iter().any(|x| x == ai) {
            c.push(ai.clone())
        }
    }
    c
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecipeBase {
    hb: i32,
    name: String,
    tags: AVec,
    actors: AVec,
    num: i32,
    #[serde(default)]
    id: i32,
}
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Recipe {
    pub name: String,
    pub id: i32,
    actors: Vec<Vec<String>>,
    tags: Vec<Vec<String>>,
    pub items: Vec<String>,
    pub hp: f32,
    pub time: i32,
    pub potency: i32,
    pub effect_level_name: Potency,
    pub level: i32,
    #[serde(default)]
    pub effect: Modifier,
    pub hearts: f32,
    pub price: i32,
    pub hp_crit: i32,
    pub time_crit: i32,
    pub level_crit: i32,
    pub crit_rate: i32,
    pub monster_rng: bool,
    pub stamina: f32,
    pub stamina_crit: f32,
    pub stamina_extra: f32,
    pub stamina_extra_crit: f32,
    pub wmc: WMC,
}

impl Recipe {
    fn rock_hard_food(items: &[String], r: &RecipeBase) -> Self {
        Self {
            actors: r.actors.vec(),
            tags: r.tags.vec(),
            name: "Rock-Hard Food".to_string(),
            hp: 1.0,
            id: if unique_len(items) == 1 { 126 } else { 3 },
            hearts: 0.25,
            price: 2,
            items: items.to_vec(),
            hp_crit: 1,
            level: 1,
            wmc: WMC::new(2, 1),
            ..Default::default()
        }
    }
    fn dubious_food(hp: f32, items: &[String], r: &RecipeBase) -> Self {
        let mut hp = hp;
        if hp < 4.0 {
            hp = 4.0;
        }
        Self {
            actors: r.actors.vec(),
            tags: r.tags.vec(),
            name: "Dubious Food".to_string(),
            hp,
            id: r.id,
            hearts: hp / 4.0,
            price: 2,
            items: items.to_vec(),
            hp_crit: hp as i32,
            level: 1,
            wmc: WMC::new(2, 2),
            ..Default::default()
        }
    }
}

fn unique(items: &[String]) -> Vec<String> {
    let mut h = HashMap::new();
    for item in items {
        h.insert(item, 1);
    }
    h.keys().map(|x| x.to_string()).collect()
}
fn unique_mod(items: &[Modifier]) -> Vec<Modifier> {
    let mut h: HashSet<Modifier> = HashSet::new();
    for item in items {
        h.insert(*item);
    }
    h.into_iter().collect()
}

fn unique_len(items: &[String]) -> usize {
    let mut h = HashMap::new();
    for item in items {
        h.insert(item, 1);
    }
    h.keys().len()
}

impl RecipeBase {
    fn matches(&self, items: &[String], tags: &[String], strict: bool, verbose: bool) -> bool {
        if verbose {
            println!("-------------------------------------");
        }
        if strict {
            if verbose {
                println!("init name {} id {} ", self.name, self.id);
                println!("strict mode");
            }
            let mut v: Vec<String> = items.iter().map(|x| x.to_string()).collect();
            v.sort_unstable();
            v.dedup();
            if v.len() != 1 {
                if verbose {
                    println!("Number of unique items != 1");
                    println!("     items: {:?}", v);
                }
                return false;
            }
        }
        let mut items_t = items.to_vec().clone();
        let mut tags_t = tags.to_vec().clone();
        if verbose {
            println!("init name {} id {} ", self.name, self.id);
            println!("     items: {:?}", items_t);
            println!("      tags: {:?}", tags_t);
            println!("    actors: {:?}", self.actors);
        }
        let out = self.matches_actors(items_t, tags_t, strict, verbose);
        if out.is_none() {
            return false;
        }
        let out = out.unwrap();
        items_t = out.0;
        tags_t = out.1;

        if verbose {
            println!("");
            println!("     items: {:?}", items_t);
            println!("      tags: {:?}", tags_t);
            println!("recipe tags: {:?}", self.tags);
        }
        let items_t = self.matches_tags(items_t, tags_t, strict, verbose);
        if verbose {
            println!("");
            println!("     items: {:?}", items_t);
        }
        if items_t.is_none() {
            return false;
        }
        let items_t = items_t.unwrap();
        if verbose {
            println!("done: {} {:?}", self.name, items_t);
        }
        if strict {
            return items_t.len() == 0;
        }
        return true;
    }
    fn matches_actors(
        &self,
        items_t: Vec<String>,
        tags_t: Vec<String>,
        strict: bool,
        verbose: bool,
    ) -> Option<(Vec<String>, Vec<String>)> {
        let mut items_t = items_t;
        let mut tags_t = tags_t;
        if strict {
            if self.actors.len() == 0 {
                if verbose {
                    println!("No actors, returning current values");
                }
                return Some((items_t, tags_t));
            }
            let v = inter(&self.actors.id(0), &items_t);
            if v.len() == 0 {
                if verbose {
                    println!("No matching actors, returning empty");
                }
                return None;
            }
            if verbose {
                println!("Found matching actors, removing from items {:?}", v);
            }
            let v = &v[0];
            let mut k = items_t.iter().position(|x| &x == &v);
            while k.is_some() {
                items_t.remove(k.unwrap());
                tags_t.remove(k.unwrap());
                k = items_t.iter().position(|x| &x == &v);
            }
            if verbose {
                println!(
                    "Found matching actors, removing from items {:?} {:?}",
                    items_t, tags_t
                );
            }
            return Some((items_t, tags_t));
        }
        let n = self.actors.len();
        if verbose {
            println!("ACTORS {n}");
        }
        for i in 0..n {
            if verbose {
                println!("{:?} {:?}", self.actors.id(i), items_t);
            }
            let v = inter(self.actors.id(i), &items_t);
            if v.len() == 0 {
                return None;
            }
            let mut k = items_t.iter().position(|x| x == &v[0]);
            while k.is_some() {
                items_t.remove(k.unwrap());
                tags_t.remove(k.unwrap());
                k = items_t.iter().position(|x| x == &v[0]);
            }
        }
        return Some((items_t, tags_t));
    }
    fn matches_tags(
        &self,
        items_t: Vec<String>,
        tags_t: Vec<String>,
        strict: bool,
        verbose: bool,
    ) -> Option<Vec<String>> {
        let mut items_t = items_t;
        let mut tags_t = tags_t;
        if verbose {
            println!("    item tags: {:?}", tags_t);
        }
        if strict {
            if self.tags.len() == 0 {
                return Some(items_t);
            }
            let v = inter(self.tags.id(0), &tags_t);
            if v.len() == 0 {
                return None;
            }
            let mut k = tags_t.iter().position(|x| x == &v[0]);
            while k.is_some() {
                items_t.remove(k.unwrap());
                tags_t.remove(k.unwrap());
                k = tags_t.iter().position(|x| x == &v[0]);
            }
            return Some(items_t);
        }

        let tags = match &self.tags {
            AVec::Two(v) => v,
            AVec::One(v) => {
                if v.len() == 0 {
                    return Some(items_t);
                } else {
                    panic!(":( {:?}", self)
                }
            }
        };
        let n = tags.len();
        for i in 0..n {
            let mut k = None;
            for j in 0..tags[i].len() {
                if verbose {
                    println!("{:?} {:?}, {} {}", tags[i], tags_t, i, j)
                }
                k = tags_t.iter().position(|x| x == &tags[i][j]);
                if k.is_some() {
                    break;
                }
            }
            if k.is_none() {
                return None;
            }

            let item = items_t[k.unwrap()].clone();
            while k.is_some() {
                items_t.remove(k.unwrap());
                tags_t.remove(k.unwrap());
                k = items_t.iter().position(|x| x == &item);
            }
        }
        Some(items_t)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Effect {
    base_time: i32,
    material_rate: f32,
    max: i32,
    min: i32,
    ssa: i32,
    #[serde(rename = "type")]
    kind: Modifier,
    xtype: String,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Item {
    hp: i32,
    effect: Modifier,
    potency: i32,
    time: i32,
    #[serde(rename = "tags")]
    tags_raw: Vec<String>,
    #[serde(default)]
    tags: String,
    name: String,
    time_boost: i32,
    hp_boost: i32,
    cook_low_price: bool,
    key_item: bool,
    roast_item: bool,
    #[serde(deserialize_with = "parse_i32")]
    sell_price: i32,
    #[serde(deserialize_with = "parse_i32")]
    buy_price: i32,
    #[serde(default)]
    boost_success_rate: i32,
}
use serde::Deserializer;
fn parse_i32<'de, D>(d: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(d).map(|x: Option<_>| x.unwrap_or(0))
}

fn read_recipes() -> Result<Vec<RecipeBase>, CookError> {
    Ok(serde_json::from_str(include_str!("cook_recipes.json"))?)
}
fn read_items() -> Result<HashMap<String, Item>, CookError> {
    //let fp = File::open(file)?;
    //let buf = BufReader::new(fp);
    Ok(serde_json::from_str(include_str!("cook_items.json"))?)
}
fn read_tags() -> Result<Vec<String>, CookError> {
    //let fp = File::open(file)?;
    //let buf = BufReader::new(fp);
    Ok(serde_json::from_str(include_str!("cook_tags.json"))?)
}
fn read_names() -> Result<HashMap<String, String>, CookError> {
    //let fp = File::open(file)?;
    //let buf = BufReader::new(fp);
    Ok(serde_json::from_str(include_str!("names.json"))?)
}
fn read_effects() -> Result<Vec<Effect>, CookError> {
    //let fp = File::open(file)?;
    //let buf = BufReader::new(fp);
    Ok(serde_json::from_str(include_str!("cook_effects.json"))?)
}

pub struct Cook {
    pub effects: Vec<Effect>,
    pub names: HashMap<String, String>,
    pub inames: HashMap<String, String>, // Inverse names
    pub tags: Vec<String>,
    pub data: HashMap<String, Item>, // items
    pub recipes: Vec<RecipeBase>,
    pub price_scale: Vec<f32>,
    pub crit_scale: Vec<i32>,
    dubious: RecipeBase,
    pub verbose: bool,
    //threshold: HashMap<&'static str, [i32; 2]>,
    //elixirs: HashMap<&'static str, &'static str>,
}

impl Cook {
    pub fn set_verbose(&mut self, verbose: bool) {
        self.verbose = verbose
    }
    pub fn new() -> Self {
        let names = read_names().unwrap();
        let mut data_raw = read_items().unwrap();
        let mut data = HashMap::new();
        // reduce_tags()
        for (name, item) in &mut data_raw {
            if item.tags_raw.len() > 1 {
                panic!("Item > 1 cook tag {} {:?}", name, item.tags_raw);
            }
            if item.tags_raw.len() == 1 {
                item.tags = item.tags_raw[0].to_string();
            }
            //if item.effect == "" || item.effect == Modifier::None {
            //    item.effect = "".to_string();
            //}
            data.insert(name.to_string(), item.clone());
        }
        // set_proper_names()
        let mut inames = HashMap::new();
        for key in names.keys() {
            if data.get(key).is_some() {
                let value = names.get(key).unwrap().to_string();
                if inames.get(&value).is_some() && key.starts_with("Animal_") {
                    continue;
                }
                inames.insert(value, key.to_string());
            }
        }
        let prefer = [
            ["Hearty Radish", "Item_PlantGet_B"],
            ["Big Hearty Radish", "Item_PlantGet_C"],
            ["Endura Carrot", "Item_PlantGet_Q"],
            ["Swift Carrot", "Item_PlantGet_M"],
            ["Silent Princess", "Item_PlantGet_J"],
            ["Octo Balloon", "Item_Enemy_57"],
            ["Master Sword", "Item_Sword_080"],
        ];
        for [key, value] in prefer {
            if inames.get(key).is_some() {
                inames.insert(key.to_string(), value.to_string());
            }
        }
        for key in data.keys() {
            if names.get(key).is_none() {
                eprintln!("Missing {key} from data {:?}", names.get(key));
            }
        }
        let mut recipes = read_recipes().unwrap();
        for i in 0..recipes.len() {
            recipes[i].id = i as i32;
        }
        let dubious = recipes
            .iter()
            .find(|x| x.name == "Dubious Food")
            .unwrap()
            .clone();
        /*
        let mut threshold = HashMap::new();
        threshold.insert("AttackUp", [5, 7]);
        threshold.insert("DefenseUp", [5, 7]);
        threshold.insert("ResistCold", [6, 999]);
        threshold.insert("ResistHot", [6, 999]);
        threshold.insert("ResistElectric", [4, 6]);
        threshold.insert("Fireproof", [7, 999]);
        threshold.insert("MovingSpeed", [5, 7]);
        threshold.insert("Quietness", [6, 9]);
        threshold.insert("LifeMaxUp", [999, 999]);
        threshold.insert("GutsRecover", [999, 999]);
        threshold.insert("ExGutsMaxUp", [999, 999]);
        threshold.insert("None", [999, 999]);
        let mut elixirs = HashMap::new();
        elixirs.insert("AttackUp", "Mighty Elixir");
        elixirs.insert("DefenseUp", "Tough Elixir");
        elixirs.insert("ResistCold", "Spicy Elixir");
        elixirs.insert("ResistHot", "Chilly Elixir");
        elixirs.insert("ResistElectric", "Electro Elixir");
        elixirs.insert("Fireproof", "Fireproof Elixir");
        elixirs.insert("MovingSpeed", "Hasty Elixir");
        elixirs.insert("Quietness", "Sneaky Elixir");
        elixirs.insert("ExGutsMaxUp", "Enduring Elixir");
        elixirs.insert("GutsRecover", "Energizing Elixir");
        elixirs.insert("LifeMaxUp", "Hearty Elixir");
        */
        Self {
            effects: read_effects().unwrap(),
            names,
            inames,
            tags: read_tags().unwrap(),
            data, // items
            recipes,
            price_scale: vec![0.0, 1.5, 1.8, 2.1, 2.4, 2.8], // Cooking::CookData:NMMR
            crit_scale: vec![5, 10, 15, 20, 25],             // Cooking::CookData::NMSSR
            dubious,
            verbose: false,
            //threshold,
            //elixirs,
        }
    }
    pub fn item_names(&self, items: &[String]) -> Result<Vec<String>, CookError> {
        let mut inames = vec![];
        for item in items {
            let value = self.inames.get(item);
            if value.is_none() {
                return Err(CookError::UnknownItem(item.to_string()));
            }
            inames.push(value.unwrap().to_string())
        }
        Ok(inames)
    }

    pub fn find_recipe(&self, items: &Vec<String>) -> RecipeBase {
        let iname: Vec<String> = self.item_names(&items).unwrap();
        let tags_t: Vec<String> = iname
            .iter()
            .map(|key| self.data.get(key.as_str()).unwrap().tags.clone())
            .collect();
        let n = 125;
        //let i = N;
        for recipe in &self.recipes[n..] {
            if recipe.matches(&iname, &tags_t, true, self.verbose) {
                return recipe.clone();
            }
        }
        for recipe in &self.recipes[..n] {
            if recipe.matches(&iname, &tags_t, false, self.verbose) {
                return recipe.clone();
            }
        }
        self.dubious.clone()
    }
    pub fn get_effect(&self, name: Modifier) -> &Effect {
        self.effects.iter().find(|eff| eff.kind == name).unwrap()
    }
    pub fn item(&self, name: &str) -> &Item {
        self.data.get(self.inames.get(name).unwrap()).unwrap()
    }
    pub fn cook<S: AsRef<str>>(&self, items: &[S]) -> Recipe {
        let items: Vec<String> = items.iter().map(|x| x.as_ref().to_string()).collect();
        let r = self.find_recipe(&items);

        let monster_rng = items.contains(&"Monster Extract".to_string())
            && r.name != "Dubious Food"
            && r.name != "Rock-Hard Food";

        let life_rate = 2;
        let mut hp = 0;
        let mut potency = 0;
        let mut time = 0;
        let mut effects = vec![];
        let mut sell_price = 0;
        let mut buy_price = 0;
        for name in &items {
            let val = self.item(&name);
            let has_effect = val.effect != Modifier::None;
            if has_effect {
                let eff = self.get_effect(val.effect);
                if self.verbose {
                    println!("effect {} {}", val.effect, eff.base_time);
                }
                time += eff.base_time;
            }
            if val.roast_item {
                time += 30;
            } else {
                time += val.time / 30;
            }
            if has_effect {
                potency += val.potency;
            }
            hp += val.hp;
            if has_effect {
                effects.push(val.effect.clone());
            }
            if val.cook_low_price {
                sell_price += 1;
                buy_price += 1;
            } else {
                sell_price += val.sell_price;
                buy_price += val.buy_price;
            }
        }
        hp *= life_rate;
        if self.verbose {
            println!("pre scaled sell_price: {sell_price}");
        }
        let sp_scale32 = sell_price as f32 * self.price_scale[items.len()];
        if self.verbose {
            println!("    scaled sell_price: {sp_scale32}");
        }
        sell_price = ((sp_scale32.floor() / 10.).ceil() * 10.) as i32;
        if self.verbose {
            println!("    scaled sell_price: {sell_price} buy price: {buy_price}");
        }

        // Selling price is capped at buying price and a limited to a min of 2
        sell_price = sell_price.max(2);
        sell_price = sell_price.min(buy_price);

        effects = unique_mod(&effects);
        let mut effect = if effects.len() == 1 {
            effects[0]
        } else {
            Modifier::None
        };
        let thresh = effect.threshold();

        let potency_level;
        let effect_level;
        if potency >= thresh[1] {
            potency_level = Potency::High;
            effect_level = 3;
        } else if potency >= thresh[0] {
            potency_level = Potency::Mid;
            effect_level = 2;
        } else {
            potency_level = Potency::Low;
            effect_level = 1;
        }
        if self.verbose {
            println!(" effect_level {effect_level} potency_level {potency_level}")
        }

        let time_boost: i32 = items
            .iter()
            .map(|item| self.item(item))
            .map(|item| item.time_boost)
            .sum();

        let hp_boost: i32 = unique(&items)
            .iter()
            .map(|item| self.item(item))
            .map(|item| item.hp_boost)
            .sum();
        if self.verbose {
            println!("time boost {time} + {time_boost}");
            println!("hp   boost {hp} + {hp_boost} + {}", r.hb);
            println!(
                "{:?}",
                unique(&items)
                    .iter()
                    .map(|item| self.item(item))
                    .collect::<Vec<_>>()
            );
        }

        let crits: Vec<_> = items
            .iter()
            .map(|item| self.item(item))
            .map(|item| item.boost_success_rate)
            .collect();
        let mut crit_rate = *crits.iter().max().unwrap();
        crit_rate += self.crit_scale[unique_len(&items) - 1];
        crit_rate = std::cmp::min(crit_rate, 100);
        if self.verbose {
            println!(
                "crits {crit_rate} {:?} {}",
                crits,
                self.crit_scale[unique_len(&items) - 1]
            );
        }

        if r.name == "Rock-Hard Food" {
            return Recipe::rock_hard_food(&items, &r);
        }
        if r.name == "Dubious Food" {
            hp = items
                .iter()
                .map(|item| self.item(item))
                .map(|item| item.hp)
                .sum();
            if hp <= 0 {
                hp = 4;
            }
            return Recipe::dubious_food(hp as f32, &items, &r);
        }
        if r.name == "Fairy Tonic" {
            sell_price = 2;
            effect = Modifier::None;
        }
        hp = hp + hp_boost + r.hb;
        let mut out = Recipe {
            name: r.name,
            id: r.id,
            actors: r.actors.vec(),
            tags: r.tags.vec(),
            items,
            hp: hp.min(120) as f32,
            time: time + time_boost,
            potency,
            effect_level_name: potency_level,
            level: std::cmp::min(effect_level, 3),
            effect: effect,
            hearts: hp as f32 / 4.0,
            price: sell_price,
            hp_crit: hp + 3 * 4,                            // Assumes +3 hearts
            time_crit: time + 5 * 60,                       // Assumes +05:00 duration
            level_crit: std::cmp::min(effect_level + 1, 3), // Assumes +1 potency tier
            crit_rate,
            stamina: 0.0,
            stamina_crit: 0.0,
            stamina_extra: 0.0,
            stamina_extra_crit: 0.0,
            monster_rng,
            wmc: WMC::new(sell_price, hp as i32),
        };

        out.time = out.time.min(30 * 60);
        out.time_crit = out.time_crit.min(30 * 60);

        if out.effect == Modifier::LifeMaxUp {
            out.hp = 0.;
            out.hearts = 0.;
            out.level = out.potency / 4;
            out.level_crit = out.level + 1;
            out.wmc = WMC::new(out.price, out.hp as i32);
        }
        if out.name == "Elixir" && out.effect != Modifier::None {
            out.name = out.effect.elixir().to_string();
        }
        let crit_stamina = 0.4;
        if out.effect == Modifier::GutsRecover {
            let recover = [0.0, 0.2, 0.4, 0.8, 1.0, 1.4, 1.6, 1.8, 2.2, 2.4, 2.8, 3.0];
            let mut potency = potency as usize;
            if potency > recover.len() - 1 {
                potency = recover.len() - 1;
            };
            out.stamina = recover[potency];
            out.stamina_crit = out.stamina + crit_stamina;
            if out.stamina_crit > 3.0 {
                out.stamina_crit = 3.0;
            }
        }
        if out.effect == Modifier::ExGutsMaxUp {
            struct Guts {
                pts: i32,
                val: f32,
            }
            let recover = vec![
                Guts { pts: 0, val: 0.0 },
                Guts { pts: 1, val: 0.2 },
                Guts { pts: 4, val: 0.4 },
                Guts { pts: 6, val: 0.6 },
                Guts { pts: 8, val: 0.8 },
                Guts { pts: 10, val: 1.0 },
                Guts { pts: 12, val: 1.2 },
                Guts { pts: 14, val: 1.4 },
                Guts { pts: 16, val: 1.6 },
                Guts { pts: 18, val: 1.8 },
                Guts { pts: 20, val: 2.0 },
            ];
            let potency = potency.max(20);
            let tmp = recover
                .iter()
                .filter(|x| x.pts <= potency)
                .collect::<Vec<&Guts>>()
                .pop()
                .unwrap();
            out.stamina_extra = tmp.val;
            out.stamina_extra_crit = out.stamina_extra + crit_stamina;
            if out.stamina_extra_crit > 2.0 {
                out.stamina_extra_crit = 2.0;
            }
        }
        if out.name == "Fairy Tonic" && out.items.contains(&"Monster Extract".to_string()) {
            // Using the maximum hp value
            //   - hp can be either 1 or 40 (=28+12)
            out.hp = out.hp_crit as f32;
            out.hearts = out.hp / 4.;
            out.wmc = WMC::new(out.price, out.hp as i32);
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::BufReader;

    #[test]
    fn basic_type() {
        let c = Cook::new();
        let r = c.find_recipe(&vec!["Fairy".to_string()]);
        println!("{:?}", r);
        let r = c.cook(&vec!["Fairy".to_string()]);
        println!("{:?}", r);
    }
    #[test]
    fn basic_reading() {
        let _v = read_recipes().unwrap();
        let _v = read_items().unwrap();
        let _v = read_tags().unwrap();
        let _v = read_names().unwrap();
        let _v = read_effects().unwrap();
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    struct RTest {
        hearts: f32,
        hp: f32,
        id: i32,
        ingredients: Vec<String>,
        #[serde(default)]
        price: i32,
        name: String,
        #[serde(default)]
        img: String,
    }

    #[test]
    fn more_tests() {
        let c = Cook::new();
        let mut k = 0;
        for file in [
            "t/wkr.json",
            "t/dubious.json",
            "t/acorns.json",
            "t/elixirs.json",
            "t/quietness.json",
            "t/other.json",
            "t/fruitcake.json",
            "t/ist.json",
        ] {
            println!("{file}");
            let fp = File::open(file).unwrap();
            let buf = BufReader::new(fp);
            let mut tests: Vec<RTest> = serde_json::from_reader(buf).unwrap();
            let mut i = 0;
            for test in &mut tests {
                if test.price == 0 && test.name == "Dubious Food" {
                    // Some are missing prices for dubious food, they are always 2
                    test.price = 2;
                }
                let r = c.cook(&test.ingredients);
                if r.name != test.name {
                    let r = c.cook(&test.ingredients);
                    panic!(
                        "names mismatch '{}' '{}' {:?} {}",
                        r.name, test.name, test, file
                    );
                }
                if r.hp != test.hp || r.price != test.price || r.hearts != test.hearts {
                    let _r = c.cook(&test.ingredients);
                    println!("{:?}", test);
                }
                assert_eq!(r.hp, test.hp, "{} {} {}", file, i, test.name);
                assert_eq!(r.id, test.id, "{} {} {}", file, i, test.name);
                assert_eq!(r.hearts, test.hearts, "{} {} {}", file, i, test.name);
                assert_eq!(r.price, test.price, "{} {} {}", file, i, test.name);
                i += 1;
            }
            k += i;
        }
        println!("total tests: {k}")
    }
    #[test]
    fn wmc_meal_with_fairy() {
        let c = Cook::new();
        let r = c.cook(&[
            "Silent Princess",
            "Fairy",
            "Fairy",
            "Fairy",
            "Roasted Endura Carrot",
        ]);
        println!("{r:?}");
    }
}
