#[derive(Debug, Clone)]
pub struct Product {
    pub name: String,
    pub unit: String,
    pub planned_quantity: u32,
    pub planned_price: f64,
}

impl Product {
    pub fn new(
        name: String,
        unit: String,
        planned_quantity: u32,
        planned_price: f64,
    ) -> Self {
        Self {
            name,
            unit,
            planned_quantity,
            planned_price,
        }
    }
}

