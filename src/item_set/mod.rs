pub mod warden_item_set;
pub mod material_grouped_warden_item_set;

use std::{collections::VecDeque, fs::File, fmt::Write as fmtWrite, io::Write as ioWrite, any::type_name};
use nalgebra::{Dyn, Matrix, RowDVector, VecStorage, U1};
use strum::IntoEnumIterator;

use crate::{cost_metric::CostMetric, CostVec, NoMaterials, QueueVec, MAX_ORDER};

pub trait ItemSet: ToString + IntoEnumIterator {
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

pub fn output_legend_file<C: ItemSet>() {
        let path = format!("{}_legend.txt", type_name::<C>().split("::").last().unwrap());
        let mut file = File::create(path).unwrap();
        
        let category_order: Vec<C> = C::iter().collect();
        let category_start_val = 'A' as u32;
        for i in 0..category_order.len() {
            let category = &category_order[i];

            let item_order = category.item_order();
            for j in 0..item_order.len() {
                let names = &item_order[j];

                let mut names_str = String::new();
                for name in names {
                    let _ = write!(names_str, "{}, ", name);
                }
                names_str.pop();
                names_str.pop();

                let _ = write!(file, "{}{}: {}\n", char::from_u32(category_start_val + i as u32).unwrap(), j.to_string(), names_str);
            }
        }
    }