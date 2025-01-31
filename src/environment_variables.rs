use std::env;
use std::collections::HashMap;
use strum_macros::{Display}; // Use Display to convert enum to string

// Step 1: Define Enum with variants (without default values directly inside)
#[derive(Debug, Display, Hash, Eq, PartialEq)] // Derive Display instead of ToString
pub enum EnvironmentVariable {
    // Path to the beam file with the key values for registering a new app in beam.
    // Internally, each line has the structure APP_<app id>_KEY=<app secret>
    BeanFilePath, // BEAN_FILE_PATH
    Host, // HOST
    Port // PORT
    // Add here more environment variables as needed.
    // Remember that a variable MyExample must be set as MY_EXAMPLE in the environment variables
}

// Step 2: Implement methods for EnvironmentVariable
impl EnvironmentVariable {
    // Step 3: Create a static HashMap with default values
    fn default_values_map() -> HashMap<EnvironmentVariable, String> {
        let mut defaults = HashMap::new();
        defaults.insert(EnvironmentVariable::Host, "0.0.0.0".to_string());
        defaults.insert(EnvironmentVariable::Port, "3000".to_string());
        // Add default values for environment variables like here:
        // defaults.insert(EnvironmentVariable::BeanFilePath, "/default/path".to_string()); // Example
        // Add more default values as needed
        defaults
    }

    // Step 4: Get the value of an environment variable or use the default
    pub fn get_env_var(&self) -> String {
        let env_var_name = self.to_string().to_uppercase(); // Convert enum to string (e.g., "BEAN_FILE_PATH")

        // Try to get the environment variable value
        match env::var(&env_var_name) {
            Ok(value) => value, // Return the value if it exists
            Err(_) => {
                // If the variable is not set, check if there's a default in the map
                let defaults = Self::default_values_map();
                if let Some(default_value) = defaults.get(self) {
                    default_value.to_string() // Return default value if it exists
                } else {
                    panic!(
                        "Environment variable '{}' is not set, and no default value is provided!",
                        env_var_name
                    );
                }
            }
        }
    }
}

/*
// Example of how to use it:
fn main() {
    // Call get_env_var for different environment variables
    let beam_file_path = EnvironmentVariable::BeanFilePath.get_env_var();

    // Output the values
    println!("Beam file path: {}", beam_file_path);
}
*/