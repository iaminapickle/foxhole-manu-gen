use std::fmt;

use crate::{NoMaterials, NO_MATERIALS};
use crate::category::Category;
use strum_macros::EnumIter;
use nalgebra::{Const, Dyn, Matrix, VecStorage};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, EnumIter)]
pub enum WardenCategory {
    SmallArms,
    HeavyArms,
    HeavyAmmunition,
    Utility,
    Medical,
    Resources,
    Uniforms
}

impl Category for WardenCategory {
    fn size(&self) -> u8 {
        return match self {
            Self::SmallArms => 25,
            Self::HeavyArms => 22,
            Self::HeavyAmmunition => 5,
            Self::Utility => 22,
            Self::Medical => 5,
            Self::Resources => 1,
            Self::Uniforms => 9,
        }
    }

    fn item_order(&self) -> Vec<Vec<String>> {
        return match self {
            Self::SmallArms => 
                vec![
                    vec![String::from("Clancy-Raca M4")],
                    vec![String::from("Malone MK.2")],
                    vec![String::from("Booker Storm Rifle Model 838")],
                    vec![String::from("Aalto Storm Rifle 24")],
                    vec![String::from("No.4 The Pillory Scattergun")],
                    vec![String::from("No.2B Hawthorne")],
                    vec![String::from("Cascadier 873")],
                    vec![String::from("A3 Harpa Fragmentation Grenade")],
                    vec![String::from("Blakerow 871")],
                    vec![String::from("Clancy Cinder M3")],
                    vec![String::from("The Hangman 757")],
                    vec![String::from("Sampo Auto-Rifle 77")],
                    vec![String::from("No.1 \"The Liar\" Submachine Gun")],
                    vec![String::from("Fiddler Submachine Gun Model 868")],
                    vec![String::from("No.2 Loughcaster")],
                    vec![String::from("Green Ash Grenade")],
                    vec![String::from(".44")],
                    vec![String::from("8mm")],
                    vec![String::from("PT-815 Smoke Grenade")],
                    vec![String::from("Cometa T2-9")],
                    vec![String::from("7.62mm")],
                    vec![String::from("9mm")],
                    vec![String::from("Buckshot")],
                    vec![String::from("7.92mm")],
                    vec![String::from("12.7mm")],
                ],
            Self::HeavyArms =>
                vec![
                    vec![String::from("Willow's Bane Model 845")],
                    vec![String::from("B2 Varsi Anti-Tank Grenade")],
                    vec![String::from("20 Neville Anti-Tank Rifle")],
                    vec![String::from("Carnyx Anti-Tank Rocket Launcher")],
                    vec![String::from("Mounted Bonesaw MK.3")],
                    vec![String::from("Malone Ratcatcher MK.1")],
                    vec![String::from("Cutler Foebreaker")],
                    vec![String::from("Cutler Launcher 4")],
                    vec![String::from("Bonesaw MK.3")],
                    vec![String::from("BF5 White Ash Flask Grenade")],
                    vec![String::from("20mm")],
                    vec![String::from("Cremari Mortar")],
                    vec![String::from("Mammon 91-b")],
                    vec![String::from("Anti-Tank Sticky Bomb")],
                    vec![String::from("AP/RPG")],
                    vec![String::from("ARC/RPG")],
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
                    vec![String::from("Falias Raiding Club")],
                    vec![String::from("Shovel")],
                    vec![String::from("Water Bucket")],
                    vec![String::from("Wrench")],
                    vec![String::from("Radio")],
                    vec![String::from("Binoculars")],
                    vec![String::from("Havoc Charge")],
                    vec![String::from("Havoc Charge Detonator")],
                    vec![String::from("Buckhorn CCQ-18")],
                    vec![String::from("Metal Beam")],
                    vec![String::from("Sledge Hammer")],
                    vec![String::from("Gas Mask Filter")],
                    vec![String::from("Gas Mask")],
                    vec![String::from("Sandbag")],
                    vec![String::from("Barbed Wire")],
                    vec![String::from("Wind Sock")],
                    vec![String::from("Radio Backpack")],
                    vec![String::from("Listening Kit")],
                    vec![String::from("Tripod")],
                ],
            Self::Medical =>
                vec![
                    vec![String::from("First Aid Kit")],
                    vec![String::from("Bandages")],
                    vec![String::from("Blood Plasma")],
                    vec![String::from("Soldier Supplies")],
                    vec![String::from("Trauma Kit")],
                ],
            Self::Resources =>
                vec![
                    vec![String::from("Maintenance Supplies")],
                ],
            Self::Uniforms =>
                vec![
                    vec![String::from("Caoivish Parka")],
                    vec![String::from("Gentleman's Peacoat")],
                    vec![String::from("Officer's Regalia")],
                    vec![String::from("Outrider's Mantle")],
                    vec![String::from("Padded Boiler Suit")],
                    vec![String::from("Physician's Jacket")],
                    vec![String::from("Sapper Gear")],
                    vec![String::from("Specialist's Overcoat")],
                    vec![String::from("Gunner's Breastplate")],
                ],
        };
    }

