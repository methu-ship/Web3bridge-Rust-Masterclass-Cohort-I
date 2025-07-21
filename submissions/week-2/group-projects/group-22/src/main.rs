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

use std::{
    collections::HashMap,
    io::{self},
};

struct Space {
    name: String,
    occupant: String,
    purpose: String,
}

struct SpaceManager {
    // spaces: Vec<Space>,
    spaces: HashMap<String, Space>,
}

impl SpaceManager {
    fn new() -> Self {
        SpaceManager {
            spaces: HashMap::new(),
        }
    }

    fn input(text: &str) -> String {
        println!("[prompt] -> {}", text);
        let mut prompt = String::new();
        io::stdin()
            .read_line(&mut prompt)
            .expect("something went wrong");

        prompt.trim().to_string()
    }

    fn add_space(&mut self) {
        let space_name = Self::input("Enter space name: ");

        if self.spaces.get(&space_name).is_none() {
            let occupant = Self::input("Enter occupant name: ");
            let purpose = Self::input("Enter purpose of the space: ");
            self.spaces.insert(
                space_name.trim().to_string(),
                Space {
                    name: space_name,
                    occupant: occupant,
                    purpose: purpose,
                },
            );
            println!("[response] -> Space Created \n");
        } else {
            println!("[response] -> Space existed already\n")
        }
    }

    fn view_spaces(&self) {
        if self.spaces.is_empty() {
            println!("[response] -> No spaces available.\n");
            return;
        }

        for (_, space) in &self.spaces {
            println!(
                "Space -> {}: Occupant: {}, Purpose: {}",
                space.name, space.occupant, space.purpose
            );
        }
        println!("");
    }

    fn remove_space(&mut self) {
        let space_name = Self::input("Enter space name: ");

        if self.spaces.get(&space_name).is_none() {
            println!("[response] -> Space does not exist.\n");
            return;
        }
        self.spaces.remove(&space_name);
        println!("[response] -> Space deleted\n");
    }

    fn edit_space(&mut self) {
        let space_name = Self::input("Enter space name: ");

        if let Some(space) = self.spaces.get_mut(&space_name) {
            let confirmation = Self::input(
                format!(
                    "Are you sure you want to edit this space {}? (yes/no): ",
                    space.name,
                )
                .as_str(),
            );
            if confirmation.to_lowercase() != "yes" {
                println!("[response] -> Edit cancelled.\n");
                return;
            }

            space.occupant = Self::input("Enter new occupant name: ");
            space.purpose = Self::input("Enter new purpose: ");
            return;
        }
        println!("[response] -> Space does not exist.\n");
    }
}

fn main() {
    let mut space_manager = SpaceManager::new();

    println!(
        "
    ===============================================
    =                  Welcome                    =
    =           Facility Space Manager            =
    =                                             =
    =   Authors:                                  =
    =           Adekeye Temitope;                 =
    =           Gbangbola Oluwagbemig;            =
    =           <other-long-ass-names-go-here>;   =
    =                                             =
    =                               Group 22;     =
    ===============================================
    "
    );

    loop {
        println!("Prompt: To create a Space, press 1");
        // println!("\tTo create a Space, press 1");
        println!("\tTo view all Spaces, press 2");
        println!("\tTo edit a Space, press 3");
        println!("\tTo delete a Space, press 4");
        println!("\tTo abort, press 5");

        let mut option = String::new();

        let _ = io::stdin().read_line(&mut option);

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
                println!("[Thank you for staying with us . . . do come again]");
                break;
            }
            _ => println!("Unrecognized command"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_space_creation() {
        let mut manager = SpaceManager::new();
        manager.add_space();
        assert_eq!(manager.spaces.len(), 1);
    }

    #[test]
    fn test_view_spaces() {
        let mut manager = SpaceManager::new();
        manager.add_space();
        manager.view_spaces();
        assert!(!manager.spaces.is_empty());
    }

    #[test]
    fn test_space_removal() {
        let mut manager = SpaceManager::new();
        manager.add_space();
        manager.remove_space();
        assert_eq!(manager.spaces.len(), 0);
    }
    #[test]
    fn test_space_editing() {
        let mut manager = SpaceManager::new();
        manager.add_space();
        manager.edit_space();
        assert_eq!(manager.spaces.len(), 1);
    }
}
