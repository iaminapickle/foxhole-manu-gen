use std::fmt;

use crate::{CostVec, material::Material};
use rayon::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, PartialEq, Eq, Hash, Clone, EnumIter)]
pub enum CostMetric {
    Affordable,
    NValid(u16),
    Stackable,
    Crateable,
    PerfectlyStackable(u16),
    PerfectlyCrateable(u16),
}

impl CostMetric {
    pub fn check_metric(&self, cv: CostVec, order: &Vec<Material>) -> bool {
        return match self {
            Self::Affordable => { 
                cv.iter().enumerate().map(|(idx, x)| x.div_ceil(order[idx].stack_value())).sum::<u16>() <= 15 
            },
            Self::NValid(n) => {
                cv.iter().enumerate().map(|(idx, x)| x.div_ceil(order[idx].stack_value())).sum::<u16>() == *n
            }, 
            Self::Stackable => {
                cv.iter().enumerate().all(|(idx, x)| x % order[idx].stack_value() == 0)
            },
            Self::Crateable => {
                cv.iter().enumerate().all(|(idx, x)| x % order[idx].crate_value() == 0)
            },
            Self::PerfectlyStackable(n) => {
                Self::Stackable.check_metric(cv, order) && 
                Self::NValid(*n).check_metric(cv, order)
            },
            Self::PerfectlyCrateable(n) => {
                Self::Crateable.check_metric(cv, order) && 
                Self::NValid(*n).check_metric(cv, order)
            }
        }
    }
}

impl fmt::Display for CostMetric {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}