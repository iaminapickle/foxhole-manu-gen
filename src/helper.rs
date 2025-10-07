use nalgebra::DMatrix;

use crate::{cost_metric::CostMetric, find_n_batches_with_metric, find_n_groups_with_metric, item_set::ItemSet, Batch, CostVec, CATEGORY_COUNT};
use std::fmt::Write;

pub fn format_cost_vector(cost_vector: &CostVec) -> String {
    let mut res: String = String::new();
    for n in cost_vector {
        let _ = write!(res, "{} ", n);
    }
    res.pop();
    return res;
}

pub fn format_batch_long<S: ItemSet>(batch: &Batch) -> String  {
    let mut res: String = String::new();
    let category_order: Vec<S> = S::iter().collect();

    for i in 0..batch.len() {
        let category = &category_order[i];
        let names = category.item_order();
        
        let queue = &batch[i];
        if queue.iter().all(|x| *x == 0) { continue; }

        let _ = write!(res, "{}(", &category.to_string());
        
        for j in 0..queue.len() {
            if queue[j] != 0 {
                let _ = write!(res, "{} x [", &queue[j].to_string());

                for n in &names[j] {
                    let _ = write!(res, "{}, ", n);
                }
                res.pop();
                res.pop();
                let _ = write!(res, "], ");
            }
        }
        res.pop();
        res.pop();
        let _ = write!(res, ") ");
    }
    return res;
}

pub fn format_batch_short<S: ItemSet>(batch: &Batch) -> String  {
    let mut res: String = String::new();
    let category_start_val = 'A' as u32;

    for i in 0..batch.len() {
        let queue = &batch[i];
        if queue.iter().all(|x| *x == 0) { continue; }
        
        for j in 0..queue.len() {
            if queue[j] != 0 {
                let _ = write!(res, "{}{}{} ", &queue[j].to_string(), char::from_u32(category_start_val + i as u32).unwrap(), j.to_string());
            }
        }
    }
    res.pop();
    return res;
}

pub fn find_all_batches_with_metric<S: ItemSet>(metric: CostMetric) {
    find_n_batches_with_metric::<S>(CATEGORY_COUNT.try_into().unwrap(), metric);
}

pub fn find_all_groups_with_metric<S: ItemSet>(metric: CostMetric) {
    find_n_groups_with_metric::<S>(CATEGORY_COUNT, metric);
}
