use std::collections::HashMap;
use std::env;
use std::fmt;
use strum_macros::EnumIter;
// For deriving EnumIter if needed in future iterations

// Step 1: Define Enum with variants (without default values directly inside)
#[derive(Debug, Hash, Eq, PartialEq, EnumIter)] // Removed Display; custom implementation added
pub enum EnvironmentVariable {
    BeamFilePath, // BEAM_FILE_PATH
    BeamAppKeyFormat, // BEAM_APP_KEY_FORMAT
    BeamFileChangeCheckCronExpression, // BEAM_FILE_CHANGE_CHECK_CRON_EXPRESSION
    BeamProxyContainerName, // BEAM_PROXY_CONTAINER_NAME
    Host,         // HOST
    Port,         // PORT
    ApiKey,       // API_KEY
    DockerHost, // DOCKER_HOST . For Windows Docker Desktop: "tcp://localhost:2375" .
    // If DockerHost is set, it will be preferred. If not, it is taken the DockerUnixSocket value.
    DockerUnixSocket // DOCKER_UNIX_SOCKET. For Unix. Normally: "/var/run/docker.sock"
    // Add more environment variables as needed.
    // Remember that a variable MyExample must be set as MY_EXAMPLE in the environment variables
}

// Utility function to convert Pascal case to uppercase with underscores
fn pascal_to_uppercase_with_underscores(input: &str) -> String {
    let mut result = String::new();
    for (i, c) in input.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            result.push('_'); // Add underscore before uppercase letters (except the first one)
        }
        result.push(c);
    }
    result.to_uppercase()
}

// Custom Display implementation for EnvironmentVariable
impl fmt::Display for EnvironmentVariable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pascal_case = format!("{:?}", self); // Get the enum variant name (e.g., "ApiKey")
        let formatted = pascal_to_uppercase_with_underscores(&pascal_case); // Convert to "API_KEY"
        write!(f, "{}", formatted)
    }
}

// Step 2: Implement methods for EnvironmentVariable
impl EnvironmentVariable {
    // Step 3: Create a static HashMap with default values
    fn default_values_map() -> HashMap<EnvironmentVariable, String> {
        let mut defaults = HashMap::new();
        defaults.insert(EnvironmentVariable::Host, "0.0.0.0".to_string());
        defaults.insert(EnvironmentVariable::Port, "3000".to_string());
        defaults.insert(EnvironmentVariable::BeamAppKeyFormat, "APP_{}_KEY".to_string());
        defaults.insert(EnvironmentVariable::BeamFileChangeCheckCronExpression, "0 * * * * *".to_string());
        defaults.insert(EnvironmentVariable::DockerUnixSocket, "/var/run/docker.sock".to_string());
        // Add default values for environment variables like here:
        defaults
    }

    // Step 4: Get the value of an environment variable or use the default
    pub fn get_env_var(&self) -> String {
        let env_var_name = self.to_string(); // Enum variant name is now in "API_KEY" format

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
            .trim() // Trim any leading/trailing whitespace or newline characters
            .to_string()
    }

    // handle panics and return Option
    pub fn catch_env_var(&self) -> Option<String> {
        // Catch any panic during the call to get_env_var
        let result = std::panic::catch_unwind(|| self.get_env_var());

        match result {
            Ok(value) => Some(value),   // Return the value if no panic occurred
            Err(_) => None,             // Return None if panic occurred
        }
    }

}
