use std::io;

#[derive(Clone, Debug)]
struct ResourceAllocator {
    id: u32,
    resource_name: String,
    project_name: String,
    quantity: u32,
}


struct Resources {
    resource: Vec<ResourceAllocator>,
    next_id: u32
}

impl Resources {
    fn init() -> Self {
        Self {
            resource: Vec::new(),
            next_id: 1
        }
    }

    fn add_resource(&mut self, resource_name: String, project_name: String, quantity: u32,) -> u32 {
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

    fn view_allocations(&self) -> &Vec<ResourceAllocator> {
        &self.resource
    }

    fn remove_allocations(&mut self, id: u32) {
        self.resource.retain(|index| index.id != id);
    }

    fn edit_allocation(&mut self, id: u32, new_resource_name: String, new_project_name: String, new_quantity: u32) {
        if let Some(rec) = self.resource.iter_mut().find(|index| index.id == id) {
            rec.resource_name = new_resource_name;
            rec.project_name = new_project_name;
            rec.quantity = new_quantity;
        }
    }
}

fn main() {
    let mut resources = Resources::init();

    println!("Resource Allocation\nAdd a record of the resources you need for your project");
    loop {
        println!("Enter your resource name");
        let mut resource_name = String::new();
        io::stdin().read_line(&mut resource_name).expect("Something is wrong with resource name");
        let resource_name: String = match resource_name.trim().parse() {
            Ok(name) => name,
            Err(_) => continue,
        };
        println!("Enter your project name");
        let mut project_name = String::new();
        io::stdin().read_line(&mut project_name).expect("Something is wrong with resource name").to_string();
        let project_name: String = match project_name.trim().parse() {
            Ok(name) => name,
            Err(_) => continue,
        };
        println!("Enter your quantity");
        let mut quantity = String::new();
        io::stdin().read_line(&mut quantity).expect("Something is wrong with resource name");
        let quantity: u32 = match quantity.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        
        
        resources.add_resource(resource_name, project_name, quantity);

        let out = resources.view_allocations();
        println!("Out first add: {:#?}\n", out);
        
        
        println!("Enter your resource name");
        let mut resource_name2 = String::new();
        io::stdin().read_line(&mut resource_name2).expect("Something is wrong with resource name");
        let resource_name2: String = match resource_name2.trim().parse() {
            Ok(name) => name,
            Err(_) => continue,
        };
        println!("Enter your project name");
        let mut project_name2 = String::new();
        io::stdin().read_line(&mut project_name2).expect("Something is wrong with resource name").to_string();
        let project_name2: String = match project_name2.trim().parse() {
            Ok(name) => name,
            Err(_) => continue,
        };
        println!("Enter your quantity");
        let mut quantity2 = String::new();
        io::stdin().read_line(&mut quantity2).expect("Something is wrong with resource name");
        let quantity2: u32 = match quantity2.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        resources.add_resource(resource_name2, project_name2, quantity2);

        let out = resources.view_allocations();
        println!("Out Second add: {:#?}", out);

        resources.remove_allocations(1);

        let out = resources.view_allocations();
        println!("Out after remove 1: {:#?}", out);

        resources.edit_allocation(2, String::from("Wood"), String::from("Nails"), 16);

        let out = resources.view_allocations();
        println!("Out after edit: {:#?}", out);

        break;
    }
}
