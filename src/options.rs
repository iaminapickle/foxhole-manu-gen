use std::{collections::HashSet, fs::File, io::BufReader, path::PathBuf};

use clap::Parser;
use serde::Deserialize;

use crate::{model::item_set::ItemSetOption, OrderNum, CATEGORY_COUNT, ITEM_SET_CATEGORY_ORDER, MAX_ORDER, MAX_ORDER_U16};

#[derive(Parser, Debug)]
pub struct Cli {
    /// Enable output files
    #[arg(short, long, default_value_t = false)]
    pub output: bool,
    /// Output file path
    #[arg(short, long, requires = "output")]
    pub path: Option<PathBuf>,
    /// Show full item names in output
    #[arg(short = 'l', long, default_value_t = false, requires = "output")]
    pub output_batch_long: bool,

    #[arg(short, long)]
    pub json_option_path: Option<PathBuf>,

    #[arg(short, long)]
    pub item_set: ItemSetOption,
}

type CategoryNum = usize;
type ItemNum = usize;
type OrderRange = Vec<OrderNum>;
type Queue = Vec<OrderNum>;

type CategoryQueue = (CategoryNum, Queue);
type CategoryItems = (CategoryNum, Vec<(ItemNum, OrderRange)>);

#[derive(Deserialize, Debug)]
pub enum OptionChoice {
    Category(CategoryNum),
    ItemOrders(CategoryItems),
    Queue(CategoryQueue)
}

impl OptionChoice {
    // Panics if not valid
    fn check_valid(&self) -> bool {
        match self {
            OptionChoice::Category(c) => {
                if *c >= CATEGORY_COUNT { panic!("JSON Options: [Category] must be between [0 - {CATEGORY_COUNT})."); }
            }
            OptionChoice::ItemOrders((c, items)) => {
                if *c >= CATEGORY_COUNT { panic!("JSON Options: [ItemsOrders] category must be between [0 - {CATEGORY_COUNT})."); }
                for (item, range) in items {
                    if *item >= usize::from(ITEM_SET_CATEGORY_ORDER[*c].size()) {
                        panic!("JSON Options: [ItemsOrders] item must be between [0 - {}) for category {}.", ITEM_SET_CATEGORY_ORDER[*c].size(), *c);
                    }

                    for r in range {
                        if *r > MAX_ORDER_U16 {
                            panic!("Json Options: [ItemsOrders] item range must be between [0 - {MAX_ORDER}].")
                        }
                    }
                }
            }
            OptionChoice::Queue((c, items)) => {
                let category_size = ITEM_SET_CATEGORY_ORDER[*c].size();

                if *c >= CATEGORY_COUNT { panic!("JSON Options: [Queue] category must be between [0 - {CATEGORY_COUNT})."); }
                if items.len() != usize::from(category_size) { panic!("JSON Options: [Queue] queue size must be {category_size}.")}
                if items.iter().sum::<u16>() > MAX_ORDER_U16 {
                    panic!("JSON Options: [Queue] orders must sum <= {MAX_ORDER}");
                }
                for item in items {
                    if *item > MAX_ORDER_U16 {
                        panic!("JSON Options: [Queue] order must be between [0 - {MAX_ORDER}).");
                    }
                }
            }
        }
        return true;
    }

    fn category(&self) -> CategoryNum {
        return match self {
            OptionChoice::Category(c) => *c,
            OptionChoice::ItemOrders((c, _)) => *c,
            OptionChoice::Queue((c, _)) => *c,
        };
    }
}

