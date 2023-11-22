struct FilterCondition {
    field: i32,
}

impl FilterCondition {
    /// Checks if the given `item` matches the `field` of the `FilterCondition`.
    ///
    /// # Arguments
    ///
    /// * `item` - The item to be checked.
    ///
    /// # Returns
    ///
    /// Returns `true` if the `item` matches the `field`, otherwise returns `false`.
    fn is_match(&self, item: i32) -> bool {
        self.field == item
    }
}

/// Filters the elements in the collection based on the given condition.
///
/// # Arguments
///
/// * `collection` - The collection of elements to be filtered.
/// * `condition` - The condition used to filter the elements.
///
/// # Returns
///
/// A new vector containing the filtered elements.
fn custom_filter_with_closures(collection: &[i32], condition: &FilterCondition) -> Vec<i32> {
    collection
        .iter()
        .filter(|&&item| condition.is_match(item))
        .copied()
        .collect()
}

/// Filters the elements in the collection based on the given condition.
///
/// # Arguments
///
/// * `collection` - The collection of elements to be filtered.
/// * `condition` - The condition used to filter the elements.
///
/// # Returns
///
/// A new vector containing the filtered elements.
fn custom_filter(collection: &[i32], condition: &FilterCondition) -> Vec<i32> {
    let mut filtered_collection = vec![];
    for item in collection {
        if condition.is_match(*item) {
            filtered_collection.push(*item);
        }
    }

    filtered_collection
}

fn main() {
    let collection = vec![1, 2, 3];
    let filter_condition = FilterCondition { field: 2 };

    let filtered_collection = custom_filter(&collection, &filter_condition);
    println!("filtered collection: {:?}", filtered_collection);

    let filtered_collection = custom_filter_with_closures(&collection, &filter_condition);
    println!("filtered collection: {:?}", filtered_collection);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_filter() {
        let collection = vec![1, 2, 3];
        let filter_condition = FilterCondition { field: 3 };

        let result = custom_filter(&collection, &filter_condition);

        assert_eq!(result, vec![3]);
    }

    #[test]
    fn test_custom_filter_with_closures() {
        let collection = vec![1, 2, 3];
        let filter_condition = FilterCondition { field: 3 };

        let result = custom_filter_with_closures(&collection, &filter_condition);

        assert_eq!(result, vec![3]);
    }
}
