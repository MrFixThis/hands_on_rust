use std::marker::PhantomData;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Object {
    name: String,
    frags: usize,
    value: f64,
    weight: f64,
    ratio: f64,
}

impl Object {
    pub fn new(name: String, frags: usize, value: f64, weight: f64) -> Self {
        if value == 0.0 || weight == 0.0 {
            panic!("neither the value nor the weight of an object can be 0");
        }

        if frags == 0 {
            panic!("objects must have at least one fragment");
        }

        let ratio = value / weight;
        Self {
            name,
            frags,
            value,
            weight,
            ratio,
        }
    }
}

// BackPack states
#[derive(Debug)]
pub struct Empty;
#[derive(Debug)]
pub struct Filled;

#[derive(Debug)]
pub struct BackPack<State = Empty> {
    objs: Vec<Object>,
    max_cap: f64,
    used_cap: f64,
    _state: PhantomData<State>,
}

impl From<BackPack<Empty>> for BackPack<Filled> {
    fn from(value: BackPack<Empty>) -> Self {
        BackPack::<Filled> {
            objs: value.objs,
            max_cap: value.max_cap,
            used_cap: value.used_cap,
            _state: PhantomData,
        }
    }
}

impl BackPack {
    pub fn new(max_cap: f64) -> Self {
        Self {
            objs: Vec::new(),
            max_cap,
            used_cap: 0.0,
            _state: PhantomData,
        }
    }

    pub async fn insert_objs(mut self, mut objs: Vec<Object>) -> BackPack<Filled> {
        objs.sort_by(|a, b| b.ratio.partial_cmp(&a.ratio).unwrap());

        let mut rem_cap = self.max_cap;
        for x in objs {
            match rem_cap.min(x.weight) {
                weight if weight == x.weight => {
                    rem_cap -= x.weight;
                    self.used_cap += x.weight;
                    self.objs.push(x);
                }
                _ => continue,
            }
        }

        self.into()
    }

    pub async fn insert_objs_frag(mut self, mut objs: Vec<Object>) -> BackPack<Filled> {
        objs.sort_by(|a, b| b.ratio.partial_cmp(&a.ratio).unwrap());

        let mut rem_cap = self.max_cap;
        'objs_iter: for mut x in objs {
            match rem_cap.min(x.weight) {
                weight if weight != rem_cap => {}
                _ => {
                    let val_per_frag = x.value / x.frags as f64;
                    let weight_per_frag = x.weight / x.frags as f64;
                    loop {
                        if x.frags == 0 {
                            continue 'objs_iter;
                        }

                        if x.weight > rem_cap {
                            x.frags -= 1;
                            x.value -= val_per_frag;
                            x.weight -= weight_per_frag;
                            x.ratio = x.value / x.weight;
                        } else {
                            break;
                        }
                    }
                }
            }

            rem_cap -= x.weight;
            self.used_cap += x.weight;
            self.objs.push(x);
        }

        self.into()
    }
}

impl BackPack<Filled> {
    pub fn cal_revenue(&self) -> f64 {
        self.objs.iter().map(|x| x.value).sum()
    }
}

pub fn show_backpack_content(backpack: BackPack<Filled>) {
    println!(
        "{}",
        format!(
            "[BackPack Info]:\n \
            - Max capacity: {:.1} kg\n \
            - Capacity used: {:.1} kg\n \
            - Obtained revenue: ${:.1}\n \
            - Ojects: {:#?}",
            backpack.max_cap,
            backpack.used_cap,
            backpack.cal_revenue(),
            backpack.objs
        )
    );
}
