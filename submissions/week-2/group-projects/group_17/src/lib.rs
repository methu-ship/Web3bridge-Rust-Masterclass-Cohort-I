
#[derive(Clone, Debug)]
pub struct ResourceAllocator {
    pub id: u32,
    pub resource_name: String,
    pub project_name: String,
    pub quantity: u32,
}

pub struct Resources {
    pub resource: Vec<ResourceAllocator>,
    next_id: u32
}

impl Resources {
    pub fn init() -> Self {
        Self {
            resource: Vec::new(),
            next_id: 1
        }
    }

    pub fn add_resource(&mut self, resource_name: String, project_name: String, quantity: u32) -> u32 {
        let id_count = self.next_id;
        self.next_id += 1;
        let record = ResourceAllocator {
            id: id_count,
            resource_name,
            project_name,
            quantity,
        };
        self.resource.push(record);
        id_count
    }

    pub fn view_allocations(&self) -> &Vec<ResourceAllocator> {
        &self.resource
    }

    pub fn remove_allocations(&mut self, id: u32) {
        self.resource.retain(|index| index.id != id);
    }

    pub fn edit_allocation(&mut self, id: u32, new_resource_name: String, new_project_name: String, new_quantity: u32) {
        if let Some(rec) = self.resource.iter_mut().find(|index| index.id == id) {
            rec.resource_name = new_resource_name;
            rec.project_name = new_project_name;
            rec.quantity = new_quantity;
        }
    }
}

// Re-export for easier imports
pub use Resources as ResourceManager;
