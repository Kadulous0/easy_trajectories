// dependencies
use libm as math;
use egui::widgets::plot::Value;

/*
-------------------------------------------
FIND MAX DISTANCE FUNCTION:
-------------------------------------------
USE CASE:

Find the max distance of a projectile for 
any target altitude.
-------------------------------------------
INPUTS:

-Projectile Settings:
Drag Value          (f64)       No Unit     
Velocity            (f64)       m/s         
Mass                (f64)       kilograms   

-Simulation Settings:
Gravity             (f64)       m/s^2 
Max Sim Time        (f64)       seconds
Precision           (i8)        No Unit

-Target Settings:
Vertical Distance   (f64)       meters
-------------------------------------------
RETURNS:

Distance            (f64)       meters
Angle               (f64)       degrees
-------------------------------------------
EXPECTED INPUTS:

Drag Value                      float > 0.0
Velocity                       float >= 0.0
Mass                            float > 0.0
Gravity             (- assumed) float > 0.0
Max Sim Time                    float > 0.0
Precision                           int > 0
Vertical Distance              all real #'s
-------------------------------------------
*/

fn find_distance_angle(drag:f64, velocity:f64, mass:f64, gravity:f64, max_time: f64, precision:i8, vertical_distance:f64) -> (f64,f64) {

    // initalize variables
    let delta_time = 0.001; // delta time
    let mut angle= 45.0;  // inital angle
    let mut delta_angle = -1.0; // delta angle
    let mut iterations = 0; // decimal points for angle
    let mut distance = 0.0; // max distance

    // determine if above target to start with
    let mut has_been_above_target: bool;
    if vertical_distance < 0.0 {
        has_been_above_target = false;
    } else {
        has_been_above_target = true;
    }

    while iterations < precision { // while iteration is less than decimal precision requested

        // start a new simulation
        let mut time = 0.0; // reset time

        while time <= max_time { // while current time is <= the max possible sim time, do...

            // main trajectory equations
            let x = (mass/drag)*velocity*math::cos(0.0174532925*angle)*(1.0-math::exp(-(drag/mass)*time)); // x displacement at time
            let y = (mass/drag)*(velocity*math::sin(0.0174532925*angle)+((mass*gravity)/drag))*(1.0-math::exp(-(drag/mass)*time))-((mass/drag)*gravity*time)-vertical_distance; // y displacement at time

            // if the projectile started below the target and is now above the y value of the target, 
            if !has_been_above_target && y >= vertical_distance {has_been_above_target = true;} // allow for collisions with the surface

            if y < vertical_distance && has_been_above_target { // if y is below target and has been above target
                
                if x > distance { // check if ending x is further
                    distance = x; // if so, set that as new max distance
                } else if x < distance { // if not, check if we are hitting closer
                    delta_angle = -0.1 * delta_angle; // flip direction and become more precise
                    iterations += 1; // log that change to the decimal precision counter
                } else { // if we are somehow exact (super unlikely)
                    distance = x; // save it
                    iterations += 5; // skip all other possible iterations
                    break; // end
                }
                break; // end simulation loop
            }
            time += delta_time; // add time step and start loop again
        }
        angle += delta_angle; // change angle and restart sim
    }
    (distance, angle) // return max distance and angle for max distance
}



/*
-------------------------------------------
FIND TRAJECTORY ANGLE FUNCTION:
-------------------------------------------
USE CASE:

Find an angle to hit a point at a set
distance away.
-------------------------------------------
INPUTS:

-Projectile Settings:
Drag Value          (f64)       No Unit     
Velocity            (f64)       m/s         
Mass                (f64)       kilograms   

-Simulation Settings:
Gravity             (f64)       m/s^2 
Max Sim Time        (f64)       seconds
Precision           (i8)        No Unit
Ballistic           (bool)      No Unit

-Target Settings:
Distance            (f64)       meters
Vertical Distance   (f64)       meters
-------------------------------------------
RETURNS:

Trajectory Data (Vec<Value>)    Coordinates
Angle               (f64)       degrees
-------------------------------------------
EXPECTED INPUTS:

Drag Value                      float > 0.0
Velocity                       float >= 0.0
Mass                            float > 0.0
Gravity             (- assumed) float > 0.0
Max Sim Time                    float > 0.0
Precision                           int > 0
Ballistic                     true or false
Distance                        float > 0.0
Vertical Distance              all real #'s
-------------------------------------------
*/

