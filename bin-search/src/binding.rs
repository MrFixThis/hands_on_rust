use wasm_bindgen::{prelude::wasm_bindgen, JsValue, UnwrapThrowExt};

#[wasm_bindgen]
extern "C" {
    fn alert(msg: &str);
    fn prompt(msg: &str, def: &str) -> String;
}

#[wasm_bindgen(start)]
pub fn search_report() -> Result<(), JsValue> {
    loop {
        let col: Vec<usize> = prompt("Specify your collection separated by commas:", "1,2,3")
            .split(',')
            .map(|n| n.parse::<usize>().unwrap_throw())
            .collect();

        let target: usize = prompt("Enter your target:", "2")
            .parse()
            .unwrap_throw();

        let result: String = match crate::bin_search(&col, target) {
            Some((i, v)) => format!("Value {v}, found at index {i}"),
            None => format!("Value {target} not found.")
        };

        alert(&result);
    }
}
