#[derive(Debug, Clone)]
pub struct Purchase {
    pub month: String,
    pub product_name: String,
    pub quantity_bought: u32,
    pub unit_price: f64,
    pub supermarket: String,
}

impl Purchase {
    pub fn new(
        month: String,
        product_name: String,
        quantity_bought: u32,
        unit_price: f64,
        supermarket: String,
    ) -> Self {
        Self {
            month,
            product_name,
            quantity_bought,
            unit_price,
            supermarket,
        }
    }
}

