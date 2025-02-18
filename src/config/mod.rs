pub enum ProcessorConfig {
    DefaultProcessor,
    BlockProcessor,
    EventProcessor,
    LendingContractProcessor(String),
}

impl ProcessorConfig {
    pub fn name(&self) -> &'static str {
        match self {
            ProcessorConfig::DefaultProcessor => "default_processor",
            ProcessorConfig::BlockProcessor => "block_processor",
            ProcessorConfig::EventProcessor => "event_processor",
            ProcessorConfig::LendingContractProcessor(_) => "lending_contract_processor",
        }
    }
}
