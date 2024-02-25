pub struct Config {
    pub block_size_limit: usize,

    // TODO: add option to pass fastmem pointer
}

impl Default for Config {
    fn default() -> Self {
        Self {
            block_size_limit: 1,
        }
    }
}