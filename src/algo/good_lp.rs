use std::collections::HashMap;

use crate::model::material::Material;
use crate::{Batch, CATEGORY_COUNT, ITEM_SET_CATEGORY_ORDER, MATERIAL_COUNT, MAX_ORDER_I32, OrderNum, TRUCK_SIZE_I32, Weights};
use crate::model::item_set::{ItemSetCategory, ItemSetCategoryWrapper};

use ndarray::Array2;
use strum::IntoEnumIterator;
use good_lp::{
    Expression, Solution, SolverModel, variable, variables, default_solver
};

pub fn solve_batch() -> Result<Batch, String> {
    // Weights are all 1 for now
    let weights: Weights = ITEM_SET_CATEGORY_ORDER.iter()
                                                  .map(|c| Array2::ones((1, usize::from(c.size()))))
                                                  .collect();

    let stack_vals: Vec<i32> = Material::iter().map(|m| i32::from(m.stack_value())).collect();
    let crate_vals: Vec<i32> = Material::iter().map(|m| i32::from(m.crate_value())).collect();

    let mut vars = variables!();

    // Declare category choice vector solver variables
    let mut category_costs: HashMap<ItemSetCategoryWrapper, Array2<OrderNum>> =
        HashMap::new();
    let mut category_cv_vars: HashMap<ItemSetCategoryWrapper, Vec<_>> = HashMap::new();

    for category in ITEM_SET_CATEGORY_ORDER.iter() {
        let category_cost: Array2<OrderNum> = category.cost_matrix_ndarray();
        let category_size = category.size();

        let mut category_vars = Vec::with_capacity(category_size.into());
        
        for _ in 0..category_size {
            let v = vars.add(
                variable()
                    .integer()
                    .min(0)
                    .max(MAX_ORDER_I32),
            );
            category_vars.push(v);
        }

        category_costs.insert(*category, category_cost);
        category_cv_vars.insert(*category, category_vars);
    }

    // Declare crate, stack and stack_slots solver variables
    let mut stack_vars = Vec::with_capacity(MATERIAL_COUNT);
    let mut crate_vars = Vec::with_capacity(MATERIAL_COUNT);
    let mut stack_slot_vars= Vec::with_capacity(MATERIAL_COUNT);
    for _ in Material::iter() {
        stack_vars.push(vars.add(variable().integer().min(0)));
        crate_vars.push(vars.add(variable().integer().min(0)));
        stack_slot_vars.push(vars.add(variable().integer().min(0)));
    }

    // Declare total items solver expression
    // total_items = sum over category_cv_vars
    let total_items_expr: Expression =
        ITEM_SET_CATEGORY_ORDER.iter()
            .flat_map(|category| category_cv_vars.get(&category).unwrap().iter().copied())
            .fold(Expression::from(0), |acc, i| acc + i);

    // Declare total slots solver expression
    // Total stack slots = sum over stack_vars
    let total_slots_expr: Expression = (0..MATERIAL_COUNT)
        .map(|m| stack_slot_vars[m])
        .fold(Expression::from(0), |acc, s| acc + s);

    // Declare weight solver expression
    // weight = sum of weights over category_cv_vars
    let mut weight_expr = Expression::from(0);

    for (idx, category) in ITEM_SET_CATEGORY_ORDER.iter().enumerate() {
        let cv = category_cv_vars.get(category).unwrap();
        let w_mat = &weights[idx];     

        for row in 0..usize::from(category.size()) {
            let w = w_mat[[0, row]] as i32; 
            if w != 0 {
                weight_expr += w * cv[row];
            }
        }
    }

    let mut problem = vars
        .maximise(weight_expr.clone())
        .using(default_solver);

    // Constraint: each category choice vector has <= MAX_ORDER
    for category in ITEM_SET_CATEGORY_ORDER.iter() {
        let category_cv = category_cv_vars.get(&category).unwrap();

        let sum_category_expr: Expression = category_cv
            .iter()
            .fold(Expression::from(0), |acc, &o| acc + o);

        problem = problem.with(sum_category_expr.leq(MAX_ORDER_I32));
    }

    // Constraint: 1 <= total_items <= TRUCK_SIZE
    problem = problem.with(total_items_expr.clone().geq(1));
    problem = problem.with(total_items_expr.clone().leq(TRUCK_SIZE_I32));

    // Constraint: Require all materials to be divisible by its crate cost
    for m in 0..MATERIAL_COUNT {
        let mut batch_cost = Expression::from(0);

        for category in ITEM_SET_CATEGORY_ORDER.iter() {
            let category_cost = category_costs.get(&category).unwrap();
            let category_cv = category_cv_vars.get(&category).unwrap();

            for row in 0..usize::from(category.size()) {
                let row_usize = row;
                let row_cost = category_cost[[row_usize, m]] as i32;
                if row_cost != 0 {
                    batch_cost += row_cost * category_cv[row_usize];
                }
            }
        }
        
        problem = problem.with((stack_vals[m] * stack_slot_vars[m]).geq(batch_cost.clone()));
        problem = problem.with((stack_vals[m] * stack_slot_vars[m] - stack_vals[m] + 1).leq(batch_cost.clone()));
        
        problem = problem.with((crate_vals[m] * crate_vars[m]).eq(batch_cost));
        // Divisible by stack val instead
        // problem = problem.with((stack_vals[m] * stack_vars[m]).eq(batch_cost));
    }

    // Constraint total_slots == 15
    problem = problem.with(total_slots_expr.clone().eq(TRUCK_SIZE_I32));

    let solution = problem
        .solve()
        .map_err(|e| format!("No feasible batch: {e}"))?;

    // Format solution
    let mut batch: Batch = Vec::with_capacity(CATEGORY_COUNT);

    for category in ITEM_SET_CATEGORY_ORDER.iter() {
        let cat_vars = category_cv_vars.get(&category).unwrap();

        let mut row: Vec<OrderNum> = Vec::with_capacity(cat_vars.len());

        for &v in cat_vars {
            let count = solution.value(v).round() as u16;
            row.push(count);
        }

        let queue = Array2::<OrderNum>::from_shape_vec((1, row.len()), row).unwrap();

        batch.push(queue);
    }

    return Ok(batch);
}
