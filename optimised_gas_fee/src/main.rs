use ethereum_types::U256;
use evm::{CostType, Schedule};

fn main() {
    let schedule = Schedule::new_constantinople();
    let exponent = vec![0u8; 100];
    let gas_fee = calculate_gas_fee(&schedule, &exponent);
    println!("gas fee: {}", gas_fee);
}

fn calculate_gas_fee(schedule: &Schedule, exponent: &[u8]) -> U256 {
    let mut gas_fee = U256::from(0);
    // Perform operations on gas_fee
    gas_fee = gas_fee + schedule.sload * U256::from(10);
    gas_fee = gas_fee + schedule.balance * U256::from(20);
    gas_fee = gas_fee + schedule.expbyte * U256::from(exponent.len());
    // Return the calculated gas fee
    return gas_fee;
}
