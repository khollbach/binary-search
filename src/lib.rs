pub mod rec_excl {
    use std::cmp::Ordering::*;

    pub fn binary_search(nums: &[i32], target: i32) -> Option<usize> {
        if nums.is_empty() {
            return None;
        }

        let mid = nums.len() / 2;

        match target.cmp(&nums[mid]) {
            Equal => Some(mid),
            Less => binary_search(&nums[..mid], target),
            Greater => binary_search(&nums[mid + 1..], target).map(|i| i + mid + 1),
        }
    }
}

pub mod tailrec_excl {
    use std::cmp::Ordering::*;

    pub fn binary_search(nums: &[i32], target: i32) -> Option<usize> {
        bs(nums, 0, nums.len(), target)
    }

    fn bs(nums: &[i32], i: usize, j: usize, target: i32) -> Option<usize> {
        if j <= i {
            return None;
        }

        let mid = (i + j) / 2;

        match target.cmp(&nums[mid]) {
            Equal => Some(mid),
            Less => bs(nums, i, mid, target),
            Greater => bs(nums, mid + 1, j, target),
        }
    }
}

pub mod tailrec_incl {
    use std::cmp::Ordering::*;

    pub fn binary_search(nums: &[i32], target: i32) -> Option<usize> {
        bs(nums, 0, nums.len().checked_sub(1)?, target)
    }

    // inclusive
    fn bs(nums: &[i32], i: usize, j: usize, target: i32) -> Option<usize> {
        if j < i {
            return None;
        }

        let mid = (i + j) / 2;

        match target.cmp(&nums[mid]) {
            Equal => Some(mid),
            Less => bs(nums, i, mid.checked_sub(1)?, target),
            Greater => bs(nums, mid + 1, j, target),
        }
    }
}

pub mod iter_excl {
    use std::cmp::Ordering::*;

    pub fn binary_search(nums: &[i32], target: i32) -> Option<usize> {
        let mut i = 0;
        let mut j = nums.len();

        while i < j {
            let mid = (i + j) / 2;

            match target.cmp(&nums[mid]) {
                Equal => return Some(mid),
                Less => j = mid,
                Greater => i = mid + 1,
            }
        }

        None
    }
}

pub mod iter_incl {
    use std::cmp::Ordering::*;

    pub fn binary_search(nums: &[i32], target: i32) -> Option<usize> {
        // incl
        let mut i = 0;
        let mut j = nums.len().checked_sub(1)?;

        while i <= j {
            let mid = (i + j) / 2;

            match target.cmp(&nums[mid]) {
                Equal => return Some(mid),
                Less => j = mid.checked_sub(1)?,
                Greater => i = mid + 1,
            }
        }

        None
    }
}

#[cfg(test)]
const ALL_IMPLS: [fn(&[i32], i32) -> Option<usize>; 5] = [
    rec_excl::binary_search,
    tailrec_excl::binary_search,
    tailrec_incl::binary_search,
    iter_excl::binary_search,
    iter_incl::binary_search,
];

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(&[], 5, None)]
    #[test_case(&[0,1,2,3,4], -1, None)]
    #[test_case(&[0,1,2,3,4], 0, Some(0))]
    #[test_case(&[0,1,2,3,4], 1, Some(1))]
    #[test_case(&[0,1,2,3,4], 2, Some(2))]
    #[test_case(&[0,1,2,3,4], 3, Some(3))]
    #[test_case(&[0,1,2,3,4], 4, Some(4))]
    #[test_case(&[0,1,2,3,4], 5, None)]
    #[test_case(&[5,5,5,5,5], 5, Some(2))]
    #[test_case(&[5,5,5,5,5], 4, None)]
    #[test_case(&[5,5,5,5,5], 6, None)]
    pub fn binary_search(nums: &[i32], target: i32, expected: Option<usize>) {
        for f in ALL_IMPLS {
            let actual = f(&nums, target);
            assert_eq!(actual, expected);
        }
    }
}

#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;
    use proptest_derive::Arbitrary;

    /// helper for sorted_array
    #[derive(Debug, Arbitrary)]
    enum Dna {
        Empty,
        NonEmpty { base: i16, deltas: Vec<u16> },
    }

    /// helper for generating inputs to binary_search
    fn sorted_array(dna: Dna) -> Vec<i32> {
        let Dna::NonEmpty { base, deltas } = dna else {
            return vec![];
        };

        let mut out = Vec::with_capacity(1 + deltas.len());

        let mut total: i32 = base.into();
        out.push(total);

        for d in deltas {
            total = total.checked_add(d.into()).unwrap();
            out.push(total);
        }

        out
    }

    proptest! {
        #[test]
        fn binary_search(dna: Dna, target: i32) {
            let nums = sorted_array(dna);

            for f in ALL_IMPLS {
                match f(&nums, target) {
                    Some(idx) => prop_assert_eq!(nums[idx], target),
                    None => prop_assert!(!nums.contains(&target)),
                }
            }
        }
    }

    /// tests for the tests
    mod tests {
        use super::*;
        use test_case::test_case;

        #[test_case(Dna::Empty, vec![])]
        #[test_case(Dna::NonEmpty { base: -5, deltas: vec![] }, vec![-5])]
        #[test_case(Dna::NonEmpty { base: -5, deltas: vec![0, 10, 10, 0] }, vec![-5, -5, 5, 15, 15])]
        fn sorted_array(dna: Dna, expected: Vec<i32>) {
            assert_eq!(super::sorted_array(dna), expected);
        }
    }
}
