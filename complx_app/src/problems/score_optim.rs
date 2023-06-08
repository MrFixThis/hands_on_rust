use std::marker::PhantomData;

use super::{Pendent, Ready, Report};

#[derive(Debug)]
pub struct ScoreOptimizer<State = Pendent> {
    max_assigned: Vec<usize>,
    max_score: isize,
    _num_teams: usize,
    _num_arbiters: usize,
    _preferences: Vec<Vec<isize>>,
    _marker: PhantomData<State>,
}

impl Report for ScoreOptimizer<Ready> {
    fn report(&self) -> String {
        if self.max_assigned.is_empty() {
            return "None of the arbiters was assigned to any match.".to_owned();
        }

        let mut res = String::new();
        res.push_str("> Score information:\n");
        res.extend(
            format!(
                "# Maximum score: {}\n- Assigned arbiters: [ {} ]",
                self.max_score,
                self.max_assigned
                    .iter()
                    .map(ToString::to_string)
                    .intersperse(", ".to_owned())
                    .collect::<String>()
            )
            .chars(),
        );
        res
    }
}

impl From<ScoreOptimizer<Pendent>> for ScoreOptimizer<Ready> {
    fn from(so: ScoreOptimizer<Pendent>) -> Self {
        Self {
            max_assigned: so.max_assigned,
            max_score: so.max_score,
            _num_teams: so._num_teams,
            _num_arbiters: so._num_arbiters,
            _preferences: so._preferences,
            _marker: PhantomData,
        }
    }
}

impl ScoreOptimizer {
    pub fn build(preferences: Vec<Vec<isize>>) -> Result<Self, &'static str> {
        let num_teams = preferences.len();
        let num_arbiters = preferences[0].len();

        if num_teams & 1 != 0 {
            return Err("The number of teams must be even.");
        }

        for a in &preferences {
            if a.len() != num_arbiters {
                return Err("The number of arbiters must be the same.");
            }
        }

        if num_teams / 2 > num_arbiters {
            return Err("The number of arbiters must be greater than the number of matches.");
        }

        Ok(Self {
            max_assigned: vec![0; num_arbiters],
            max_score: isize::MIN,
            _num_teams: num_teams,
            _num_arbiters: num_arbiters,
            _preferences: preferences,
            _marker: PhantomData,
        })
    }

    pub fn find_optimal_assigment(mut self) -> ScoreOptimizer<Ready> {
        let mut curr_assigned: Vec<usize> = vec![0; self._num_arbiters];
        self.backtrack(&mut curr_assigned, 0, 0);
        self.into()
    }

    pub fn backtrack(
        &mut self,
        curr_assigned: &mut Vec<usize>,
        curr_score: isize,
        curr_match: usize,
    ) {
        if curr_match == self._num_teams / 2 {
            if curr_score > self.max_score {
                self.max_score = curr_score;
                self.max_assigned = curr_assigned.clone();
            }
            return;
        }

        for arbiter in 0..self._num_arbiters {
            let mut valid_assignment = true;

            for team in 0..self._num_teams {
                if self._preferences[team][arbiter] == -isize::MAX {
                    valid_assignment = false;
                    break;
                }

                for i in 0..curr_match {
                    if curr_assigned[i * 2] == team
                        || curr_assigned[i * 2 + 1] == team
                            && self._preferences[team][arbiter]
                                > self._preferences[team][curr_assigned[i * 2]]
                        || self._preferences[team][arbiter]
                            > self._preferences[team][curr_assigned[i * 2 + 1]]
                    {
                        valid_assignment = false;
                        break;
                    }
                }

                if !valid_assignment {
                    break;
                }
            }

            if valid_assignment {
                curr_assigned[curr_match * 2] = arbiter;

                for team in 0..self._num_teams {
                    if self._preferences[team][arbiter] != -isize::MAX {
                        curr_assigned[curr_match * 2 + 1] = team;
                        self.backtrack(
                            curr_assigned,
                            curr_score + self._preferences[team][arbiter],
                            curr_match + 1,
                        );
                    }
                }

                curr_assigned[curr_match * 2] = 0;
                curr_assigned[curr_match * 2 + 1] = 0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_optimal_assigment_is_made_properly() {
        let preferences: Vec<Vec<isize>> = vec![
            vec![10, 8, 6, 4, 2],
            vec![2, 4, 6, 8, 10],
            vec![4, 6, 2, 10, 8],
            vec![8, 10, 4, 6, 2],
        ];

        let sor = ScoreOptimizer::build(preferences)
            .unwrap()
            .find_optimal_assigment();

        assert_eq!(sor.max_score, 20);
        assert_eq!(sor.max_assigned, vec![4, 1, 1, 3, 0]);
    }
}
