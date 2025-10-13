mod item_set;
mod material;
mod cost_metric;
mod helper;

use std::{env::current_dir, fs::{create_dir_all, File}, io::Write, path::{PathBuf}, time::Instant};

use crate::{cost_metric::{count_stacks, CostMetric}, 
            helper::{find_all_batches_with_metric, find_all_groups_with_metric, find_all_prime_groups_with_metric, format_batch_groups, format_batch_long, format_batch_short, format_cost_vector},
            item_set::{material_grouped_warden_item_set::MaterialGroupedWardenItemSet, output_legend_file, ItemSet},
            material::Material};
use clap::Parser;
use nalgebra::{RowDVector, RowSVector, U4, U7};
use strum::IntoEnumIterator;
use lazy_static::lazy_static;

// 1 x MATERIAL_COUNT matrix
type CostVec = RowSVector<u16, MATERIAL_COUNT>;
type QueueVec = RowDVector<u16>;
type Batch = Vec<QueueVec>;

type CategoryCount = U7;
const CATEGORY_COUNT: usize = 7;

type MaterialCount = U4;
const MATERIAL_COUNT: usize = 4;

const TRUCK_SIZE: usize = 15;
const TRUCK_SIZE_U16: u16 = 15;
const MAX_ORDER: usize = 4;

lazy_static! {
    // [BMat, EMat, HEMat, RMat]
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
    // Crash if n < 1  
    if n < 1 { panic!("n must be >= 1, was provided {n}"); }

    // [Small Arms, Heavy Arms, Heavy Ammunition, Utility, Medical, Resources, Uniforms]
    let category_order: Vec<S> = S::iter().collect();
    // Base valid queues for all categories
    let base_queues: Vec<Vec<(QueueVec, CostVec, u16)>> = category_order.iter().map(|c|  { let mut res = c.generate_valid_queue_vec();
                                                                                               res.reverse();
                                                                                               return res;
                                                                                             }).collect();

    // Stack for DFS: Vec<(batch, cost, item_count)>
    let mut stack: Vec<(Batch, CostVec, u16)> = Vec::new();
    for (queue, cost, item_count) in base_queues.first().unwrap().clone() {
        stack.push((vec![queue], cost, item_count));
    }

    let output_suffix = if ARGS.output_batch_long { String::from("long") } else { String::from("short")};
    let file_str = format!("{n}_batches_with_{}_{}.txt", metric, output_suffix);
    let output_path = OUTPUT_PATH.join(&file_str);

    let mut output = if ARGS.output { Some(File::create(output_path).unwrap()) } else { None };
    if ARGS.output && !ARGS.output_batch_long { output_legend_file::<S>(); }

    while let Some((cur_batch, cur_cost, cur_item_count)) = stack.pop() {
        // Exit conditions
        // If batch is length n and satisfied the metric, output to file
        if cur_batch.len() == n {
            if let Some(ref mut f) = output && metric.satisfies_metric(&cur_cost) {
                let batch_string = if ARGS.output_batch_long { format_batch_long::<S>(&cur_batch) } else { format_batch_short::<S>(&cur_batch) };
                let _ = write!(f, "Batch: {}\nCost : {}\n", batch_string, format_cost_vector(&cur_cost));
            }
            continue;
        }

        // Add children to continue search
        if let Some(next_queues) = base_queues.get(cur_batch.len()) {
            // For all base queues in the next category
            for (next_queue, next_cost, next_item_count) in next_queues {
                let new_cost = cur_cost + *next_cost;
                let new_item_count = cur_item_count + next_item_count;

                // If the new batch is affordable and the number of items < TRUCK_SIZE_U16, push to stack
                if CostMetric::Affordable.satisfies_metric(&new_cost) && new_item_count <= TRUCK_SIZE_U16 {
                    let mut new_batch = cur_batch.clone();
                    new_batch.push(next_queue.clone());
                    stack.push((new_batch, new_cost, new_item_count));
                }
            }
        }
    }
}

