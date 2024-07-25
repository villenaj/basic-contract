#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod basiccontract {
    use ink_prelude::vec::Vec;

    #[ink(storage)]
    pub struct BasicContract {
        // Add `value` variable as vec in u32
    }

    impl BasicContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            // Initialize the vec storage
            todo!();
        }

        #[ink(message)]
        pub fn add_value(
            &mut self, 
            value: u32
        ) {
            // Push some value in the vec storage
            todo!();
        }

        #[ink(message)]
        pub fn get_values(&self) -> Vec<u32> {
            // Get all values in the storage
            // Note: make sure you don't change the ownership
            todo!();
        }

        #[ink(message)]
        pub fn get_value(
            &self, 
            index: u32
        ) -> Option<u32> {
            // Get specific value in the vec
            // Note: index uses `usize` data type and don't change ownership
            // Turn primitive types into other primitive types
            todo!();
        }

        #[ink(message)]
        pub fn remove_value(&mut self, index: u32) -> bool {
            // Remove the value in the storage using index
            // Add checking if the size of storage is greater than or equal to index
            // Return `true` if value was removed else, return false
            todo!();
        }

        #[ink(message)]
        pub fn update_value(
            &mut self, 
            index: u32, 
            value: u32
        ) -> bool {
            // Update the value in the storage using index
            // Return `true` if value was updated else, return false
            todo!();
        }

        #[ink(message)]
        pub fn value_exists(
            &self, 
            value: u32
        ) -> bool {
            // Check if the storage contains `value` parameter
            todo!();
        }
    }

    /// NOTE: DO NOT MODIFY BELOW
    /// Testing functions
    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn it_works() {
            let mut contract = BasicContract::new();
            contract.add_value(42);
            assert_eq!(contract.get_value(0), Some(42));
            assert_eq!(contract.get_values(), vec![42]);
        }

        #[ink::test]
        fn add_and_get_values() {
            let mut contract = BasicContract::new();
            contract.add_value(1);
            contract.add_value(2);
            contract.add_value(3);
            assert_eq!(contract.get_values(), vec![1, 2, 3]);
            assert_eq!(contract.get_value(1), Some(2));
        }

        #[ink::test]
        fn remove_value() {
            let mut contract = BasicContract::new();
            contract.add_value(1);
            contract.add_value(2);
            contract.add_value(3);
            assert!(contract.remove_value(1));
            assert_eq!(contract.get_values(), vec![1, 3]);
            assert!(!contract.remove_value(10));
        }

        #[ink::test]
        fn update_value() {
            let mut contract = BasicContract::new();
            contract.add_value(1);
            contract.add_value(2);
            assert!(contract.update_value(1, 5));
            assert_eq!(contract.get_values(), vec![1, 5]);
            assert!(!contract.update_value(10, 5));
        }

        #[ink::test]
        fn value_exists() {
            let mut contract = BasicContract::new();
            contract.add_value(1);
            contract.add_value(2);
            contract.add_value(3);
            assert!(contract.value_exists(2));
            assert!(!contract.value_exists(4));
        }
    }
}
