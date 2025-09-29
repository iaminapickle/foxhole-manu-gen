mod category;
mod material;
mod cost_metric;

use std::{collections::HashMap, fmt::Write as fmtWrite, fs::File, io::Write as ioWrite, time::Instant};

use crate::{category::{material_grouped_warden_category::MaterialGroupedWardenCategory, warden_category::WardenCategory, Category}, cost_metric::CostMetric, material::Material};
use bit_vec::BitVec;
use nalgebra::{base, ArrayStorage, DVector, Dyn, Matrix, RowDVector, RowSVector, SVector, VecStorage, U1, U4, U7};
use strum::IntoEnumIterator;
use lazy_static::lazy_static;

type CostVec = RowSVector<u16, NO_MATERIALS>;
type QueueVec = RowDVector<u16>;
type QueueGroup = (Vec<QueueVec>, BitVec);
type Batch = Vec<QueueVec>;

type NoCategories = U7;
const NO_CATEGORIES: usize = 7;

type NoMaterials = U4;
const NO_MATERIALS: usize = 4;

const TRUCK_SIZE: usize = 15;
const TRUCK_SIZE_U16: u16 = 15;
const MAX_ORDER: usize = 4;

lazy_static! {
    static ref MATERIAL_ORDER: Vec<Material> = Material::iter().collect();
}

pub fn format_cost_vector(cost_vector: CostVec) -> String {
    let mut res: String = String::from("[");
    for n in &cost_vector {
        let _ = write!(res, "{}, ", n);
    }
    res.pop();
    res.pop();
    let _ = write!(res, "]");
    return res;
}

pub fn format_batch(batch: Batch) -> String {
    let mut res: String = String::new();
    let order: Vec<MaterialGroupedWardenCategory> = MaterialGroupedWardenCategory::iter().collect();

    for c in 0..batch.len() {
        let category = &order[c];
        let names = category.item_order();
        
        let queue = &batch[c];
        if queue.iter().all(|x| *x == 0) { continue; }

        let _ = write!(res, "{}(", &category.to_string());
        
        for i in 0..queue.len() {
            if queue[i] != 0 {
                let _ = write!(res, "{} x [", &queue[i].to_string());

                for n in &names[i] {
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

pub fn find_n_batches_with_metrics(n: usize, metrics: Vec<CostMetric>) {
    let mut base_queues: HashMap<MaterialGroupedWardenCategory, Vec<(QueueVec, CostVec)>> = HashMap::new();
    for c in MaterialGroupedWardenCategory::iter() {
        base_queues.insert(c.clone(), c.generate_valid_queue_vecs());
    }
    
    let order: Vec<MaterialGroupedWardenCategory> = MaterialGroupedWardenCategory::iter().collect();

    let mut stack: Vec<(Batch, CostVec)> = Vec::new();
    for (q, c) in base_queues[order.first().unwrap()].clone() {
        stack.push((vec![q], c));
    }

    let path = format!("{n}_batches_with_{:?}.txt", metrics);
    let mut output = File::create(path).unwrap();
    while !stack.is_empty() {
        let cur: (Batch, CostVec) = stack.pop().unwrap();
        if cur.0.len() == n {
            if metrics.iter().all(|m| m.check_metric(&cur.1)) {
                let _ = write!(output, "Batch: {}\nCost: {}\n", format_batch(cur.0), format_cost_vector(cur.1));
            }
            continue;
        }

        let next = order[cur.0.len()];
        for (q,c) in base_queues[&next].clone() {
            let new_cost = cur.1 + c;
            if CostMetric::Affordable.check_metric(&new_cost) {
                let mut tmp = cur.0.clone();
                tmp.push(q);
                stack.push((tmp, new_cost));
            }
        }
    }
}

pub fn find_all_batches_with_metrics(metrics: Vec<CostMetric>) {
    find_n_batches_with_metrics(NO_CATEGORIES.try_into().unwrap(), metrics);
}

fn main() {
    let now = Instant::now();
    let metrics: Vec<CostMetric> = Vec::from([
        // CostMetric::PerfectlyCrateable(TRUCK_SIZE_u16),
        CostMetric::PerfectlyStackable(TRUCK_SIZE_U16)
    ]);
    find_n_batches_with_metrics(2, metrics);
    // find_all_batches_with_metrics(metrics);
    println!("Elapsed: {:.2?}", now.elapsed());
}

// pub fn find_n_groups_with_metrics(n: u8, metrics: Vec<CostMetric>) {
//     let mut base_queues: HashMap<MaterialGroupedWardenCategory, Vec<(QueueVec, CostVec)>> = HashMap::new();
//     for c in MaterialGroupedWardenCategory::iter() {
//         base_queues.insert(c.clone(), c.generate_valid_queue_vecs());
//     }
    
//     let order: Vec<MaterialGroupedWardenCategory> = MaterialGroupedWardenCategory::iter().collect();
    
//     let mut stack: Vec<(QueueGroup, CostVec)> = Vec::new();
//     for i in 0..NO_CATEGORIES {
//         let queues = base_queues[&order[i]];
//         let bv = BitVec::from_elem(NO_CATEGORIES, false);
//         bv[i] = true;
//         for (q, c) in queues {
//             stack.push(((vec![q], bv), c))
//         }
//     }

//     let path = format!("all_batches_with_{:?}.txt", metrics);
//     let mut output = File::create(path).unwrap();
//     while !stack.is_empty() {
//         let cur: (Batch, CostVec) = stack.pop().unwrap();
//         if cur.0.len() == NO_CATEGORIES {
//             if metrics.iter().all(|m| m.check_metric(cur.1)) {
//                 let _ = write!(output, "Batch: {}\nCost: {}\n", format_batch(cur.0), format_cost_vector(cur.1));
//             }
//             continue;
//         }

//         let next = order[cur.0.len()].clone();
//         for (q,c) in base_queues[&next].clone() {
//             if CostMetric::Affordable.check_metric(cur.1 + c) {
//                 let mut tmp = cur.0.clone();
//                 tmp.push(q);
//                 stack.push((tmp, cur.1 + c));
//             }
//         }
//     }
// }