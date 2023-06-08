use std::marker::PhantomData;

use super::{Pendent, Ready, Report};

#[derive(Debug)]
pub struct MenuOptimizer<State = Pendent> {
    optimal_menu: Option<Vec<(String, u32)>>,
    _min_diff: u32,
    _marker: PhantomData<State>,
}

impl Report for MenuOptimizer<Ready> {
    fn report(&self) -> String {
        match self.optimal_menu {
            Some(ref menu) => {
                let mut dishes = String::new();
                let total_cals: u32 = menu.iter().map(|&(_, c)| c).sum();

                dishes.push_str("> Optimal menu found:\n");
                menu.iter().enumerate().for_each(|(i, (n, c))| {
                    dishes.extend(format!("  {}: {n} -> {c} calories\n", i + 1).chars())
                });
                dishes.extend(format!("#> Total calories: {total_cals}").chars());
                dishes
            }
            None => "> It was not possible to find an optimal menu for the \
                    target calories specified."
                .to_owned(),
        }
    }
}

impl From<MenuOptimizer<Pendent>> for MenuOptimizer<Ready> {
    fn from(mo: MenuOptimizer<Pendent>) -> Self {
        MenuOptimizer::<Ready> {
            optimal_menu: mo.optimal_menu,
            _min_diff: mo._min_diff,
            _marker: PhantomData,
        }
    }
}

impl MenuOptimizer {
    pub fn new() -> Self {
        Self {
            optimal_menu: None,
            _min_diff: u32::MAX,
            _marker: PhantomData,
        }
    }

    pub fn find_optimal_menu(
        mut self,
        target_calories: u32,
        base_menu: Vec<(String, u32)>,
    ) -> MenuOptimizer<Ready> {
        let mut curr_menu: Vec<(String, u32)> = Vec::new();
        self.backtrack(target_calories, &base_menu, 0, 0, &mut curr_menu);
        self.into()
    }

    fn backtrack(
        &mut self,
        target_calories: u32,
        base_menu: &Vec<(String, u32)>,
        entry: usize,
        curr_cals: u32,
        curr_menu: &mut Vec<(String, u32)>,
    ) {
        if curr_cals >= target_calories && curr_cals - target_calories < self._min_diff {
            self._min_diff = curr_cals - target_calories;
            self.optimal_menu.replace(curr_menu.clone());
        }

        (entry..base_menu.len()).for_each(|i| {
            if let Some(dish) = base_menu.get(i) {
                curr_menu.push(dish.clone());
                self.backtrack(
                    target_calories,
                    base_menu,
                    i + 1,
                    curr_cals + dish.1,
                    curr_menu,
                );
                curr_menu.pop();
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_optimal_menu_is_found_with_coherent_target_calories() {
        let mo = MenuOptimizer::new();
        let target_calories = 1000;
        let base_menu: Vec<(String, u32)> = vec![
            ("Chicken".to_owned(), 300),
            ("Salad".to_owned(), 200),
            ("Soup".to_owned(), 150),
            ("WaterMelon".to_owned(), 80),
            ("Apple".to_owned(), 70),
            ("Fish".to_owned(), 400),
        ];

        let mor = mo.find_optimal_menu(target_calories, base_menu);
        assert_eq!(
            mor.optimal_menu,
            Some(vec![
                ("Chicken".to_owned(), 300),
                ("Soup".to_owned(), 150),
                ("WaterMelon".to_owned(), 80),
                ("Apple".to_owned(), 70),
                ("Fish".to_owned(), 400),
            ])
        )
    }

    #[test]
    fn test_optimal_menu_is_not_found_with_unbalanced_target_calories() {
        let mo = MenuOptimizer::new();
        let target_calories = 9999;
        let base_menu: Vec<(String, u32)> = vec![
            ("Chicken".to_owned(), 300),
            ("Salad".to_owned(), 200),
            ("Soup".to_owned(), 150),
            ("WaterMelon".to_owned(), 80),
            ("Apple".to_owned(), 70),
            ("Fish".to_owned(), 400),
        ];

        let mor = mo.find_optimal_menu(target_calories, base_menu);
        assert_eq!(mor.optimal_menu, None)
    }
}
