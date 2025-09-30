use std::fmt;

use crate::{material::Material, CostVec, TRUCK_SIZE_U16, MATERIAL_ORDER};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub fn no_slots(cv: &CostVec) -> u16 {
    return cv.iter().enumerate().map(|(idx, x)| x.div_ceil(MATERIAL_ORDER[idx].stack_value())).sum::<u16>();
}

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
    pub fn check_metric(&self, cv: &CostVec) -> bool {
        return match self {
            Self::Affordable => { 
                cv.iter().enumerate().map(|(idx, x)| x.div_ceil(MATERIAL_ORDER[idx].stack_value())).sum::<u16>() <= TRUCK_SIZE_U16
            },
            Self::NValid(n) => {
                cv.iter().enumerate().map(|(idx, x)| x.div_ceil(MATERIAL_ORDER[idx].stack_value())).sum::<u16>() == *n
            }, 
            Self::Stackable => {
                cv.iter().enumerate().all(|(idx, x)| x % MATERIAL_ORDER[idx].stack_value() == 0)
            },
            Self::Crateable => {
                cv.iter().enumerate().all(|(idx, x)| x % MATERIAL_ORDER[idx].crate_value() == 0)
            },
            // Self::Stackable && Self::NValid(15) but in one iterator loop with early termination
            Self::PerfectlyStackable(n) => {
                let res = cv.iter().enumerate().try_fold(
                    0,
                    |sum, (idx, x)| {
                        if x % MATERIAL_ORDER[idx].stack_value() != 0 {
                            Err(sum)
                        } else {
                            Ok(sum + x.div_ceil(MATERIAL_ORDER[idx].stack_value()))
                        }
                    }
                );
                return match res {
                    Ok(sum) => sum == *n,
                    Err(_) => false, 
                };
            },
            // Self::Crateable && Self::NValid(15) but in one iterator loop with early termination
            Self::PerfectlyCrateable(n) => {
                let res = cv.iter().enumerate().try_fold(
                    0,
                    |sum, (idx, x)| {
                        if x % MATERIAL_ORDER[idx].crate_value() != 0 {
                            Err(sum)
                        } else {
                            Ok(sum + x.div_ceil(MATERIAL_ORDER[idx].stack_value()))
                        }
                    }
                );
                return match res {
                    Ok(sum) => sum == *n,
                    Err(_) => false, 
                };
            }
        }
    }
}

impl fmt::Display for CostMetric {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}