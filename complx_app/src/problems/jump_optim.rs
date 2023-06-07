use std::marker::PhantomData;

use super::{Pendent, Ready, Report};

#[derive(Debug)]
pub struct JumpOptimizer<State = Pendent> {
    min_jums: Option<usize>,
    _field_size: (usize, usize),
    _jump_length: (usize, usize),
    _visited: Vec<Vec<bool>>,
    _marker: PhantomData<State>,
}

impl Report for JumpOptimizer<Ready> {
    fn report(&self) -> String {
        match self.min_jums {
            Some(jumps) => {
                format!(
                    "> The minimum number of jumps done to go from point `A` to point `B` is {jumps}.",
                )
            },
            None => "> It was not possible to get from point `A` to point `B`.".to_owned(),
        }
    }
}

impl From<JumpOptimizer<Pendent>> for JumpOptimizer<Ready> {
    fn from(jo: JumpOptimizer<Pendent>) -> Self {
        JumpOptimizer::<Ready> {
            min_jums: jo.min_jums,
            _field_size: jo._field_size,
            _jump_length: jo._jump_length,
            _visited: jo._visited,
            _marker: PhantomData,
        }
    }
}

impl JumpOptimizer {
    pub fn new(field_size: (usize, usize), jump_length: (usize, usize)) -> Self {
        Self {
            min_jums: None,
            _field_size: field_size,
            _jump_length: jump_length,
            _visited: vec![vec![false; field_size.1]; field_size.0],
            _marker: PhantomData,
        }
    }

    pub fn find_min_jumps(
        mut self,
        start_point: (usize, usize),
        target_point: (usize, usize),
    ) -> Result<JumpOptimizer<Ready>, &'static str> {
        if target_point.0 > self._field_size.0 || target_point.1 > self._field_size.1 {
            return Err("The rabbit cannot jump outside the farm field!");
        }

        self._visited[start_point.0][start_point.1] = true;
        self.backtrack(&start_point, &target_point, 0);
        self._visited[start_point.0][start_point.1] = false;
        Ok(self.into())
    }

    fn backtrack(
        &mut self,
        current_point: &(usize, usize),
        target_point: &(usize, usize),
        curr_jums: usize,
    ) {
        // if destination reached
        if current_point.0 == target_point.0 && current_point.1 == target_point.1 {
            self.min_jums.replace(self.min_jums.map_or(curr_jums, |m| m.min(curr_jums)));
            return;
        }

        // if the current number of jumps is greater than the last one
        if self.min_jums.is_some_and(|j| curr_jums >= j) {
            return;
        }

        self.determine_movements().into_iter().for_each(|(x, y)| {
            let next_point = (current_point.0 as isize + x, current_point.1 as isize + y);
            if self.is_valid_move(&next_point) {
                let next_point = (next_point.0 as usize, next_point.1 as usize);
                self._visited[next_point.0][next_point.1] = true;
                self.backtrack(&next_point, target_point, curr_jums + 1);
                self._visited[next_point.0][next_point.1] = false;
            }
        });
    }

    #[rustfmt::skip]
    fn determine_movements(&self) -> Vec<(isize, isize)> {
        let (p, q) = (self._jump_length.0 as isize, self._jump_length.1 as isize);
        vec![
            ( p, q ), ( p, -q ), ( -p, q ), ( -p, -q ),
            ( q, p ), ( q, -p ), ( -q, p ), ( -q, -p )
        ]
    }

    fn is_valid_move(&self, point: &(isize, isize)) -> bool {
        let &(n, m) = &(self._field_size.0 as isize, self._field_size.1 as isize);
        let &(x, y) = point;
        x >= 0 && x < m && y >= 0 && y < n && !self._visited[x as usize][y as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_rabbit_get_from_a_to_b_properly() {
        let field_size = (10, 10);
        let jump_length = (1, 1);
        let start_point = (0, 0);
        let target_point = (2, 2);

        let jor =
            JumpOptimizer::new(field_size, jump_length).find_min_jumps(start_point, target_point);

        assert_eq!(jor.unwrap().min_jums, Some(2));
    }

    #[test]
    fn test_rabbit_cannot_reach_b_from_a() {
        let field_size = (5, 5);
        let jump_length = (3, 3);
        let start_point = (0, 0);
        let target_point = (5, 5);

        let jor =
            JumpOptimizer::new(field_size, jump_length).find_min_jumps(start_point, target_point);

        assert!(jor.unwrap().min_jums.is_none());
    }

    #[test]
    fn test_rabbit_jump_outside_bounds_is_warned() {
        let field_size = (5, 5);
        let jump_length = (3, 3);
        let start_point = (0, 0);
        let target_point = (6, 7);

        let jor =
            JumpOptimizer::new(field_size, jump_length).find_min_jumps(start_point, target_point);

        assert!(jor.is_err());
    }
}
