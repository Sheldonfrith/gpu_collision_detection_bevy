# gpu_accelerated_collision_detection

Comparing performance of naive GPU vs CPU collision detection for massive simulations with rust + bevy

## Discussion

- Main reason GPU acceleration doesn't work as collision detection service is that number of collisions is unknown, but we have to pre-allocate memory when working with the GPU, leading to a lot of waste and slowdown
- Vs the CPU where we do not have to preallocate memory, so we only end up using the amount of memory necessary to hold the correct number of collisions

## How to improve GPU accelerated collision performance:

- If the maximum storage buffer size was much larger, this could be improved significantly
- Switch to integer positions and integer math instead of floating point
- If most or all of the simulation logic (movements and reactions to collisions) were moved to the GPU performance would improve and buffer size would not be a bottleneck

## How to improve CPU accelerated collision performance:

- In the same way that moving more logic onto the GPU would improve performance by decreasing data transfer and memory allocation costs, inlining logic on the CPU side has the same benefits. This requires prior knowledge of the entire simulation, and leads to tightly coupled, non-reusable code. But the performance gains are very significant.
