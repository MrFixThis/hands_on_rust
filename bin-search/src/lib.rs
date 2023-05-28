mod binding;

pub fn bin_search<T>(col: &[T], target: T) -> Option<(usize, &T)>
where
    T: PartialEq + PartialOrd
{
    let (start, end) = (0, col.len() - 1);
    search(col, target, start, end)
}

fn search<T>(col: &[T], target: T, start: usize, end: usize) -> Option<(usize, &T)>
where
    T: PartialEq + PartialOrd
{
    if start > end {
        return None;
    }

    let middle = (start + end) / 2;
    let middle_item = col.get(middle).unwrap();

    if *middle_item == target {
        return Some((middle, middle_item));
    }

    if *middle_item < target {
        search(col, target, middle + 1, end)
    } else {
        search(col, target, start, middle - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let col_test = vec![1, 2, 3, 4, 5, 6];
        let res = bin_search(&col_test, 1);
        assert_eq!(res, Some((0, &1)))
    }
}
