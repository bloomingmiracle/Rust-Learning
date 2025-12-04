use crate::models::product::Product;
use crate::models::purchase::Purchase;

#[derive(Debug, Default)]
pub struct Manager {
    products: Vec<Product>,
    purchases: Vec<Purchase>,
}

impl Manager {
    pub fn new() -> Self {
        Self {
            products: Vec::new(),
            purchases: Vec::new(),
        }
    }

    // --------------------------------------------------
    // PRODUCT MANAGEMENT
    // --------------------------------------------------

    pub fn add_product(
        &mut self,
        name: String,
        unit: String,
        planned_quantity: u32,
        planned_price: f64,
    ) {
        let p = Product::new(name, unit, planned_quantity, planned_price);
        self.products.push(p);
    }

    pub fn update_planned_price(&mut self, name: &str, new_price: f64) -> bool {
        for p in &mut self.products {
            if p.name == name {
                p.planned_price = new_price;
                return true;
            }
        }
        false
    }

    pub fn find_product(&self, name: &str) -> Option<&Product> {
        self.products.iter().find(|p| p.name == name)
    }

    pub fn list_products(&self) {
        println!(
            "{:<15} {:<10} {:<15} {:<15}",
            "Name", "Unit", "Planned Qty", "Planned Price"
        );

        for p in &self.products {
            println!(
                "{:<15} {:<10} {:<15} {:<15.2}",
                p.name, p.unit, p.planned_quantity, p.planned_price
            );
        }
    }

    // --------------------------------------------------
    // PURCHASE MANAGEMENT
    // --------------------------------------------------

    pub fn add_purchase(
        &mut self,
        month: String,
        product: String,
        qty: u32,
        price: f64,
        supermarket: String,
    ) -> Result<(), String> {
        if self.find_product(&product).is_none() {
            return Err(format!("Product '{}' does not exist", product));
        }

        let p = Purchase::new(
            month,
            product,
            qty as u32,
            price,
            supermarket,
        );
        self.purchases.push(p);
        Ok(())
    }

    pub fn list_purchases(&self) {
        println!(
            "{:<10} {:<15} {:<10} {:<10} {:<15}",
            "Month", "Product", "Qty", "Unit Price", "Supermarket"
        );

        for p in &self.purchases {
            println!(
                "{:<10} {:<15} {:<10} {:<10.2} {:<15}",
                p.month, p.product_name, p.quantity_bought, p.unit_price, p.supermarket
            );
        }
    }

    // --------------------------------------------------
    // REPORTS & CALCULATIONS
    // --------------------------------------------------

    pub fn total_for_month(&self, month: &str) -> f64 {
        self.purchases
            .iter()
            .filter(|p| p.month == month)
            .map(|p| p.unit_price * p.quantity_bought as f64)
            .sum()
    }

    pub fn lowest_price_details(&self, product: &str) -> String {
        let mut filtered: Vec<&Purchase> = self
            .purchases
            .iter()
            .filter(|p| p.product_name == product)
            .collect();

        if filtered.is_empty() {
            return format!("No purchases found for '{}'.", product);
        }

        filtered.sort_by(|a, b| a.unit_price.partial_cmp(&b.unit_price).unwrap());

        let p = filtered[0];
        format!(
            "Lowest price for '{}': R$ {:.2} at {} ({})",
            product, p.unit_price, p.supermarket, p.month
        )
    }

    pub fn highest_price_details(&self, product: &str) -> String {
        let mut filtered: Vec<&Purchase> = self
            .purchases
            .iter()
            .filter(|p| p.product_name == product)
            .collect();

        if filtered.is_empty() {
            return format!("No purchases found for '{}'.", product);
        }

        filtered.sort_by(|a, b| b.unit_price.partial_cmp(&a.unit_price).unwrap());

        let p = filtered[0];
        format!(
            "Highest price for '{}': R$ {:.2} at {} ({})",
            product, p.unit_price, p.supermarket, p.month
        )
    }

    pub fn cheapest_supermarket_in_month(&self, month: &str) -> String {
        let mut totals: std::collections::HashMap<String, f64> = std::collections::HashMap::new();

        for p in &self.purchases {
            if p.month == month {
                let entry = totals.entry(p.supermarket.clone()).or_insert(0.0);
                *entry += p.unit_price * p.quantity_bought as f64;
            }
        }

        if totals.is_empty() {
            return "No purchases in this month.".to_string();
        }

        totals
            .into_iter()
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .map(|(m, _)| m)
            .unwrap()
    }

    pub fn most_expensive_supermarket_in_month(&self, month: &str) -> String {
        let mut totals: std::collections::HashMap<String, f64> = std::collections::HashMap::new();

        for p in &self.purchases {
            if p.month == month {
                let entry = totals.entry(p.supermarket.clone()).or_insert(0.0);
                *entry += p.unit_price * p.quantity_bought as f64;
            }
        }

        if totals.is_empty() {
            return "No purchases in this month.".to_string();
        }

        totals
            .into_iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .map(|(m, _)| m)
            .unwrap()
    }

    pub fn compare_planned_price(&self, product: &str, month: &str) {
        let product_data = match self.find_product(product) {
            Some(p) => p,
            None => {
                println!("Product not found.");
                return;
            }
        };

        let month_total: f64 = self
            .purchases
            .iter()
            .filter(|p| p.product_name == product && p.month == month)
            .map(|p| p.unit_price * p.quantity_bought as f64)
            .sum();

        println!("Planned price for '{}': R$ {:.2}", product, product_data.planned_price);
        println!("Total spent in {}: R$ {:.2}", month, month_total);

        if month_total > product_data.planned_price {
            println!("⚠ You spent MORE than planned.");
        } else {
            println!("✔ You spent within the planned price!");
        }
    }
}

