use ndarray::{Array, Array2};

use crate::{Batch, CATEGORY_COUNT, CostVec, ITEM_SET_CATEGORY_ORDER, MATERIAL_COUNT, model::item_set::ItemSetCategory};
use std::fmt::Write;

pub fn batch_cost(batch: &Batch) -> CostVec {
    return batch.iter()
                .enumerate()
                .map(|(idx, r)| r.clone().dot(&ITEM_SET_CATEGORY_ORDER[idx].cost_matrix_ndarray()))
                .fold(Array2::zeros((1, MATERIAL_COUNT)), |sum, i| sum + i);
}

pub fn format_cost_vector(cost_vector: &CostVec) -> String {
    let mut res: String = String::new();
    for n in cost_vector {
        let _ = write!(res, "{} ", n);
    }
    res.pop();
    return res;
}

pub fn format_batch_long(batch: &Batch) -> String  {
    let mut res: String = String::new();

    for i in 0..batch.len() {
        let category = &ITEM_SET_CATEGORY_ORDER[i];
        let names = category.item_order();
        
        let queue = &batch[i];
        if queue.iter().all(|x| *x == 0) { continue; }

        let _ = write!(res, "{}(", &category.to_string());
        
        for (j, q) in queue.row(0).iter().enumerate() {
            if *q != 0 {
                let _ = write!(res, "{} x [", q.to_string());

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

pub fn format_batch_short(batch: &Batch) -> String  {
    let mut res: String = String::new();
    let category_start_val = 'A' as u32;

    for i in 0..batch.len() {
        let queue = &batch[i];
        if queue.iter().all(|x| *x == 0) { continue; }

        for (j, q) in queue.row(0).iter().enumerate() {
            if *q != 0 {
                let _ = write!(res, "{}{}{} ", q.to_string(), char::from_u32(category_start_val + i as u32).unwrap(), j.to_string());
            }
        }
    }
    res.pop();
    return res;
}

pub fn format_batch_groups(batch: &Batch) -> String {
    let mut res: String = String::new();
    for i in 0..CATEGORY_COUNT {
        if i < batch.len() && batch[i].iter().any(|x| *x != 0) {
            let _ = write!(res, "1 ");
        } else {
            let _ = write!(res, "0 ");
        }
    }
    res.pop();
    return res;
}