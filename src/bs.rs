use libm::{exp, log, sqrt};

/// Error Function
/// erf(x) for x >= 0
/// See https://en.wikipedia.org/wiki/Error_function#Numerical_approximations
pub fn erf(x: f64) -> f64 {
    const P: f64 = 0.47047;
    const A1: f64 = 0.3480242;
    const A2: f64 = -0.0958798;
    const A3: f64 = 0.7478556;

    let t = 1.0 / (1.0 + P * x);

    1.0 - (A1 * t + A2 * t*t + A3 * t*t*t) * exp(-x*x)
}

/// Normal Cumulative Distribution Function
pub fn norm_cdf(x: f64) -> f64 {
    //noinspection RsApproxConstant
    const SQRT2: f64 = 1.41421356237;

    let er = if x > 0.0 { erf(x / SQRT2) } else { -erf(-x / SQRT2) };

    (1.0 + er) / 2.0
}

/// Black-Scholes Call Option Price
/// call(s, k, t, v, r)
/// s: current stock price
/// k: strike price
/// t: number of years to expiration
/// v: annualized volatility
/// r: risk-free interest rate
/// See https://www.investopedia.com/terms/b/blackscholes.asp#toc-the-black-scholes-model-formula
pub fn call_price(s: f64, k: f64, t: f64, v: f64, r: f64) -> f64 {
    let d1 = (log(s / k) + (r + v*v / 2.0) * t) / (v * sqrt(t));
    let d2: f64 = d1 - v * sqrt(t);

    s * norm_cdf(d1) - k * exp(-r * t) * norm_cdf(d2)
}