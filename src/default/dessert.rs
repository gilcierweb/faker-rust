//! Dessert generator - generates dessert-related data

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random dessert variety
pub fn variety() -> String {
    fetch_locale("dessert.varieties", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_VARIETIES).to_string())
}

/// Generate a random dessert topping
pub fn topping() -> String {
    fetch_locale("dessert.toppings", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_TOPPINGS).to_string())
}

/// Generate a random dessert flavor
pub fn flavor() -> String {
    fetch_locale("dessert.flavors", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_FLAVORS).to_string())
}

// Fallback data
const FALLBACK_VARIETIES: &[&str] = &[
    "Cake", "Pie", "Ice Cream", "Pudding", "Mousse", "Tart", "Crumble",
    "Brownie", "Cookie", "Donut", "Cupcake", "Cheesecake", "Parfait",
    "Trifle", "Sorbet", "Gelato", "Custard", "Flan", "Macaron",
    "Eclair", "Profiterole", "Tiramisu", "Pavlova", "Baklava", "Cannoli",
];

const FALLBACK_TOPPINGS: &[&str] = &[
    "Whipped Cream", "Chocolate Chips", "Sprinkles", "Caramel Sauce",
    "Strawberries", "Blueberries", "Raspberries", "Nuts", "Marshmallows",
    "Cherries", "Oreo Crumbs", "Peanut Butter", "Honey", "Coconut Flakes",
    "Hot Fudge", "Butterscotch", "Cinnamon", "Powdered Sugar", "Fruit",
];

const FALLBACK_FLAVORS: &[&str] = &[
    "Vanilla", "Chocolate", "Strawberry", "Mint", "Cookies and Cream",
    "Salted Caramel", "Coffee", "Lemon", "Raspberry", "Mango", "Peach",
    "Pistachio", "Coconut", "Maple", "Butter Pecan", "Rocky Road",
    "Cheesecake", "Red Velvet", "Banana", "Cotton Candy", "Bubblegum",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variety() {
        assert!(!variety().is_empty());
    }

    #[test]
    fn test_topping() {
        assert!(!topping().is_empty());
    }

    #[test]
    fn test_flavor() {
        assert!(!flavor().is_empty());
    }
}
