# Black-Scholes Option Pricing Model

This example uses Nexus to prove an execution of the [Black-Scholes option pricing model](https://en.wikipedia.org/wiki/Black%E2%80%93Scholes_model). 

In order to target `riscv32` under `no_std`, math functions from `libm` instead of `std` are used.
Furthermore, instead of using a commonly available statistics packages like `rstat`, the error function 
and the CDF for the normal distribution are implemented here using numerical approximation.