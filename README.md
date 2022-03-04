# easy_trajectories
Simple to use trajectory simulations for Rust

## Install
(To be implemented)

## Usage

Example of the *find_distance_angle()* function:
```markdown
use easy_trajectory as e_traj;

let (distance, angle) = e_traj::simulation::find_distance_angle(drag, velocity, mass, gravity, max_time,precision, vertical_distance);

println!("Distance : {}", distance);
println!("Angle    : ()", angle);
```

More detailed documentation for each function in the simulation.rs file above each function explaining, use case, inputs, outputs, and what numbers should go in. Later I will implement functions to verify inputs are valid instead of manual understanding.