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
