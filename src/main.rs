mod item_set;
mod material;
mod cost_metric;
mod helper;

use std::{env::current_dir, fs::{create_dir_all, File}, io::Write, path::{PathBuf}, time::Instant};

use crate::{cost_metric::CostMetric, 
            helper::{format_batch_long, format_batch_short, format_cost_vector},
            item_set::{material_grouped_warden_item_set::MaterialGroupedWardenItemSet, output_legend_file, warden_item_set::WardenItemSet, ItemSet},
            material::Material};
use clap::Parser;
use nalgebra::{RowDVector, RowSVector, U4, U7};
use strum::IntoEnumIterator;
use lazy_static::lazy_static;

// 1 x NO_MATERIALS matrix
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
    static ref ARGS: Cli = Cli::parse();
    static ref OUTPUT_PATH: PathBuf = {
        let path = ARGS.path.clone().unwrap_or(current_dir().unwrap());
        if !path.exists() {
            let _ = create_dir_all(&path);
        }
        return path;
    };
}

#[derive(Parser, Debug)]
struct Cli {
    /// Enable output files
    #[arg(short, long, default_value_t = false)]
    output: bool,
    /// Output file path
    #[arg(short, long, requires = "output")]
    path: Option<PathBuf>,
    /// Show full item names in output
    #[arg(short = 'l', long, default_value_t = false, requires = "output")]
    output_batch_long: bool,
}

pub fn find_n_batches_with_metric<S: ItemSet>(n: usize, metric: CostMetric) where {   
    let category_order: Vec<S> = S::iter().collect();
    let base_queues: Vec<Vec<(QueueVec, CostVec)>> = category_order.iter().map(|c|  c.generate_valid_queue_vecs()).collect();

    let mut stack: Vec<(Batch, CostVec)> = Vec::new();
    for (q, c) in base_queues.first().unwrap().clone() {
        stack.push((vec![q], c));
    }

    let output_suffix = if ARGS.output_batch_long { String::from("long") } else { String::from("short")};
    let file_str = format!("{n}_batches_with_{}_{}.txt", metric, output_suffix);
    let output_path = OUTPUT_PATH.join(&file_str);

    let mut output = if ARGS.output { Some(File::create(output_path).unwrap()) } else { None };
    if !ARGS.output_batch_long { output_legend_file::<MaterialGroupedWardenItemSet>(); }

    while let Some((cur_batch, cur_cost)) = stack.pop() {
        if cur_batch.len() == n {
            if metric.satisfies_metric(&cur_cost) &&
               let Some(ref mut f) = output {
                let batch_string = if ARGS.output_batch_long { format_batch_long::<S>(&cur_batch) } else { format_batch_short::<S>(&cur_batch) };
                let _ = write!(f, "B: {}\nC: {}\n", batch_string, format_cost_vector(&cur_cost));
            }
            continue;
        }

        if let Some(next_queues) = base_queues.get(cur_batch.len()) {
            for (next_queue, next_cost) in next_queues {
                let new_cost = cur_cost + *next_cost;
                
                if CostMetric::Affordable.satisfies_metric(&new_cost) {
                    let mut new_batch = cur_batch.clone();
                    new_batch.push(next_queue.clone());
                    stack.push((new_batch, new_cost));
                }
            }
        }
    }
}

pub fn find_all_batches_with_metric<S: ItemSet>(metric: CostMetric) {
    find_n_batches_with_metric::<S>(NO_CATEGORIES.try_into().unwrap(), metric);
}

fn main() {
    let now = Instant::now();
    find_n_batches_with_metric::<MaterialGroupedWardenItemSet>(2, CostMetric::PerfectlyStackable(TRUCK_SIZE_U16));
    // find_all_batches_with_metric::<MaterialGroupedWardenItemSet>(CostMetric::PerfectlyStackable(TRUCK_SIZE_U16));
    println!("Elapsed: {:.2?}", now.elapsed());
}