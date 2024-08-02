#![cfg_attr(target_arch = "riscv32", no_std, no_main)]

mod bs;

use nexus_rt::print;
use bs::call_price;
use libm::fabs;

#[nexus_rt::main]
fn main() {
    let s = 52.0; // current stock price: $52
    let k = 50.0; // strike price of the option: $50
    let t = 0.5; // 6 months to expiration
    let v = 0.12; // annualized volatility: 12%
    let r = 0.05; // risk-free interest rate: 5%

    let result_call_price = call_price(s, k, t, v, r);
    let expected_call_price = 3.788;
    let tolerance = 0.001;

    print!("{}\n", result_call_price);
    assert!(fabs(result_call_price - expected_call_price) < tolerance);
}
