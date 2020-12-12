pub(super) fn find_chain(adapters: &[i32]) -> i32 {
    if adapters.is_empty() {
        return 0;
    }

    let mut diff_1_jolt = 0;
    let mut diff_3_jolts = 1;
    let mut last_adapter = 0;

    for adapter in adapters {
        if *adapter == last_adapter + 1 {
            diff_1_jolt += 1;
        }
        if *adapter == last_adapter + 3 {
            diff_3_jolts += 1;
        }

        last_adapter = *adapter;
    }

    return diff_1_jolt * diff_3_jolts;
}

#[cfg(test)]
mod tests {
    use hamcrest2::prelude::*;

    use super::*;

    #[test]
    fn find_chain_returns_0_for_empty_list() {
        let adapters = vec![];

        let result = find_chain(&adapters);

        assert_that!(result, equal_to(0));
    }

    #[test]
    fn find_chain_works_with_first_example() {
        let mut adapters = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        adapters.sort();

        let result = find_chain(&adapters);

        assert_that!(result, equal_to(7 * 5));
    }

    #[test]
    fn find_chain_works_with_second_example() {
        let mut adapters = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        adapters.sort();

        let result = find_chain(&adapters);

        assert_that!(result, equal_to(22 * 10));
    }
}
