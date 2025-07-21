use std::collections::HashMap;

pub struct Item {
    pub name: String,
    pub quantity: u32,
    pub price: f64,
}

impl Item {
    pub fn new(name: String, quantity: u32, price: f64) -> Self {
        Item {
            name,
            quantity,
            price
        }
    }

    pub fn display(&self) -> String {
        format!(
            "Item: {}, Quantity: {}, Price: ${:.2}",
            self.name, self.quantity, self.price
        )
    }

    pub fn total_price(&self) -> f64 {
        self.quantity as f64 * self.price
    }
}

#[derive(Debug)]
pub enum InventoryError {
    ItemNotFound,
    ItemAlreadyExists,
    EmptyInventory,
    InvalidInput(String)
}

impl std::fmt::Display for InventoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InventoryError::ItemNotFound => write!(f, "Item not found in inventory."),
            InventoryError::ItemAlreadyExists => write!(f, "Item already exists. Use Edit option to modify existing items"),
            InventoryError::EmptyInventory => write!(f, "Inventory is empty."),
            InventoryError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
        }
    }
}

impl std::error::Error for InventoryError {}

pub struct InventoryManager {
    items:  HashMap<String, Item>,
}

impl InventoryManager {
    pub fn new() -> Self {
        InventoryManager {
            items: HashMap::new(),
        }
    }

    pub fn add_item(&mut self, item: Item) -> Result<(), InventoryError> {
        if self.items.contains_key(&item.name) {
            return Err(InventoryError::ItemAlreadyExists);
        }
        self.items.insert(item.name.clone(), item);
        Ok(())
    }

    pub fn remove_item(&mut self, name: &str) -> Result<(), InventoryError> {
        if self.items.remove(name).is_none() {
            return Err(InventoryError::ItemNotFound);
        }
        Ok(())
    }

    pub fn edit_item(&mut self, name: &str, quantity: u32, price: f64) -> Result<(), InventoryError> {
        match self.items.get_mut(name) {
            Some(item) => {
                item.quantity = quantity;
                item.price = price;
                Ok(())
            },
            None => Err(InventoryError::ItemNotFound),
        }
    }

    pub fn list_items(&self) -> Result<Vec<String>, InventoryError> {
        if self.items.is_empty() {
            return Err(InventoryError::EmptyInventory);
        }
        Ok(self.items.values().map(|item| item.display()).collect())
    }
}

impl Default for InventoryManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_creation() {
        let item = Item::new("Test Item".to_string(), 10, 15.99);
        assert_eq!(item.name, "Test Item");
        assert_eq!(item.quantity, 10);
        assert_eq!(item.price, 15.99);
    }

    #[test]
    fn test_item_display() {
        let item = Item::new("Test Item".to_string(), 5, 9.99);
        let display = item.display();
        assert!(display.contains("Test Item"));
        assert!(display.contains("5"));
        assert!(display.contains("9.99"));
    }

    #[test]
    fn test_item_total_value() {
        let item = Item::new("Test Item".to_string(), 5, 10.0);
        assert_eq!(item.total_value(), 50.0);
    }

    #[test]
    fn test_inventory_manager_creation() {
        let manager = InventoryManager::new();
        assert_eq!(manager.item_count(), 0);
        assert!(manager.is_empty());
    }

    #[test]
    fn test_add_item_success() {
        let mut manager = InventoryManager::new();
        let result = manager.add_item("Test Item".to_string(), 10, 15.99);
        assert!(result.is_ok());
        assert_eq!(manager.item_count(), 1);
    }

    #[test]
    fn test_add_duplicate_item() {
        let mut manager = InventoryManager::new();
        manager.add_item("Test Item".to_string(), 10, 15.99).unwrap();
        let result = manager.add_item("test item".to_string(), 5, 10.0);
        assert!(matches!(result, Err(InventoryError::ItemAlreadyExists)));
    }

    #[test]
    fn test_remove_item() {
        let mut manager = InventoryManager::new();
        manager.add_item("Test Item".to_string(), 10, 15.99).unwrap();
        
        let removed_item = manager.remove_item("Test Item").unwrap();
        assert_eq!(removed_item.name, "Test Item");
        assert!(manager.is_empty());
    }

    #[test]
    fn test_edit_item() {
        let mut manager = InventoryManager::new();
        manager.add_item("Old Name".to_string(), 10, 15.99).unwrap();
        
        let result = manager.edit_item("Old Name", "New Name".to_string(), 20, 25.99);
        assert!(result.is_ok());
        
        let updated_item = result.unwrap();
        assert_eq!(updated_item.name, "New Name");
        assert_eq!(updated_item.quantity, 20);
        assert_eq!(updated_item.price, 25.99);
    }
}