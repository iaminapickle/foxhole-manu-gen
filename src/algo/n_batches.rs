use std::{fs::File, io::{BufWriter, Write}};

use crate::{cost_metric::CostMetric, helper::{format_batch_long, format_batch_short, format_cost_vector}, model::item_set::{material_grouped_warden_categories::MaterialGroupedWardenCategories, output_legend_file, warden_categories::WardenCategories, ItemSetCategory, ItemSetOption}, Batch, CostVec, QueueVec, ARGS, CATEGORY_COUNT, ITEM_SET_CATEGORY_ORDER, OUTPUT_PATH, TRUCK_SIZE_U16};

pub fn find_all_batches_with_metric(metric: CostMetric) {
    find_n_batches_with_metric(CATEGORY_COUNT, metric);
}


pub fn find_n_batches_with_metric(n: usize, metric: CostMetric) {
    // Crash if n < 1  
    if n < 1 { panic!("n must be >= 1, was provided {n}"); }

    // Base valid queues for all categories
    let base_queues: Vec<Vec<(QueueVec, CostVec, u16)>> = ITEM_SET_CATEGORY_ORDER.iter().map(|c|  { let mut res = c.generate_valid_queue_vec();
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

    let mut output = if ARGS.output { Some(BufWriter::new(File::create(output_path).unwrap())) } else { None };
    if ARGS.output && !ARGS.output_batch_long { output_legend_file(); }

    while let Some((cur_batch, cur_cost, cur_item_count)) = stack.pop() {
        // Exit conditions
        // If batch is length n and satisfied the metric, output to file
        if cur_batch.len() == n {
            if let Some(ref mut f) = output && metric.satisfies_metric(&cur_cost) {
                let batch_string = if ARGS.output_batch_long { format_batch_long(&cur_batch) } else { format_batch_short(&cur_batch) };
                let _ = writeln!(f, "Batch: {}\nCost : {}", batch_string, format_cost_vector(&cur_cost));
            }
            continue;
        }

        // Add children to continue search
        if let Some(next_queues) = base_queues.get(cur_batch.len()) {
            // For all base queues in the next category
            for (next_queue, next_cost, next_item_count) in next_queues {
                let new_cost = cur_cost.clone() + next_cost;
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