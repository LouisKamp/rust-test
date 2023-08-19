use std::collections::HashMap;

fn main_math() {
    let x = Var { name: "x" };
    let vars = HashMap::from([("x".to_owned(), 10.0)]);

    let a = Add {
        rhs: Add { rhs: x, lhs: 4.0 },
        lhs: 1.0,
    };

    let b = a.calc(&Some(vars));
    print!("The result is: {}", b);
}

trait Operation {
    type Output: Operation;

    fn calc(&self, vars: &Option<HashMap<String, f64>>) -> f64;
    fn diff(&self) -> Self::Output;
}

impl Operation for f64 {
    type Output = f64;

    fn calc(&self, _: &Option<HashMap<String, f64>>) -> f64 {
        *self
    }

    fn diff(&self) -> Self::Output {
        0.0
    }
}

struct Var {
    name: &'static str,
}

impl Operation for Var {
    type Output = Var;

    fn calc(&self, vars: &Option<HashMap<String, f64>>) -> f64 {
        if let Some(var_map) = &vars {
            if let Some(value) = var_map.get(self.name) {
                return *value;
            }
        }
        panic!("No value provided for variable {}", self.name);
    }

    fn diff(&self) -> Self::Output {
        todo!()
    }
}

struct Add<RHS, LHS>
where
    RHS: Operation,
    LHS: Operation,
{
    rhs: RHS,
    lhs: LHS,
}

impl<RHS, LHS> Operation for Add<RHS, LHS>
where
    RHS: Operation,
    LHS: Operation,
{
    type Output = Add<RHS::Output, LHS::Output>;

    fn calc(&self, vars: &Option<HashMap<String, f64>>) -> f64 {
        self.rhs.calc(&vars) + self.lhs.calc(&vars)
    }

    fn diff(&self) -> Self::Output {
        Add {
            rhs: self.rhs.diff(),
            lhs: self.lhs.diff(),
        }
    }
}
