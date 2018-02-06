# Coprime Probability Monte Carlo
A simple program that estimates the probability that two integers on `[1, 2^64 - 1]` are coprime using a Monte Carlo simulation.

I recently saw a Numberphile video titled [*Tree Gaps and Orchard Problems*](https://www.youtube.com/watch?v=p-xa-3V5KO8). At one point, the video mentions that the probability of two numbers being coprime is equal to `1 / zeta(2)`, zeta being the Riemann Zeta function, which is `6 / pi^2`, or about 60.79%.

*"That's really interesting,"* I thought, *"I wonder if trying to derive that value via a coprimality Monte Carlo experiment would tell you anything interesting about the RNG you used."*.

I wrote this program to tinker with that idea, but:
* I don't specify an RNG explicitly, so it's up to the `rand` crate
* I'm not sure this is an interesting metric to compare RNGs with

## Running
Requires a recent version of stable Rust. 1.23.0 was used when I wrote this, so that ought to work.

To run the simulation, use:
```
cargo run
```

## LICENSE
This repository is available under the terms of the Unlicense. See [LICENSE](LICENSE) or [unlicense.org](https://unlicense.org) for more details.