#[derive(Deserialize, Default, Debug)]
pub struct JsonOptions {
    // Default order range
    pub order_range: Option<OrderRange>,
    // If specified, these options will not be included in any batches/groups
    // Blacklist:
    //     * Categories   (category)                    -> category queue will default to all 0's
    //     * Item Orders  (category, (item, [orders]))  -> item orders will not be included in category queue vec
    //     * Queues       (category, [queue])           -> specific category queue will not be included in any batch/group
    pub blacklist: Option<Vec<OptionChoice>>,
    // If specified, these options will be included if its category is in a batch/group
    // Whitelist:
    //     * Item Orders  (category, (item, [orders]))  -> item orders will be included in category queue vec (overrides order_range)
    //                                                  -> item must have a single order
    //                                                  -> category queue must still be valid
    //     * Queues       (category, [queue])           -> specific category queue will be included in all batches/groups
    pub whitelist: Option<Vec<OptionChoice>>
}

impl JsonOptions {
    pub fn from_file(json_path: &PathBuf) -> JsonOptions {
        let file = File::open(json_path).unwrap();
        let reader = BufReader::new(file);

        let json_options: JsonOptions = serde_json::from_reader(reader).unwrap();
        return json_options;
    }

    // Panics if not valid
    // Not to be used as a golden vector
    pub fn check_valid(&self) -> bool {
        let mut order_range_values: HashSet<OrderNum> = HashSet::new();
        if self.order_range.is_some() {
            for order in self.order_range.as_ref().unwrap().iter() {
                if *order > MAX_ORDER_U16 { panic!("JSON Options: [Order Range] value must be between [0 - {MAX_ORDER}].") }
                if order_range_values.contains(&order) { panic!("JSON Options: [Order Range] value must be unique.") }
                order_range_values.insert(*order);
            }
        }
        
        let mut blacklist_categories: HashSet<CategoryNum> = HashSet::new();
        let mut blacklist_item_orders: HashSet<CategoryNum> = HashSet::new();
        let mut blacklist_queues: HashSet<CategoryNum> = HashSet::new();
        if self.blacklist.is_some() {
            for option_choice in self.blacklist.as_ref().unwrap().iter() {
                option_choice.check_valid();

                let category = option_choice.category();
                match option_choice {
                    OptionChoice::Category(_) => {
                        if blacklist_categories.contains(&category) { panic!("JSON Options: [Blacklist] [Category] must be unique.") }
                        blacklist_categories.insert(category);
                    },
                    OptionChoice::ItemOrders(_) => {
                        if blacklist_item_orders.contains(&category) { panic!("JSON Options: [Blacklist] [ItemOrder] categories must be unique.") }
                        blacklist_item_orders.insert(category);
                    },
                    OptionChoice::Queue(_) => {
                        if blacklist_queues.contains(&category) { panic!("JSON Options: [Blacklist] [Queue] categories must be unique.") }
                        blacklist_queues.insert(category);
                    }
                }
                
            }
            
            if !blacklist_categories.is_disjoint(&blacklist_item_orders) || !blacklist_categories.is_disjoint(&blacklist_queues) {
                panic!("JSON Options: [Blacklist] [Category] cannot be duplicated in [ItemOrder] or [Queue].")
            }
        }
        
        let mut whitelist_item_orders: HashSet<CategoryNum> = HashSet::new();
        let mut whitelist_queues: HashSet<CategoryNum> = HashSet::new();
        if self.whitelist.is_some() {
            for option_choice in self.whitelist.as_ref().unwrap().iter() {
                option_choice.check_valid();
                
                let category = option_choice.category();
                match option_choice {
                    OptionChoice::Category(_) => { panic!("JSON Options: [Whitelist] does not support [Category].") },
                    OptionChoice::ItemOrders(_) => {
                        if whitelist_item_orders.contains(&category) { panic!("JSON Options: [Whitelist] [ItemOrder] categories must be unique.") }
                        whitelist_item_orders.insert(category);
                    },
                    OptionChoice::Queue(_) => {
                        if whitelist_queues.contains(&category) { panic!("JSON Options: [Whitelist] [Queue] categories must be unique.") }
                        whitelist_queues.insert(category);
                    }
                }
            }

            if !whitelist_item_orders.is_disjoint(&whitelist_queues) {
                panic!("JSON Options: [Whitelist] [ItemOrder] category cannot be duplicated in [Queue].")
            }
        }

        return true;
    }
}