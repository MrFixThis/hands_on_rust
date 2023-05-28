use std::{collections::HashMap, fmt::Display};

/// A complete `report` that contains some statistical data related to an analyzing
/// collection of numeric elements.
///
/// This information includes the following:
/// - Mode
/// - Median
/// - Mean
/// - Maximim value
#[derive(Debug, Default, PartialEq)]
pub struct Report {
    mode: usize,
    median: f32,
    mean: f32,
    max: usize
}

impl Report {
    /// Determines the `mode`, `median`, and `mean` of a collection of
    /// [`std::usize`] elements.
    ///
    /// On the other hand, it also determines the `maximum` value in the collection.
    pub fn from(coll: Vec<usize>) -> Self {
        // if the collection is empty, return the default value
        if coll.is_empty() { return Self::default(); }

        let mut iter = coll.iter();
        let mut max = *iter.next().unwrap();
        let mut avrg_sum = max as f32;
        iter.for_each(|&x| {
            // finding the max value
            if x > max { max = x; }
            // caculating the mean's sum
            avrg_sum += x as f32;
        });

        Self {
            mode: Self::find_mode(&coll),
            median: Self::find_median(&coll),
            mean: avrg_sum / coll.len() as f32,
            max,
        }
    }

    /// Determines the `mode` of the specified collection of numeric elements.
    fn find_mode(coll: &[usize]) -> usize {
        //to find the mode, lets iterate over all of the elements of the collection
        //and compare them each other.
        let mut sum = HashMap::new();
        // counting the elements inside the collection
        coll
            .iter()
            .for_each(|&x| *sum.entry(x).or_insert(0) += 1);
        // getting the mode
        sum
            .into_iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .unwrap().0
    }

    /// Determines the `median` of the specified collection of numeric elements.
    fn find_median(coll: &[usize]) -> f32 {
        // this closure determines the median value
        let len = coll.len();
        if len % 2 != 0 {
            *coll.get(len / 2).unwrap() as f32
        } else {
            (
                *coll.get(len / 2).unwrap() as f32 +
                *coll.get((len / 2) - 1).unwrap() as f32
            ) / 2.0
        }
    }
}

impl Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
r#"[Report]
    - Mode: {}
    - Median: {}
    - Mean: {}
    - Maximum: {}
"#, self.mode, self.median, self.mean, self.max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_default() {
        let vec = Vec::new();
        let rep = Report::from(vec);

        assert_eq!(rep, Report { mode: 0, median: 0.0, mean: 0.0, max: 0 })
    }

    #[test]
    fn should_return_report_fullfilled_from_uneven_lengthed_collection() {
        let vec = vec![1, 4, 5, 4, 7];
        let rep = Report::from(vec);

        assert_eq!(rep, Report { mode: 4, median: 5.0, mean: 4.2, max: 7 })
    }

    #[test]
    fn should_return_report_fullfilled_from_even_lengthed_collection() {
        let vec = vec![1, 7, 23, 16, 1, 10];
        let rep = Report::from(vec);

        assert_eq!(rep, Report { mode: 1, median: 19.5, mean: 9.666667, max: 23 })
    }
}
