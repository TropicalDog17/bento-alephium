pub enum ProcessorConfig {
    DefaultProcessor,
    TransactionProcessor,
    BlockEventProcessor,
}

impl ProcessorConfig {
    pub fn name(&self) -> &'static str {
        match self {
            ProcessorConfig::DefaultProcessor => "default_processor",
            ProcessorConfig::TransactionProcessor => "transaction_processor",
            ProcessorConfig::BlockEventProcessor => "block_event_processor",
        }
    }
}
