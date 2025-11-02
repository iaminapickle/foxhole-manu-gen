#[cfg(test)]
mod tests {
    use crate::{item_set::{material_grouped_warden_categories::MaterialGroupedWardenCategories, warden_categories::WardenCategories, ItemSetCategory}, ITEM_SET_CATEGORY_ORDER, MATERIAL_COUNT};

    fn item_order_length_test<S: ItemSetCategory>() {
        for category in ITEM_SET_CATEGORY_ORDER.iter() {
            assert_eq!(usize::from(category.size()), category.item_order().len());
        }
    }

    #[test]
    fn warden_item_set_item_order_length_test() {
        item_order_length_test::<WardenCategories>();
    }

    #[test]
    fn material_grouped_warden_item_set_item_order_length_test() {
        item_order_length_test::<MaterialGroupedWardenCategories>();
    }

    fn cost_matrix_dimension_test<S: ItemSetCategory>() {
        for category in ITEM_SET_CATEGORY_ORDER.iter() {
            let cost_matrix = category.cost_matrix();
            assert_eq!(usize::from(category.size()), cost_matrix.nrows());
            assert_eq!(MATERIAL_COUNT, cost_matrix.ncols());
        }
    }

    #[test]
    fn warden_item_set_cost_matrix_dimension_test() {
        cost_matrix_dimension_test::<WardenCategories>();
    }

    #[test]
    fn material_grouped_warden_item_set_cost_matrix_dimension_test() {
        cost_matrix_dimension_test::<MaterialGroupedWardenCategories>();
    }
}