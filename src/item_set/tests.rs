#[cfg(test)]
mod tests {
    use crate::{item_set::{material_grouped_warden_item_set::MaterialGroupedWardenItemSet, warden_item_set::WardenItemSet, ItemSet}, MATERIAL_COUNT};

    fn item_order_length_test<S: ItemSet>() {
        for category in S::iter() {
            assert_eq!(usize::from(category.size()), category.item_order().len());
        }
    }

    #[test]
    fn warden_item_set_item_order_length_test() {
        item_order_length_test::<WardenItemSet>();
    }

    #[test]
    fn material_grouped_warden_item_set_item_order_length_test() {
        item_order_length_test::<MaterialGroupedWardenItemSet>();
    }

    fn cost_matrix_dimension_test<S: ItemSet>() {
        for category in S::iter() {
            let cost_matrix = category.cost_matrix();
            assert_eq!(usize::from(category.size()), cost_matrix.nrows());
            assert_eq!(MATERIAL_COUNT, cost_matrix.ncols());
        }
    }

    #[test]
    fn warden_item_set_cost_matrix_dimension_test() {
        cost_matrix_dimension_test::<WardenItemSet>();
    }

    #[test]
    fn material_grouped_warden_item_set_cost_matrix_dimension_test() {
        cost_matrix_dimension_test::<MaterialGroupedWardenItemSet>();
    }
}