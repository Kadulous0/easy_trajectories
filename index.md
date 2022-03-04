## Easy Trajectories

A simple, easy, and accurate trajectory simulator made in Rust. It only requires a few variables to be known, things like mass of the projectile, velocity, launch angle, and even allows for projectiles with drag!

### Example: The find_max_distance Function

Input 6 values: Drag, Velocity, Gravity, Mass, Vertical Distance (difference between target altitude and your altitude), and Precision (in how many decimal places for the angle). Receive the distance and the angle for that distance.

```markdown
use easy_trajectories as etraj;

let (distance, angle) = etraj::simulation::find_max_distance(drag, velocity, gravity, mass, vertical_distance, precision);

println!("Distance : {}", distance);
println!("Angle    : ()", angle);
```
