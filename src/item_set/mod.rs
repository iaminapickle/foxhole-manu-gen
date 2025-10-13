pub mod warden_item_set;
pub mod material_grouped_warden_item_set;
pub mod tests;

use std::{collections::VecDeque, fs::File, fmt::Write as fmtWrite, io::Write as ioWrite, any::type_name};
use nalgebra::{Dyn, Matrix, RowDVector, VecStorage, U1};
use strum::IntoEnumIterator;

use crate::{cost_metric::CostMetric, CostVec, MaterialCount, QueueVec, MAX_ORDER, OUTPUT_PATH, TRUCK_SIZE_U16};

pub trait ItemSet: ToString + IntoEnumIterator {
    // Returns the number of items in a category
    fn size(&self) -> u8;
    // Returns the names of items
    fn item_order(&self) -> Vec<Vec<String>>;
    // Returns a self.size() x MATERIAL_COUNT matrix 
    fn cost_matrix(&self) -> Matrix<u16, Dyn, MaterialCount, VecStorage<u16, Dyn, MaterialCount>>;
    // Generates all valid queues for this category
    // Returns Vec<(queue, cost, item_count)>
    fn generate_valid_queue_vec(&self) -> Vec<(QueueVec, CostVec, u16)> {
        let mut queue = VecDeque::new();
        queue.push_back(vec![]);
        
        while let Some(current) = queue.pop_front() {  
            let sum: u16 = current.iter().sum();
            // Add any in [0, MAX_ORDER] to the current queue 
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
                        return (r.clone(), r * self.cost_matrix(), v.iter().sum::<u16>());
                    })
                    .filter(|(_, c, s)| CostMetric::Affordable.satisfies_metric(c) && *s <= TRUCK_SIZE_U16)
                    .collect();
    }

    // Debug function that outputs all valid queues of a category to a file
    fn output_valid_queue_vec(&self) {
        let item_set_name = type_name::<Self>().split("::").last().unwrap();
        let file_str: String = format!("{}_{}_valid_queue_vec.txt", item_set_name, self.to_string());
        let output_path = OUTPUT_PATH.join(&file_str);
        let mut file = File::create(output_path).unwrap();

        let valid_queues_vec = self.generate_valid_queue_vec();
        let _ = write!(file, "There are {} valid queues\n", valid_queues_vec.len());
        for (queue, cost, _) in valid_queues_vec {
            let mut queue_string = String::from("Q: [");
            for n in &queue {
                let _ = write!(queue_string, "{n} ");
            }
            queue_string.pop();
            let _ = write!(queue_string, "]\nC: [");
            for n in &cost {
                let _ = write!(queue_string, "{n} ");
            }
            queue_string.pop();
            let _ = write!(queue_string, "]\n");
            let _ = write!(file, "{}", queue_string);
        }
    }
}

// Outputs the legend file for an ItemSet
pub fn output_legend_file<S: ItemSet>() {
        let file_str: String = format!("{}_legend.txt", type_name::<S>().split("::").last().unwrap());
        let output_path = OUTPUT_PATH.join(&file_str);
        let mut file = File::create(output_path).unwrap();
        
        let category_order: Vec<S> = S::iter().collect();
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