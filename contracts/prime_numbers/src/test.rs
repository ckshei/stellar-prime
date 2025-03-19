#![cfg(test)]
extern crate std;
use soroban_sdk::{Env, String};
use crate::{Contract, ContractClient};

#[test]
fn test_check_prime() {
    let env = Env::default();
    // Register the contract. No constructor arguments are needed.
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    // Negative number: returns "0"
    assert_eq!(client.check_prime(&-5), String::from_str(&env, "0"));

    // 0 is not prime. We define its factor as "0".
    assert_eq!(
        client.check_prime(&0),
        String::from_str(&env, "El número 0 no es primo. Factores: 0")
    );

    // 1 is not prime; factors: 1
    assert_eq!(
        client.check_prime(&1),
        String::from_str(&env, "El número 1 no es primo. Factores: 1")
    );

    // 2 is prime.
    assert_eq!(
        client.check_prime(&2),
        String::from_str(&env, "El número 2 es primo.")
    );

    // 4 is not prime; factors: 1, 2, 4.
    assert_eq!(
        client.check_prime(&4),
        String::from_str(&env, "El número 4 no es primo. Factores: 1 2 4")
    );

    // 7 is prime.
    assert_eq!(
        client.check_prime(&7),
        String::from_str(&env, "El número 7 es primo.")
    );

    // 9 is not prime; factors: 1, 3, 9.
    assert_eq!(
        client.check_prime(&9),
        String::from_str(&env, "El número 9 no es primo. Factores: 1 3 9")
    );
}
