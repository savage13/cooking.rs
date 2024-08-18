use serde::{Deserialize, Serialize};
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WMC {
    pub attack_up: Option<i32>,
    pub durability_up: bool,
    pub critical_hit: bool,
    pub long_throw: Option<f32>,
    pub multi_shot: Option<i32>,
    pub zoom: bool,
    pub quick_shot: Option<f32>,
    pub surf_master: Option<f32>,
    pub shield_guard_up: Option<i32>,
    pub yellow_modifier: bool,
    pub hp: i32,
    pub price: i32,
}

fn is_bit_set(value: i32, k: i32) -> bool {
    value & (1 << k) != 0
}

impl WMC {
    pub fn new(price: i32, hp: i32) -> Self {
        let attack_up = is_bit_set(price, 0).then_some(hp.min(120));
        let long_throw = is_bit_set(price, 3).then_some(hp.min(120) as f32 / 1000.);
        let multi_shot = is_bit_set(price, 4).then_some(hp.min(10));
        let quick_shot = is_bit_set(price, 6).then_some(hp.min(120) as f32 / 1000.);
        let surf_master = is_bit_set(price, 7).then_some(hp.min(120) as f32 / 1000.);
        let shield_guard_up = is_bit_set(price, 8).then_some(hp.min(120));
        Self {
            attack_up,
            durability_up: is_bit_set(price, 1),
            critical_hit: is_bit_set(price, 2),
            long_throw,
            multi_shot,
            zoom: is_bit_set(price, 5),
            quick_shot,
            surf_master,
            shield_guard_up,
            yellow_modifier: is_bit_set(price, 31),
            hp,
            price,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn wmc_test() {
        let mut w0 = WMC {
            ..Default::default()
        };
        let mut w = WMC::new(0, 0);
        assert!(w == w0);

        w0.attack_up = Some(0);
        w0.price = 1;
        w = WMC::new(1, 0);
        assert!(w == w0, "{w:?}");

        w0.attack_up = None;
        w0.yellow_modifier = true;
        w = WMC::new(-512, 0);
        w0.price = -512;
        assert!(w == w0, "{w:?}");

        w0 = WMC {
            ..Default::default()
        };

        w0.attack_up = Some(120);
        w0.durability_up = true;
        w0.long_throw = Some(0.12);
        w0.multi_shot = Some(10);
        w0.surf_master = Some(0.12);
        w0.price = 155;
        w0.hp = 120;
        w = WMC::new(155, 120);
        assert!(w == w0, "{w:?}");

        w0 = WMC {
            ..Default::default()
        };

        w0.attack_up = Some(120);
        w0.multi_shot = Some(10);
        w0.surf_master = Some(0.12);
        w0.price = 145;
        w0.hp = 120;
        w = WMC::new(w0.price, w0.hp);
        assert!(w == w0, "{w:?}");
    }
}
