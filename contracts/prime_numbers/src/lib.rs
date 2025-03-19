#![no_std]
extern crate alloc;
use alloc::string::{String as StdString, ToString};
use soroban_sdk::{contract, contractimpl, Env, String};

#[contract]
#[derive(Default)]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn check_prime(env: Env, num: i32) -> String {
        // If the number is negative, return "0".
        if num < 0 {
            return String::from_str(&env, "0");
        }
        
        // Determine if the number is prime.
        // We'll treat 0 and 1 as non-prime.
        let is_prime = if num < 2 {
            false
        } else if num == 2 {
            true
        } else if num % 2 == 0 {
            false
        } else {
            let sqrt_num = (num as f64).sqrt() as i32;
            let mut prime = true;
            let mut i = 3;
            while i <= sqrt_num {
                if num % i == 0 {
                    prime = false;
                    break;
                }
                i += 2;
            }
            prime
        };

        // Convert the number to a standard string.
        let num_str = num.to_string();
        
        // Build the output message.
        let std_result: StdString = if is_prime {
            "El número ".to_string() + &num_str + " es primo."
        } else {
            // For non-prime numbers, compute the factors.
            let mut factors = StdString::new();
            if num == 0 {
                // By convention, we define the factor of 0 as "0".
                factors.push_str("0");
            } else {
                for i in 1..=num {
                    if num % i == 0 {
                        if factors.is_empty() {
                            factors.push_str(&i.to_string());
                        } else {
                            factors.push_str(" ");
                            factors.push_str(&i.to_string());
                        }
                    }
                }
            }
            "El número ".to_string() + &num_str + " no es primo. Factores: " + &factors
        };

        // Convert the standard string to a Soroban string.
        String::from_str(&env, &std_result)
    }
}

mod test;