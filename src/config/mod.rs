pub enum ProcessorConfig {
    DefaultProcessor,
    EventProcessor,
    TransactionProcessor,
    BlockProcessor,
}

impl ProcessorConfig {
    pub fn name(&self) -> &'static str {
        match self {
            ProcessorConfig::DefaultProcessor => "default_processor",
            ProcessorConfig::EventProcessor => "event_processor",
            ProcessorConfig::TransactionProcessor => "transaction_processor",
            ProcessorConfig::BlockProcessor => "block_processor",
        }
    }
}