pub fn find_n_groups_with_metric<S: ItemSet>(n: usize, metric: CostMetric) {
    // Crash if n < 1  
    if n < 1 { panic!("n must be >= 1, was provided {n}"); }
    
    // [Small Arms, Heavy Arms, Heavy Ammunition, Utility, Medical, Resources, Uniforms]
    let category_order: Vec<S> = S::iter().collect();
    // Base valid queues for all categories
    let base_queues: Vec<Vec<(QueueVec, CostVec, u16)>> = category_order.iter().map(|c|  { let mut res = c.generate_valid_queue_vec();
                                                                                               res.reverse();
                                                                                               return res;
                                                                                             }).collect();

    // Stack for DFS: Vec<(batch, cost, item_count, non_zero_queue_count)>
    let mut stack: Vec<(Batch, CostVec, u16, u8)> = Vec::new();
    for (queue, cost, item_count) in base_queues.first().unwrap().clone() {
        // Check if non-zero queue
        let non_zero_queue = if queue.iter().all(|x| *x == 0) { 0 } else { 1 };
        stack.push((vec![queue], cost, item_count, non_zero_queue));
    }

    let output_suffix = if ARGS.output_batch_long { String::from("long") } else { String::from("short")};
    let file_str = format!("{n}_groups_with_{}_{}.txt", metric, output_suffix);
    let output_path: PathBuf = OUTPUT_PATH.join(&file_str);

    let mut output = if ARGS.output { Some(File::create(output_path).unwrap()) } else { None };
    if !ARGS.output_batch_long { output_legend_file::<S>(); }

    while let Some((cur_batch, cur_cost, cur_item_count, cur_non_zero_queue_count)) = stack.pop() {
        // Exit conditions
        // If group has n non-zero queues and satisfies metric, output to file
        if usize::from(cur_non_zero_queue_count) == n {
            if metric.satisfies_metric(&cur_cost) &&
               let Some(ref mut f) = output {
                let batch_string = if ARGS.output_batch_long { format_batch_long::<S>(&cur_batch) } else { format_batch_short::<S>(&cur_batch) };
                let _ = write!(f, "Batch: {}\nCost : {}\n", batch_string, format_cost_vector(&cur_cost));
            }
            continue;
        }

        // Add children to continue search
        // For all base queues in the next category
        if let Some(next_queues) = base_queues.get(cur_batch.len()) {
            for (next_queue, next_cost, next_item_count) in next_queues {
                let new_cost = cur_cost + *next_cost;
                let new_item_count = cur_item_count + *next_item_count;

                let non_zero_queue = if next_queue.iter().all(|x| *x == 0) { 0 } else { 1 };
                let new_non_zero_queue_count = cur_non_zero_queue_count + non_zero_queue; 

                // If the new group is affordable and the number of items < TRUCK_SIZE_U16, push to stack
                if CostMetric::Affordable.satisfies_metric(&new_cost) && new_item_count <= TRUCK_SIZE_U16 {
                    let mut new_batch = cur_batch.clone();
                    new_batch.push(next_queue.clone());
                    stack.push((new_batch, new_cost, new_item_count, new_non_zero_queue_count));
                }
            }
        }
    }
}

