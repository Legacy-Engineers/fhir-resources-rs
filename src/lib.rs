pub mod human_name;
pub mod identifier;
pub mod patient;
pub mod patient_contact;
pub mod patient_communication;
pub mod patient_link;
pub mod data_types;
pub mod period;
pub mod account;
pub mod account_coverage;
pub mod account_guarantor;
pub mod account_diagnosis;
pub mod account_procedure;
pub mod account_related_account;
pub mod account_balance;    
pub mod money;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
