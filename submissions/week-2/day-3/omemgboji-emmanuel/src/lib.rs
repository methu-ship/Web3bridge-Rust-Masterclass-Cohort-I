pub mod enums;
pub mod structs;

use enums::EmployeeRoles;
use structs::Employee;

impl Employee {
    pub fn check_access(&self) -> Result<(), String> {
        if !self.is_employed {
            return Err("Access denied: You are not employed.".to_string());
        }

        match self.role {
            EmployeeRoles::MediaTeam | EmployeeRoles::ITDepartment | EmployeeRoles::Manager => {
                Ok(())
            }
            _ => Err("Access denied: You do not have the required role.".to_string()),
        }
    }

    pub fn access_garage(&self) -> Result<(), String> {
        self.check_access()?;
        println!("Welcome back! Access granted to the Web3Bridge Garage.");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access_garage() {
        let employee = Employee {
            role: EmployeeRoles::ITDepartment,
            is_employed: true,
        };
        assert!(employee.access_garage().is_ok());
    }

    #[test]
    fn test_access_garage_denied() {
        let employee = Employee {
            role: EmployeeRoles::KitchenStaff,
            is_employed: true,
        };
        assert!(employee.access_garage().is_err());
    }

    #[test]
    fn test_access_garage_not_employed() {
        let employee = Employee {
            role: EmployeeRoles::MediaTeam,
            is_employed: false,
        };
        assert!(employee.access_garage().is_err());
    }
}