fn find_trajectory_angle(drag:f64, velocity:f64, mass:f64, gravity:f64, max_time:f64, precision:i8, ballistic:bool, distance:f64, vertical_distance:f64) -> (Vec<Value>, f64) {

    // determines max range and panics if requested range is greater than the max range
    let (max_distance, max_dist_angle) = find_distance_angle(drag, velocity, mass, gravity, max_time, precision, vertical_distance);
    if distance > max_distance {
        panic!("Invalid Requested Range, decrease to {} or less!", max_distance); 
    }

    // initialize variables
    let delta_time = 0.001; // delta time
    let mut angle= max_dist_angle; // initial angle
    let mut delta_angle = -1.0; // change angle
    let mut iterations = 0; // decimal point for angle

    // determine if above target to start with
    let mut has_been_above_target: bool; 
    if vertical_distance < 0.0 {
        has_been_above_target = false;
    } else {
        has_been_above_target = true;
    }

    // change delta angle to find requested trajectory
    if ballistic {
        delta_angle *= -1.0;
    } 

    // create trajectory logger
    let mut trajectory: Vec<Value> = Vec::new();

    while iterations < precision {  // while iteration is less than decimal precision requested

        // Start a new simulation
        trajectory.clear(); // clears old trajectory
        let mut time = 0.0; // resets time (i forgot this and it was very bad)

        while time <= max_time { // while current time is <= the max possible sim time, do...
            
            // main trajectory equations
            let x = (mass/drag)*velocity*math::cos(0.0174532925*angle)*(1.0-math::exp(-(drag/mass)*time)); // x displacement at time
            let y = (mass/drag)*(velocity*math::sin(0.0174532925*angle)+((mass*gravity)/drag))*(1.0-math::exp(-(drag/mass)*time))-((mass/drag)*gravity*time)-vertical_distance; // y displacement at time
            
            trajectory.push(Value::new(x, y)); // add trajectory points

            // if the projectile started below the target and is now above the y value of the target, 
            if !has_been_above_target && y >= vertical_distance {has_been_above_target = true;} // allow for collisions with the surface

            if y < vertical_distance && has_been_above_target { // if y is below target and has been above target

                if delta_angle > 0.0 && x > distance { // if the change in angle between sims is positive and if we overshot this simulation
                    delta_angle *= -0.1; // flip direction and decrease the step size to 1/10th the last
                    iterations += 1; // add iteration to main while loop
                } else if delta_angle < 0.0 && x < distance { // if the change in angle between sims is negative and if we undershot this simulation
                    delta_angle *= -0.1; // flip direction and decrease the step size to 1/10th the last
                    iterations += 1; // add iteration to main while loop
                }
                break; // and end this sim as we have impacted the gound
            }
            time += delta_time; // add delta time to current time and run loop again
        }
        angle += delta_angle; // add change in angle to the angle
    }
    (trajectory,angle) // return trajectory data and angle required for the shot to hit
}



/*
-------------------------------------------
FIND TRAJECTORY DISTANCE TIME FUNCTION:
-------------------------------------------
USE CASE:

Calculate a trajectory, final x location,
and total flight time from a few variables.
-------------------------------------------
INPUTS:

-Projectile Settings:
Drag Value          (f64)       No Unit     
Velocity            (f64)       m/s         
Mass                (f64)       kilograms  
Angle               (f64)       degrees 

-Simulation Settings:
Gravity             (f64)       m/s^2 
Max Sim Time        (f64)       seconds

-Target Settings:
Vertical Distance   (f64)       meters
-------------------------------------------
RETURNS:

Trajectory Data (Vec<Value>)    Coordinates
X Displacement      (f64)       meters
Time of Flight      (f64)       seconds
-------------------------------------------
EXPECTED INPUTS:

Drag Value  (can be 0 but why?) float > 0.0
Velocity                       float >= 0.0
Mass                            float > 0.0
Angle                90.0 >= float >= -90.0
Gravity             (- assumed) float > 0.0
Max Sim Time                    float > 0.0
Vertical Distance              all real #'s
-------------------------------------------
*/

fn find_trajectory_distance_time(drag:f64, velocity:f64, mass:f64, angle:f64, gravity:f64, max_time:f64, vertical_distance:f64) -> (Vec<Value>,f64,f64) {
    // initalize variables
    let delta_time = 0.001; // delta time
    let mut time = 0.0; // reset time

    let mut x = 0.0;
    let mut y = 0.0;

    // determine if above target to start with
    let mut has_been_above_target: bool; 
    if vertical_distance < 0.0 {
        has_been_above_target = false;
    } else {
        has_been_above_target = true;
    }

    // create trajectory logger
    let mut trajectory: Vec<Value> = Vec::new();

    while time <= max_time { // while current time is <= the max possible sim time, do...
        // main trajectory equations
        x = (mass/drag)*velocity*math::cos(0.0174532925*angle)*(1.0-math::exp(-(drag/mass)*time)); // x displacement at time
        y = (mass/drag)*(velocity*math::sin(0.0174532925*angle)+((mass*gravity)/drag))*(1.0-math::exp(-(drag/mass)*time))-((mass/drag)*gravity*time)-vertical_distance; // y displacement at time

        trajectory.push(Value::new(x, y)); // add trajectory points

        // if the projectile started below the target and is now above the y value of the target, 
        if !has_been_above_target && y >= vertical_distance {has_been_above_target = true;} // allow for collisions with the surface

        if y < vertical_distance && has_been_above_target { // if y is below target and has been above target
            break; // end while loop
        }
        time += delta_time; // add delta time to current time and run loop again
    }
    (trajectory, x, time) // return trajectory data, final x location, and time of flight
}