pub mod warden_categories;
pub mod material_grouped_warden_categories;

use std::{any::type_name, collections::VecDeque, fmt::Write as fmtWrite, fs::File, io::{BufWriter, Write as ioWrite}};
use clap::ValueEnum;
use ndarray::{Array, Array2};

use crate::{CostVec, ITEM_SET_CATEGORY_ORDER, ITEM_SET_NAME, JSON_OPTIONS, MAX_ORDER_U16, OUTPUT_PATH, OrderNum, QueueVec, TRUCK_SIZE_U16, WeightNum, cost_metric::CostMetric, model::item_set::{material_grouped_warden_categories::MaterialGroupedWardenCategories, warden_categories::WardenCategories}};

const DEFAULT_ORDER_RANGE: std::ops::RangeInclusive<u16> = 0..=MAX_ORDER_U16;

#[derive(Debug, Clone, ValueEnum)]
pub enum ItemSetOption {
    Warden,
    MaterialGroupedWarden,
    // Collie,
    // MaterialGroupedCollie
}

impl ItemSetOption {
    pub fn largest_category_size(&self) -> u8 {
        return self.item_set_category_order().iter()
                                             .map(|c| c.size()).into_iter()
                                             .max().unwrap();
    }

    pub fn item_set_category_order(&self) -> Vec<ItemSetCategoryWrapper> {
        return match self {
            ItemSetOption::Warden => vec![
                ItemSetCategoryWrapper::Warden(WardenCategories::SmallArms),
                ItemSetCategoryWrapper::Warden(WardenCategories::HeavyArms),
                ItemSetCategoryWrapper::Warden(WardenCategories::HeavyAmmunition),
                ItemSetCategoryWrapper::Warden(WardenCategories::Utility),
                ItemSetCategoryWrapper::Warden(WardenCategories::Medical),
                ItemSetCategoryWrapper::Warden(WardenCategories::Resources),
                ItemSetCategoryWrapper::Warden(WardenCategories::Uniforms),
            ],
            ItemSetOption::MaterialGroupedWarden => vec![
                ItemSetCategoryWrapper::MaterialGroupedWarden(MaterialGroupedWardenCategories::SmallArms),
                ItemSetCategoryWrapper::MaterialGroupedWarden(MaterialGroupedWardenCategories::HeavyArms),
                ItemSetCategoryWrapper::MaterialGroupedWarden(MaterialGroupedWardenCategories::HeavyAmmunition),
                ItemSetCategoryWrapper::MaterialGroupedWarden(MaterialGroupedWardenCategories::Utility),
                ItemSetCategoryWrapper::MaterialGroupedWarden(MaterialGroupedWardenCategories::Medical),
                ItemSetCategoryWrapper::MaterialGroupedWarden(MaterialGroupedWardenCategories::Resources),
                ItemSetCategoryWrapper::MaterialGroupedWarden(MaterialGroupedWardenCategories::Uniforms),
            ],
        };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItemSetCategoryWrapper {
    Warden(WardenCategories),
    MaterialGroupedWarden(MaterialGroupedWardenCategories),
}

impl ItemSetCategory for ItemSetCategoryWrapper {
    fn size(&self) -> u8 {
        match self {
            ItemSetCategoryWrapper::Warden(c) => c.size(),
            ItemSetCategoryWrapper::MaterialGroupedWarden(c) => c.size(),
        }
    }

    fn item_order(&self) -> Vec<Vec<String>> {
        match self {
            ItemSetCategoryWrapper::Warden(c) => c.item_order(),
            ItemSetCategoryWrapper::MaterialGroupedWarden(c) => c.item_order(),
        }
    }

    fn cost_matrix(&self) -> Vec<OrderNum> {
        match self {
            ItemSetCategoryWrapper::Warden(c) => c.cost_matrix(),
            ItemSetCategoryWrapper::MaterialGroupedWarden(c) => c.cost_matrix(),
        }
    }

    fn cost_matrix_ndarray(&self) -> Array2<OrderNum> {
        match self {
            ItemSetCategoryWrapper::Warden(c) => c.cost_matrix_ndarray(),
            ItemSetCategoryWrapper::MaterialGroupedWarden(c) => c.cost_matrix_ndarray(),
        }
    }

    fn to_string(&self) -> String {
        match self {
            ItemSetCategoryWrapper::Warden(c) => c.to_string(),
            ItemSetCategoryWrapper::MaterialGroupedWarden(c) => c.to_string(),
        }
    }
}

pub trait ItemSetCategory: Sync {
    // Returns the number of items in a category
    fn size(&self) -> u8;
    // Returns the names of items
    fn item_order(&self) -> Vec<Vec<String>>;
    // Returns a self.size() x MATERIAL_COUNT matrix
    // Retruns a largest_category_size() x MATERIAL_COUNT matrix
    // fn cost_matrix(&self) -> Array2<u16>;
    fn cost_matrix(&self) -> Vec<OrderNum>;
    fn cost_matrix_ndarray(&self) -> Array2<OrderNum>;
    fn to_string(&self) -> String;
    fn category_num(&self) -> usize {
        // Convert both to strings for comparison since we can't directly compare trait objects
        let self_str = self.to_string();
        ITEM_SET_CATEGORY_ORDER.iter()
            .position(|c| c.to_string() == self_str)
            .expect("Category not found in ITEM_SET_CATEGORY_ORDER")
    }
    // Generates all valid queues for this category
    // Returns Vec<(queue, cost, item_count)>
    fn generate_valid_queue_vec(&self) -> Vec<(QueueVec, CostVec, u16)> {
        let mut queue = VecDeque::new();
        queue.push_back(vec![]);

        // If there is a specified order range, use it - otherwise use the default
        let order_range: Vec<u16>;
        if JSON_OPTIONS.order_range.is_some() {
            order_range = JSON_OPTIONS.order_range.as_ref().unwrap().to_vec();
        } else {
            order_range = DEFAULT_ORDER_RANGE.collect();
        }
        
        while let Some(current) = queue.pop_front() {  
            let sum: u16 = current.iter().sum();
            // Add any in order_range to the current queue 
            for n in order_range.iter() {
                if sum + n <= MAX_ORDER_U16 {
                    let mut next: Vec<u16> = current.clone();
                    next.push(*n);
                    queue.push_back(next);
                }
            }

            if queue.front().unwrap().len() == usize::from(self.size()) {
                break; 
            }
        }

        return queue.iter()
                    .map(|v| {
                        let r: Array2<u16> = Array::from_shape_vec((1, v.len()), v.clone()).unwrap();
                        let c =  r.clone().dot(&self.cost_matrix_ndarray());
                        return (r, c, v.iter().sum::<u16>());
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
        let _ = writeln!(file, "There are {} valid queues", valid_queues_vec.len());
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
pub fn output_legend_file() {
    let file_str: String = format!("{}_legend.txt", *ITEM_SET_NAME);
    let output_path = OUTPUT_PATH.join(&file_str);
    let mut file = BufWriter::new(File::create(output_path).unwrap());
    
    let category_start_val = 'A' as u32;
    for i in 0..ITEM_SET_CATEGORY_ORDER.len() {
        let category = &ITEM_SET_CATEGORY_ORDER[i];

        let item_order = category.item_order();
        for j in 0..item_order.len() {
            let names = &item_order[j];

            let mut names_str = String::new();
            for name in names {
                let _ = write!(names_str, "{}, ", name);
            }
            names_str.pop();
            names_str.pop();

            let _ = writeln!(file, "{}{}: {}", char::from_u32(category_start_val + i as u32).unwrap(), j.to_string(), names_str);
        }
    }
}