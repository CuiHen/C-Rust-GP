Code for: CRust_GP: A Framework for the Modular Construction and Evaluation of Cartesian Genetic Programming in Rust

# Rust
The code is written in Rust only.  
For installation, see: https://github.com/rust-lang/rust/blob/master/README.md

# Building
You have to build everything yourself. You will need a working `Rust` and `Cargo` setup. [Rustup](https://rustup.rs/) is the simplest way to set this up on either Windows, Mac or Linux.

Once the prerequisites have been installed, compilation on your native platform is as simple as running the following in a terminal:

```
cargo build --release
```

# Overview
CRust_GP aims to make construction and modification of Cartesian Genetic Programming (CGP) as simple and reliable as possible. 
It provides a comprehensive set of operators, extensions, and QoL tools.
Due to its component-based architecture, creating a new CGP configuration can be realized by just replacing a few lines of code.

Key features include:
- Simple and modular construction of CGP configurations
- Crossover Operators:
  - Singe Point Crossover
  - $n$-Point Crossover
  - Uniform Crossover
- Mutation Operators:
  - Probabilistic Mutation
  - Single Active Mutation (see [https://doi.org/10.1007/978-3-642-37207-0_6](https://doi.org/10.1007/978-3-642-37207-0_6))
  - Split Mutation (see [https://doi.org/10.1007/978-3-031-21094-5_14](https://doi.org/10.1007/978-3-031-21094-5_14))
  - Multi-$n$ Mutation (see [https://doi.org/10.1007/978-3-031-21094-5_14](https://doi.org/10.1007/978-3-031-21094-5_14))
- Selection Methods:
  - (1+4)-ES with neutral search (Standard in CGP)
  - ($mu$ + $\lambda$)-ES with neutral search
  - Tournament selection with optional elitists
- Extensions:
  -  DAG (see [https://doi.org/10.1145/2463372.2463482](https://doi.org/10.1145/2463372.2463482))
  -  Reorder (see [https://doi.org/10.1145/2463372.2463482](https://doi.org/10.1145/2463372.2463482))
  -  Equidistant Reorder (see [https://doi.org/10.5220/0012174100003595](https://doi.org/10.5220/0012174100003595))
  -  Negative Bias Reorder (see [https://doi.org/10.48550/arXiv.2410.00518](https://doi.org/10.48550/arXiv.2410.00518))
  -  Normal Distributed Reorder (see [https://doi.org/10.48550/arXiv.2410.00518](https://doi.org/10.48550/arXiv.2410.00518))
  -  Left Skewed Reorder (see [https://doi.org/10.48550/arXiv.2410.00518](https://doi.org/10.48550/arXiv.2410.00518))
    
Although CRust_GP has been developed primarily as a research tool, it can be used to solve real-world problems.
