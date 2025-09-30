pub mod warden_category;
pub mod material_grouped_warden_category;

use std::collections::VecDeque;

use nalgebra::{DVector, Dyn, Matrix, RowDVector, VecStorage, U1};
use strum::IntoEnumIterator;

use crate::{cost_metric::CostMetric, material::Material, CostVec, NoMaterials, QueueVec, MAX_ORDER};

pub trait Category {
    fn size(&self) -> u8;
    fn item_order(&self) -> Vec<Vec<String>>;
    fn cost_matrix(&self) -> Matrix<u16, Dyn, NoMaterials, VecStorage<u16, Dyn, NoMaterials>>;

    fn generate_valid_queue_vecs(&self) -> Vec<(QueueVec, CostVec)> {
        let mut queue = VecDeque::new();
        queue.push_back(vec![]);
        
        while let Some(current) = queue.pop_front() {  
            let sum: u16 = current.iter().sum();
            for n in 0..=MAX_ORDER {
                if usize::from(sum) + n <= MAX_ORDER {
                    let mut next: Vec<u16> = current.clone();
                    next.push(n.try_into().unwrap());
                    queue.push_back(next);
                }
            }

            if queue.front().unwrap().len() == self.size().into() {
                break; 
            }
        }

        return queue.iter()
                    .map(|v| {
                        let r = RowDVector::from_row_slice_generic(U1, Dyn(self.size().into()), &v);
                        return (r.clone(), r * self.cost_matrix());
                    })
                    .filter(|(_, c)| CostMetric::Affordable.check_metric(c))
                    .collect();
    }
}