use std::fmt;

use crate::{OrderNum, MATERIAL_COUNT};
use crate::model::item_set::ItemSetCategory;
use ndarray::{Array, Array2};
use strum_macros::EnumIter;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, EnumIter)]
pub enum MaterialGroupedWardenCategories {
    SmallArms,
    HeavyArms,
    HeavyAmmunition,
    Utility,
    Medical,
    Resources,
    Uniforms
}

impl ItemSetCategory for MaterialGroupedWardenCategories {
    fn size(&self) -> u8 {
        return match self {
            Self::SmallArms => 13,
            Self::HeavyArms => 18,
            Self::HeavyAmmunition => 5,
            Self::Utility => 14,
            Self::Medical => 2,
            Self::Resources => 1,
            Self::Uniforms => 2,
        }
    }

    fn item_order(&self) -> Vec<Vec<String>> {
        return match self {
            Self::SmallArms => 
                vec![
                    vec![String::from("Clancy-Raca M4")],
                    vec![String::from("Malone MK.2")],
                    vec![String::from("Booker Storm Rifle Model 838"),
                         String::from("Aalto Storm Rifle 24")],
                    vec![String::from("No.4 The Pillory Scattergun"),
                         String::from("PT-815 Smoke Grenade"),
                         String::from("7.62mm"),
                         String::from("9mm"),
                         String::from("Buckshot")],
                    vec![String::from("No.2B Hawthorne")],
                    vec![String::from("Cascadier 873"),
                         String::from("Cometa T2-9")],
                    vec![String::from("A3 Harpa Fragmentation Grenade")],
                    vec![String::from("Blakerow 871"),
                         String::from("Green Ash Grenade")],
                    vec![String::from("Clancy Cinder M3")],
                    vec![String::from("The Hangman 757"),
                         String::from("Sampo Auto-Rifle 77")],
                    vec![String::from("No.1 \"The Liar\" Submachine Gun"),
                         String::from("Fiddler Submachine Gun Model 868"),
                         String::from("7.92mm")],
                    vec![String::from("No.2 Loughcaster"),
                         String::from("12.7mm")],
                    vec![String::from(".44"),
                         String::from("8mm")],
                ],
            Self::HeavyArms =>
                vec![
                    vec![String::from("Willow's Bane Model 845")],
                    vec![String::from("B2 Varsi Anti-Tank Grenade")],
                    vec![String::from("20 Neville Anti-Tank Rifle")],
                    vec![String::from("Carnyx Anti-Tank Rocket Launcher")],
                    vec![String::from("Mounted Bonesaw MK.3"),
                         String::from("Malone Ratcatcher MK.1"),
                         String::from("Cutler Foebreaker")],
                    vec![String::from("Cutler Launcher 4")],
                    vec![String::from("Bonesaw MK.3"),
                         String::from("Cremari Mortar")],
                    vec![String::from("BF5 White Ash Flask Grenade")],
                    vec![String::from("20mm")],
                    vec![String::from("Mammon 91-b")],
                    vec![String::from("Anti-Tank Sticky Bomb")],
                    vec![String::from("AP/RPG"),
                         String::from("ARC/RPG")],
                    vec![String::from("Flare Mortar Shell")],
                    vec![String::from("Shrapnel Mortar Shell")],
                    vec![String::from("Mortar Shell")],
                    vec![String::from("RPG")],
                    vec![String::from("Tremola Grenade GPb-1")],
                    vec![String::from("30mm")],
                ],
            Self::HeavyAmmunition =>
                vec![
                    vec![String::from("68mm")],
                    vec![String::from("250mm \"Purity\" Shell")],
                    vec![String::from("120mm")],
                    vec![String::from("150mm")],
                    vec![String::from("40mm")],
                ],
            Self::Utility =>
                vec![
                    vec![String::from("The Ospreay")],
                    vec![String::from("Willow's Bane Ammo")],
                    vec![String::from("Alligator Charge")],
                    vec![String::from("Falias Raiding Club"),
                         String::from("Shovel"),
                         String::from("Sledge Hammer")],
                    vec![String::from("Water Bucket")],
                    vec![String::from("Wrench"),
                         String::from("Radio"),
                         String::from("Binoculars")],
                    vec![String::from("Havoc Charge")],
                    vec![String::from("Havoc Charge Detonator")],
                    vec![String::from("Buckhorn CCQ-18")],
                    vec![String::from("Metal Beam")],
                    vec![String::from("Gas Mask Filter"),
                         String::from("Tripod")],
                    vec![String::from("Gas Mask")],
                    vec![String::from("Sandbag"),
                         String::from("Barbed Wire")],
                    vec![String::from("Wind Sock"),
                         String::from("Radio Backpack"),
                         String::from("Listening Kit")],
                ],
            Self::Medical =>
                vec![
                    vec![String::from("First Aid Kit")],
                    vec![String::from("Bandages"),
                         String::from("Blood Plasma"),
                         String::from("Soldier Supplies"),
                         String::from("Trauma Kit")],
                ],
            Self::Resources =>
                vec![
                    vec![String::from("Maintenance Supplies")],
                ],
            Self::Uniforms =>
                vec![
                    vec![String::from("Caoivish Parka"),
                         String::from("Gentleman's Peacoat"),
                         String::from("Officer's Regalia"),
                         String::from("Outrider's Mantle"),
                         String::from("Padded Boiler Suit"),
                         String::from("Physician's Jacket"),
                         String::from("Sapper Gear"),
                         String::from("Specialist's Overcoat")],
                    vec![String::from("Gunner's Breastplate")],
                ],
        };
    }

