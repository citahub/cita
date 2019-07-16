/// Definition of the cost schedule for a transaction.
pub struct TxGasSchedule {
    /// Transaction cost
    pub tx_gas: usize,
    /// `CREATE` transaction cost
    pub tx_create_gas: usize,
    /// Additional cost for empty data transaction
    pub tx_data_zero_gas: usize,
    /// Aditional cost for non-empty data transaction
    pub tx_data_non_zero_gas: usize,
    /// Cost for contract length when executing `CREATE`
    pub create_data_gas: usize,
}

impl Default for TxGasSchedule {
    fn default() -> Self {
        TxGasSchedule {
            tx_gas: 21_000,
            tx_create_gas: 53_000,
            tx_data_zero_gas: 4,
            tx_data_non_zero_gas: 68,
            create_data_gas: 200,
        }
    }
}
