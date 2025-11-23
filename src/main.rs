mod model;
mod cost_metric;
mod helper;
mod options;
mod algo;

use std::{env::current_dir, fs::create_dir_all, path::PathBuf, time::Instant};

use crate::{algo::{good_lp::solve_batch, n_batches::find_n_batches_with_metric, n_prime_groups::find_all_prime_groups_with_metric}, cost_metric::CostMetric, helper::{batch_cost, format_batch_short, format_cost_vector}, model::{item_set::{ItemSetCategory, ItemSetCategoryWrapper, ItemSetOption}, material::Material}, options::{Cli, JsonOptions}};
use clap::Parser;
use ndarray::Array2;
use strum::IntoEnumIterator;
use lazy_static::lazy_static;

type CostNum = u16;
type OrderNum = u16;
type WeightNum = f32;
type Weights = Vec<WeightVec>;
// 1 x MATERIAL_COUNT row vector
type CostVec = Array2<CostNum>;
// 1 x category.size() row vector
type QueueVec = Array2<OrderNum>; 
// 1 x category.size() row vector
type WeightVec = Array2<OrderNum>; 
type Batch = Vec<QueueVec>;

const CATEGORY_COUNT: usize = 7;
const MATERIAL_COUNT: usize = 4;

const TRUCK_SIZE: usize = 15;
const TRUCK_SIZE_I32: i32 = 15;
const TRUCK_SIZE_U16: u16 = 15;
const MAX_ORDER: usize = 4;
const MAX_ORDER_I32: i32 = 4;
const MAX_ORDER_U16: u16 = 4;

lazy_static! {
    // [BMat, EMat, HEMat, RMat]
    static ref MATERIAL_ORDER: Vec<Material> = Material::iter().collect();

    pub static ref ARGS: Cli = Cli::parse();
    pub static ref OUTPUT_PATH: PathBuf = {
        let path = ARGS.path.clone().unwrap_or(current_dir().unwrap());
        if !path.exists() {
            let _ = create_dir_all(&path);
        }
        return path;
    };

    pub static ref JSON_OPTIONS: JsonOptions = {
        let json_options = if ARGS.json_option_path.is_some() { JsonOptions::from_file(&ARGS.json_option_path.as_ref().unwrap()) } 
                                                         else { JsonOptions::default() };
        json_options.check_valid();
        return json_options
    };

    pub static ref ITEM_SET_NAME: String = {
        return match ARGS.item_set {
            ItemSetOption::Warden => String::from("WardenItemSet"),
            ItemSetOption::MaterialGroupedWarden => String::from("MaterialGroupedWardenItemSet"),
        };
    };

    pub static ref ITEM_SET_CATEGORY_ORDER: Vec<ItemSetCategoryWrapper> = ARGS.item_set.item_set_category_order();
}

fn main() {
    let now = Instant::now();
    // find_all_batches_with_metric::<MaterialGroupedWardenItemSet>(CostMetric::PerfectlyStackable(TRUCK_SIZE_U16));
    // find_n_groups_with_metric::<MaterialGroupedWardenItemSet>(2, CostMetric::PerfectlyStackable(TRUCK_SIZE_U16));
    // find_all_groups_with_metric::<MaterialGroupedWardenItemSet>(CostMetric::PerfectlyCrateable(TRUCK_SIZE_U16));
    // find_prime_n_groups_with_metric::<MaterialGroupedWardenItemSet>(2, CostMetric::Stackable);
    // find_n_batches_with_metric(2, CostMetric::PerfectlyStackable(TRUCK_SIZE_U16));
    let res = solve_batch().unwrap();
    let _ = println!("Batch: {}\nCost : {}", format_batch_short(&res), format_cost_vector(&batch_cost(&res)));
    // find_all_prime_groups_with_metric(CostMetric::Stackable);
    println!("Elapsed: {:.2?}", now.elapsed());
}