fn find_prime_n_groups_with_metric<S: ItemSet>(n: usize, metric: CostMetric) {
    // Crash if n < 1  
    if n < 1 { panic!("n must be >= 1, was provided {n}"); }
    
    // [Small Arms, Heavy Arms, Heavy Ammunition, Utility, Medical, Resources, Uniforms]
    let category_order: Vec<S> = S::iter().collect();
    // Base valid queues for all categories
    let base_queues: Vec<Vec<(QueueVec, CostVec, u16)>> = category_order.iter().map(|c|  { let mut res = c.generate_valid_queue_vec();
                                                                                               res.reverse();
                                                                                               return res;
                                                                                             }).collect();

    // Stack for DFS: Vec<(batch, cost, item_count, non_zero_queue_count)>
    let mut stack: Vec<(Batch, CostVec, u16, u8)> = Vec::new();
    for (queue, cost, item_count) in base_queues.first().unwrap().clone() {
        // Check if non-zero queue
        let non_zero_queue = if queue.iter().all(|x| *x == 0) { 0 } else { 1 };
        stack.push((vec![queue], cost, item_count, non_zero_queue));
    }

    let output_suffix = if ARGS.output_batch_long { String::from("long") } else { String::from("short")};
    let mut outputs: [Option<File>; TRUCK_SIZE * CATEGORY_COUNT] = [const { None }; TRUCK_SIZE * CATEGORY_COUNT];
    if !ARGS.output_batch_long { output_legend_file::<S>(); }
    
    while let Some((cur_batch, cur_cost, cur_item_count, cur_non_zero_queue_count)) = stack.pop() {
        // Exit conditions 
        if cur_non_zero_queue_count == 0 { continue; }
        // If batch has n non-zero queues and satisfies metric, output to file
        if metric.satisfies_metric(&cur_cost) {
            if ARGS.output {
                let stack_count =  count_stacks(&cur_cost);
                let file_str = format!("prime_{}_groups_{}_stacks_{}.txt", cur_non_zero_queue_count, stack_count, output_suffix);
                let batch_string = if ARGS.output_batch_long { format_batch_long::<S>(&cur_batch) } else { format_batch_short::<S>(&cur_batch) };
                
                let idx = usize::from(cur_non_zero_queue_count - 1) * TRUCK_SIZE + usize::from(stack_count - 1);
                let output_path: PathBuf = OUTPUT_PATH.join(&file_str);

                let mut f = match &outputs[idx] {
                    Some(f) => f,
                    None    => {
                        let f = File::create(output_path).unwrap();
                        outputs[idx] = Some(f);
                        &(outputs[idx].as_ref().unwrap())
                    },
                };

                let _ = write!(f, "Batch : {}\nCost  : {}\nGroups: {}\n", batch_string, format_cost_vector(&cur_cost), format_batch_groups::<S>(&cur_batch));
            }
            continue;
        }

        if usize::from(cur_non_zero_queue_count) == n { continue; }

        // Add children to continue
        // For all base queues in the next category
        if let Some(next_queues) = base_queues.get(cur_batch.len()) {
            for (next_queue, next_cost, next_item_count) in next_queues {
                let new_cost = cur_cost + *next_cost;
                let new_item_count = cur_item_count + *next_item_count;

                let non_zero_queue = if next_queue.iter().all(|x| *x == 0) { 0 } else { 1 };
                let new_non_zero_queue_count = cur_non_zero_queue_count + non_zero_queue; 

                // If the new group is affordable and the number of items < TRUCK_SIZE_U16, push to stack
                if CostMetric::Affordable.satisfies_metric(&new_cost) && new_item_count <= TRUCK_SIZE_U16 {
                    let mut new_batch = cur_batch.clone();
                    new_batch.push(next_queue.clone());
                    stack.push((new_batch, new_cost, new_item_count, new_non_zero_queue_count));
                }
            }
        }
    }
}

fn main() {
    let now = Instant::now();
    // find_n_batches_with_metric::<MaterialGroupedWardenItemSet>(2, CostMetric::PerfectlyStackable(TRUCK_SIZE_U16));
    // find_all_batches_with_metric::<MaterialGroupedWardenItemSet>(CostMetric::PerfectlyStackable(TRUCK_SIZE_U16));
    // find_n_groups_with_metric::<MaterialGroupedWardenItemSet>(2, CostMetric::PerfectlyStackable(TRUCK_SIZE_U16));
    // find_all_groups_with_metric::<MaterialGroupedWardenItemSet>(CostMetric::PerfectlyCrateable(TRUCK_SIZE_U16));
    // find_prime_n_groups_with_metric::<MaterialGroupedWardenItemSet>(2, CostMetric::Stackable);
    find_all_prime_groups_with_metric::<MaterialGroupedWardenItemSet>(CostMetric::Stackable);
    // MaterialGroupedWardenItemSet::HeavyAmmunition.output_valid_queue_vec();
    println!("Elapsed: {:.2?}", now.elapsed());
}