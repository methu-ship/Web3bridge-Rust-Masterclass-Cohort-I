// ### Group 22: Facility Space Manager

// - Description: Manage office space allocations.
// - Stage 1:
//   - Add space assignments (space name, occupant, purpose).
//   - View all assignments.
// - Stage 2:
//   - Remove assignments.
// - Stage 3:
//   - Edit assignment details.
//   - Cancel edits.
// - Implementation Tips: Use a `Vec` initially, then a `HashMap` with space name as the key.

use std::io;

struct Space {
    name: String,
    occupant: String,
    purpose: String,
}

struct SpaceManager {
    spaces: Vec<Space>,
}

impl SpaceManager {
    fn new() -> Self {
        SpaceManager { spaces: vec![] }
    }

    fn input(text: String) -> String {
        println!("{}", text);
        let mut prompt = String::new();
        io::stdin()
            .read_line(&mut prompt)
            .expect("something went wrong");

        prompt
    }

    fn add_space(&mut self) {
        let space_name = Self::input("Enter space name: ".to_string());
        if !self
            .spaces
            .iter_mut()
            .find(|el| el.name == space_name)
            .is_some()
        {
            let occupant = Self::input("Enter occupant name: ".to_string());
            let purpose = Self::input("Enter purpose of the space: ".to_string());
            self.spaces.push(Space {
                name: space_name.to_string(),
                occupant: occupant.trim().to_string(),
                purpose: purpose.trim().to_string(),
            });
        } else {
            println!("Space existed already")
        }
    }

    fn view_spaces(&self) {
        if self.spaces.is_empty() {
            println!("No spaces available.");
            return;
        }

        for space in &self.spaces {
            println!(
                "Space: {}, Occupant: {}, Purpose: {}",
                space.name, space.occupant, space.purpose
            );
        }
    }

    fn remove_space(&mut self) {
        let space_name = Self::input("Enter space name: ".to_string());

        if !self.spaces.iter().any(|s| s.name == space_name) {
            println!("Space does not exist.");
            return;
        }
        self.spaces.retain(|space| space.name != space_name);
        println!("Space deleted");
    }

    fn edit_space(&mut self) {
        let space_name = Self::input("Enter space name: ".to_string());
        if !self.spaces.iter().any(|s| s.name == space_name) {
            println!("Space does not exist.");
            return;
        }

        if let Some(space) = self.spaces.iter_mut().find(|s| s.name == space_name) {
            space.occupant = Self::input("Enter new occupant name: ".to_string());
            space.purpose = Self::input("Enter new purpose: ".to_string());
        }
    }
}

fn main() {
    let mut space_manager = SpaceManager::new();

    loop {
        println!("To create a Space, press 1");
        println!("To view all Spaces, press 2");
        println!("To edit a Space, press 3");
        println!("To delete a Space, press 4");
        println!("To abort, press 5");

        let mut option = String::new();

        io::stdin().read_line(&mut option);

        match option.trim() {
            "1" => {
                space_manager.add_space();
            }
            "2" => {
                space_manager.view_spaces();
            }
            "3" => {
                space_manager.edit_space();
            }
            "4" => {
                space_manager.remove_space();
            }
            "5" => {
                println!("Abort!!!");
                break;
            }
            _ => println!("Unrecognized command"),
        }
    }
}