    fn cost_matrix(&self) -> Matrix<u16, Dyn, NoMaterials, VecStorage<u16, Dyn, NoMaterials>>
    {
        let data = match self {
            Self::SmallArms => 
                vec![
                    250, 0,  0, 25,
                    0,   0,  0, 25,
                    0,   0,  0, 15,
                    0,   0,  0, 15,
                    80,  0,  0, 0,
                    70,  0,  0, 0,
                    60,  0,  0, 0,
                    100, 40, 0, 0,
                    140, 0,  0, 0,
                    130, 0,  0, 0,
                    125, 0,  0, 0,
                    125, 0,  0, 0,
                    120, 0,  0, 0,
                    120, 0,  0, 0,
                    100, 0,  0, 0,
                    140, 0,  0, 0,
                    40,  0,  0, 0,
                    40,  0,  0, 0,
                    80,  0,  0, 0,
                    60,  0,  0, 0,
                    80,  0,  0, 0,
                    80,  0,  0, 0,
                    80,  0,  0, 0,
                    120, 0,  0, 0,
                    100, 0,  0, 0,
                ],
            Self::HeavyArms =>
                vec![
                    165,  0, 0, 30,
                    95, 125, 0, 0,
                    150,  0, 0, 0,
                    125,  0, 0, 15,
                    100,  0, 0, 5,
                    100,  0, 0, 5,
                    100,  0, 0, 5,
                    100,  0, 0, 35,
                    100,  0, 0, 25,
                    100, 80, 0, 0,
                    100,  0, 0, 0,
                    100,  0, 0, 25,
                    100, 20, 0, 0,
                    50, 100, 0, 0,
                    60, 150, 0, 0,
                    60, 150, 0, 0,
                    60,  15, 0, 0,
                    60,  20, 0, 0,
                    60,  70, 0, 0,
                    60,  90, 0, 0,
                    75, 100, 0, 0,
                    80,  40, 0, 0,
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
                    200, 0,   0,  0,
                    80,  0,   0,  0,
                    75,  0,   0,  0,
                    75,  0,   0,  0,
                    75,  0,   0,  0,
                    75,  0,   40, 0,
                    75,  0,   20, 0,
                    40,  0,   0,  0,
                    25,  0,   0,  0,
                    200, 0,   0,  0,
                    100, 0,   0,  0,
                    160, 0,   0,  0,
                    15,  0,   0,  0,
                    15,  0,   0,  0,
                    150, 0,   0,  0,
                    150, 0,   0,  0,
                    150, 0,   0,  0,
                    100, 0,   0,  0,
                ],
            Self::Medical => 
                vec![
                    60, 0, 0, 0,
                    80, 0, 0, 0,
                    80, 0, 0, 0,
                    80, 0, 0, 0,
                    80, 0, 0, 0,
                ],
            Self::Resources =>
                vec![
                    250, 0, 0, 0,
                ],
            Self::Uniforms =>
                vec![
                    100, 0, 0, 0,
                    100, 0, 0, 0,
                    100, 0, 0, 0,
                    100, 0, 0, 0,
                    100, 0, 0, 0,
                    100, 0, 0, 0,
                    100, 0, 0, 0,
                    100, 0, 0, 0,
                    150, 0, 0, 0,
                ],
        };
        return Matrix::from_row_slice_generic(Dyn(self.size().into()), Const::<NO_MATERIALS>, &data);
    }
}

impl fmt::Display for WardenCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}