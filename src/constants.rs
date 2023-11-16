pub const BUFFER_SIZE: usize = 4096;

#[cfg(test)]
pub const TEST_RESOURCES_DIR_PATH: &str = "./test_resources/";

#[cfg(test)]
pub fn test_resource_path(resource_relative_path: &str) -> String {
    TEST_RESOURCES_DIR_PATH.to_owned() + resource_relative_path
}