    fn cost_matrix(&self) -> Vec<OrderNum>
    {
        return match self {
            Self::SmallArms => 
                vec![
                    250, 0,  0, 25,
                    0,   0,  0, 25,
                    0,   0,  0, 15,
                    80,  0,  0, 0,
                    70,  0,  0, 0,
                    60,  0,  0, 0,
                    100, 40, 0, 0,
                    140, 0,  0, 0,
                    130, 0,  0, 0,
                    125, 0,  0, 0,
                    120, 0,  0, 0,
                    100, 0,  0, 0,
                    40,  0,  0, 0,
                ],
            Self::HeavyArms =>
                vec![
                    165, 0,   0, 30,
                    95,  125, 0, 0,
                    150, 0,   0, 0,
                    125, 0,   0, 15,
                    100, 0,   0, 5,
                    100, 0,   0, 35,
                    100, 0,   0, 25,
                    100, 80,  0, 0,
                    100, 0,   0, 0,
                    100, 20,  0, 0,
                    50,  100, 0, 0,
                    60,  150, 0, 0,
                    60,  15,  0, 0,
                    60,  20,  0, 0,
                    60,  70,  0, 0,
                    60,  90,  0, 0,
                    75,  100, 0, 0,
                    80,  40,  0, 0,
                ],
            Self::HeavyAmmunition =>
                vec![
                    120, 240, 0,   0,
                    120, 0,   100, 0,
                    120, 0,   10,  0,
                    120, 0,   60,  0,
                    160, 240, 0,   0,
                ],
            Self::Utility => 
                vec![
                    85,  0,   0,  10,
                    135, 0,   20, 0,
                    150, 160, 0,  0,
                    200, 0,   0,  0,
                    80,  0,   0,  0,
                    75,  0,   0,  0,
                    75,  0,   40, 0,
                    75,  0,   20, 0,
                    40,  0,   0,  0,
                    25,  0,   0,  0,
                    100, 0,   0,  0,
                    160, 0,   0,  0,
                    15,  0,   0,  0,
                    150, 0,   0,  0,
                ],
            Self::Medical => 
                vec![
                    60, 0, 0, 0,
                    80, 0, 0, 0,
                ],
            Self::Resources =>
                vec![
                    250, 0, 0, 0,
                ],
            Self::Uniforms =>
                vec![
                    100, 0, 0, 0,
                    150, 0, 0, 0,
                ],
        };
    }

    fn cost_matrix_ndarray(&self) -> Array2<OrderNum> {
        return Array::from_shape_vec((usize::from(self.size()), MATERIAL_COUNT), self.cost_matrix()).unwrap();
    }

    fn to_string(&self) -> String {
        return String::from("MaterialGroupedWarden");
    }
}