use std::{fs::File, io::{BufWriter, Write}, path::PathBuf};

use crate::{cost_metric::{count_stacks, CostMetric}, helper::{format_batch_groups, format_batch_long, format_batch_short, format_cost_vector}, model::item_set::{material_grouped_warden_categories::MaterialGroupedWardenCategories, output_legend_file, warden_categories::WardenCategories, ItemSetCategory, ItemSetOption}, Batch, CostVec, QueueVec, ARGS, CATEGORY_COUNT, ITEM_SET_CATEGORY_ORDER, OUTPUT_PATH, TRUCK_SIZE, TRUCK_SIZE_U16};

pub fn find_all_prime_groups_with_metric(metric: CostMetric) {
    match ARGS.item_set {
        ItemSetOption::Warden                => _find_prime_n_groups_with_metric::<WardenCategories>(CATEGORY_COUNT, metric),
        ItemSetOption::MaterialGroupedWarden => _find_prime_n_groups_with_metric::<MaterialGroupedWardenCategories>(CATEGORY_COUNT, metric),
    };
}

pub fn find_prime_n_groups_with_metric(n:usize, metric: CostMetric) {
    match ARGS.item_set {
        ItemSetOption::Warden                => _find_prime_n_groups_with_metric::<WardenCategories>(n, metric),
        ItemSetOption::MaterialGroupedWarden => _find_prime_n_groups_with_metric::<MaterialGroupedWardenCategories>(n, metric),
    };
}

fn _find_prime_n_groups_with_metric<S: ItemSetCategory>(n: usize, metric: CostMetric) {
    // Crash if n < 1  
    if n < 1 { panic!("n must be >= 1, was provided {n}"); }
    
    // Base valid queues for all categories
    let base_queues: Vec<Vec<(QueueVec, CostVec, u16)>> = ITEM_SET_CATEGORY_ORDER.iter().map(|c|  { let mut res = c.generate_valid_queue_vec();
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
    let mut outputs: [Option<BufWriter<File>>; TRUCK_SIZE * CATEGORY_COUNT] = [const { None }; TRUCK_SIZE * CATEGORY_COUNT];
    if !ARGS.output_batch_long { output_legend_file(); }
    
    while let Some((cur_batch, cur_cost, cur_item_count, cur_non_zero_queue_count)) = stack.pop() {
        // Exit conditions 
        if cur_non_zero_queue_count == 0 { continue; }
        // If batch has n non-zero queues and satisfies metric, output to file
        if metric.satisfies_metric(&cur_cost) {
            if ARGS.output {
                let stack_count =  count_stacks(&cur_cost);
                let file_str = format!("prime_{}_groups_{}_stacks_{}.txt", cur_non_zero_queue_count, stack_count, output_suffix);
                let batch_string = if ARGS.output_batch_long { format_batch_long(&cur_batch) } else { format_batch_short(&cur_batch) };
                
                let idx = usize::from(cur_non_zero_queue_count - 1) * TRUCK_SIZE + usize::from(stack_count - 1);
                let output_path: PathBuf = OUTPUT_PATH.join(&file_str);

                let f = outputs[idx].get_or_insert(BufWriter::new(File::create(output_path).unwrap()));

                let _ = writeln!(f, "Batch : {}\nCost  : {}\nGroups: {}", batch_string, format_cost_vector(&cur_cost), format_batch_groups(&cur_batch));
            }
            continue;
        }

        if usize::from(cur_non_zero_queue_count) == n { continue; }

        // Add children to continue
        // For all base queues in the next category
        if let Some(next_queues) = base_queues.get(cur_batch.len()) {
            for (next_queue, next_cost, next_item_count) in next_queues {
                let new_cost = cur_cost.clone() + next_cost;
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
