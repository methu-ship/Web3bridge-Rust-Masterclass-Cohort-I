use std::{collections::HashMap, io};

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

impl SpaceManager {
    fn add_space_test(&mut self, space_name: &str, occupant: &str, purpose: &str) {
        if self.spaces.contains_key(space_name) {
            return println!("Space existed already");
        }
        self.spaces.insert(
            space_name.to_string(),
            Space {
                name: space_name.to_string(),
                occupant: occupant.to_string(),
                purpose: purpose.to_string(),
            },
        );
    }

    fn view_spaces_test(&self) {
        if self.spaces.is_empty() {
            return println!("No spaces available");
        }
    }

    fn remove_space_test(&mut self, space_name: &str) {
        if self.spaces.remove(space_name).is_none() {
            return println!("Space does not exist");
        }
    }

    fn edit_space_test(&mut self, space_name: &str, new_occupant: &str, new_purpose: &str) {
        if let Some(space) = self.spaces.get_mut(space_name) {
            space.occupant = new_occupant.to_string();
            space.purpose = new_purpose.to_string();
        } else {
            println!("Space does not exist");
        }
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
    fn initialize() -> SpaceManager {
        let mut space_manager = SpaceManager::new();
        space_manager.add_space_test("Web3Bridge Garage", "Gbemiga", "Meetings");
        space_manager.add_space_test("Ethereum house", "Temi", "Sleeping");
        space_manager
    }

    #[test]
    fn test_add_space() {
        let space_manager = initialize();
        assert_eq!(space_manager.spaces.len(), 2);
    }

    #[test]
    fn test_view_spaces() {
        let space_manager = initialize();
        assert!(!space_manager.spaces.is_empty());
    }

    #[test]
    fn test_remove_space() {
        let mut space_manager = initialize();
        space_manager.remove_space_test("Ethereum house");
        assert_eq!(space_manager.spaces.len(), 1);
    }

    #[test]
    fn test_edit_space() {
        let mut space_manager = initialize();
        space_manager.edit_space_test("Web3Bridge Garage", "Gbemiga", "Meetings and Coding");
        assert_eq!(
            space_manager
                .spaces
                .get("Web3Bridge Garage")
                .unwrap()
                .purpose,
            "Meetings and Coding"
        );
    }
}
