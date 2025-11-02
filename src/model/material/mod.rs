use strum_macros::EnumIter;

#[derive(Debug, PartialEq, Eq, Hash, Clone, EnumIter)]
pub enum Material {
    BasicMaterial,
    ExplosiveMaterial,
    HeavyExplosiveMaterial,
    RefinedMaterial,
}

impl Material {
    pub fn stack_value(&self) -> u16 {
        return match self {
            Self::BasicMaterial => 100,
            Self::ExplosiveMaterial => 100,
            Self::HeavyExplosiveMaterial => 100,
            Self::RefinedMaterial => 100,
        }
    }

    pub fn crate_value(&self) -> u16 {
        return match self {
            Self::BasicMaterial => 100,
            Self::ExplosiveMaterial => 40,
            Self::HeavyExplosiveMaterial => 30,
            Self::RefinedMaterial => 20,
        }
    }
}