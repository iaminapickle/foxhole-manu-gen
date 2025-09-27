use std::fmt;

use crate::{CostVec, material::Material};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, PartialEq, Eq, Hash, Clone, EnumIter)]
pub enum OptimalityMetric {
    Affordable,
    NValid(u16),
    Stackable,
    Crateable,
    PerfectMetric,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, EnumIter)]
pub enum PerfectMetric {
    PerfectlyStackable(u16),
    PerfectlyCrateable(u16),
}

impl PerfectMetric {
    pub fn check_metric(&self, cv: CostVec) -> bool {
        return match self {
            Self::PerfectlyStackable(n) => {
                OptimalityMetric::Stackable.check_metric(cv) && 
                OptimalityMetric::NValid(*n).check_metric(cv)
            },
            Self::PerfectlyCrateable(n) => {
                OptimalityMetric::Crateable.check_metric(cv) && 
                OptimalityMetric::NValid(*n).check_metric(cv)
            }
        }
    }
}

impl fmt::Display for PerfectMetric {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl OptimalityMetric {
    pub fn check_metric(&self, cv: CostVec) -> bool {
        let material_order: Vec<Material> = Material::iter().collect();
        return match self {
            Self::Affordable => { 
                cv.iter().enumerate().map(|(idx, x)| x.div_ceil(material_order[idx].stack_value())).sum::<u16>() <= 15 
            },
            Self::NValid(n) => {
                cv.iter().enumerate().map(|(idx, x)| x.div_ceil(material_order[idx].stack_value())).sum::<u16>() == *n
            }, 
            Self::Stackable => {
                cv.iter().enumerate().all(|(idx, x)| x % material_order[idx].stack_value() == 0)
            },
            Self::Crateable => {
                cv.iter().enumerate().all(|(idx, x)| x % material_order[idx].crate_value() == 0)
            },
            Self::PerfectMetric => {
                self.check_metric(cv)
            }
        }
    }
}

impl fmt::Display for OptimalityMetric {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}