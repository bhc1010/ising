# ising-rs

![](https://github.com/bhc1010/ising/blob/master/assets/preview.gif)

Real-time interactive Ising model running in terminal, written in Rust.

# Ising Model
The 2D Ising model is a mathematical model in statisical mechanics of a two dimensional spin lattice.
A spin state is a state that can take one of two values, typically denoted up and down.
The Ising model couples neighboring spins in the lattice by imposing the Hamiltonian
$$H = J \sum_{<i, j>} \sigma_i \sigma_j$$
where $J \in \mathbb{R}$ is the coupling constant, $\sigma_i \in {-1, 1}$ is the spin value of the $i^{\textrm{th}}$ lattice site, and $\sum_{<i, j>}$ denotes the sum over nearest neighbor pairs.
This simple Hamilontian exhibits complex large-scale behavior such as phase transitions and scale invariance.
With a coupling constant of $J = 1$, a phase transition occurs when the temperature drops below the critical temperature $T_c = 2.696$, while scale invariance can be observed at the critical temperature. 

Additionally, one cna add an external magnetic field term to bias the system:
$$H = -J\sum_{<i, j>}\sigma_i\sigma_j - h\mu\sum_i\sigma_i$$
where h is the magnetic field strength and $\mu$ is the magnetic moment.
The second sum represents the coupling of each spin to the external magnetic field.
This interaction bias' the spins to point in the direction of the magnetic field.
The complete Ising model Hamiltonian is a simple but useful model of (anti-)ferromagnetic material.

# Monte Carlo
