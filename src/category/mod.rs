pub mod warden_category;
pub mod material_grouped_warden_category;

use std::collections::VecDeque;

use nalgebra::{Dyn, Matrix, VecStorage, U1};

use crate::{optimality_metric::OptimalityMetric, CostVec, NoMaterials, QueueVec, NO_MATERIALS};

pub trait Category {
    fn size(&self) -> u8;
    fn item_order(&self) -> Vec<Vec<String>>;
    fn cost_matrix(&self) -> Matrix<u16, Dyn, NoMaterials, VecStorage<u16, Dyn, NoMaterials>>;

    fn generate_valid_queue_vecs(&self) -> Vec<(QueueVec, CostVec)> {
        let mut queue = VecDeque::new();
        
        queue.push_back(vec![]);
        
        while let Some(current) = queue.pop_front() {  
            let sum: u16 = current.iter().sum();
            for n in 0..=NO_MATERIALS {
                let tmp: u16 = n.try_into().unwrap();
                if sum + tmp <= 4 {
                    let mut next: Vec<u16> = current.clone();
                    next.push(tmp);
                    queue.push_back(next);
                }
            }

            if queue.front().unwrap().len() == self.size().into() {
                break; 
            }
        }

        return queue.iter()
                    .map(|v| {
                        let m = Matrix::from_row_slice_generic(U1, Dyn(self.size().into()), &v);
                        return (m.clone(), m * self.cost_matrix());
                    })
                    .filter(|(_, c)| OptimalityMetric::Affordable.check_metric(*c))
                    .collect();
    }
}
