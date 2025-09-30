mod item_set;
mod material;
mod cost_metric;
mod helper;

use std::{fs::File, io::Write, time::Instant};

use crate::{cost_metric::CostMetric, helper::{format_batch_long, format_batch_short, format_cost_vector}, item_set::{material_grouped_warden_item_set::MaterialGroupedWardenItemSet, output_legend_file, warden_item_set::WardenItemSet, ItemSet}, material::Material};
use clap::Parser;
use nalgebra::{RowDVector, RowSVector, U4, U7};
use strum::IntoEnumIterator;
use lazy_static::lazy_static;

type CostVec = RowSVector<u16, NO_MATERIALS>;
type QueueVec = RowDVector<u16>;
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
    static ref args: Cli = Cli::parse();
}

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long, default_value_t = false)]
    output: bool,
    #[arg(long, default_value_t = false)]
    output_batch_long: bool,
}

pub fn find_n_batches_with_metrics<S: ItemSet>(n: usize, metrics: Vec<CostMetric>) where {   
    let category_order: Vec<S> = S::iter().collect();
    let base_queues: Vec<Vec<(QueueVec, CostVec)>> = category_order.iter().map(|c|  c.generate_valid_queue_vecs()).collect();

    let mut stack: Vec<(Batch, CostVec)> = Vec::new();
    for (q, c) in base_queues.first().unwrap().clone() {
        stack.push((vec![q], c));
    }

    let output_suffix = if args.output_batch_long { String::from("long") } else { String::from("short")};
    let path = format!("{n}_batches_with_{:?}_{}.txt", metrics, output_suffix);
    let mut output = if args.output { Some(File::create(path).unwrap()) } else { None };
    if args.output && !args.output_batch_long { output_legend_file::<MaterialGroupedWardenItemSet>(); }

    while !stack.is_empty() {
        let cur: (Batch, CostVec) = stack.pop().unwrap();
        if cur.0.len() == n {
            if metrics.iter().all(|m| m.check_metric(&cur.1)) &&
               let Some(ref mut f) = output {
                let batch_string = if args.output_batch_long { format_batch_long::<S>(cur.0) } else { format_batch_short::<S>(cur.0) };
                let _ = write!(f, "Batch: {}\nCost: {}\n", batch_string, format_cost_vector(cur.1));
            }
            continue;
        }

        for (q,c) in base_queues[cur.0.len()].clone() {
            let new_cost = cur.1 + c;
            if CostMetric::Affordable.check_metric(&new_cost) {
                let mut tmp = cur.0.clone();
                tmp.push(q);
                stack.push((tmp, new_cost));
            }
        }
    }
}

pub fn find_all_batches_with_metrics<S: ItemSet>(metrics: Vec<CostMetric>) {
    find_n_batches_with_metrics::<S>(NO_CATEGORIES.try_into().unwrap(), metrics);
}

fn main() {
    let metrics: Vec<CostMetric> = Vec::from([
        // CostMetric::PerfectlyCrateable(TRUCK_SIZE_u16),
        CostMetric::PerfectlyStackable(TRUCK_SIZE_U16)
        ]);
    let now = Instant::now();
    find_n_batches_with_metrics::<MaterialGroupedWardenItemSet>(2, metrics);
    // find_all_batches_with_metrics::<MaterialGroupedWardenCategory>(metrics);
    println!("Elapsed: {:.2?}", now.elapsed());
}