# easy_trajectories
Simple to use trajectory simulations for Rust

## Install
To install go to https://crates.io/crates/easy_trajectories and copy the lines under the install area to always get the latest version.

To use easy_trajectories, just add

```markdown
use easy_trajectories
```
or 
```markdown
use easy_trajectories as e_traj // <--- or whatever alias you prefer
```

## Usage

Example of the *find_distance_angle()* function:
```markdown
use easy_trajectory as e_traj;

let (distance, angle) = e_traj::simulation::find_distance_angle(drag, velocity, mass, gravity, max_time, precision, vertical_distance);

println!("Distance : {}", distance);
println!("Angle    : {}", angle);
```

More detailed documentation for each function in the simulation.rs file above each function explaining, use case, inputs, outputs, and what numbers should go in. Later I will implement functions to verify inputs are valid instead of manual understanding.

Function names are subject to change, and they will probably change.
