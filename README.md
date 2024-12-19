# gpu_accelerated_collision_detection

Comparing performance of naive GPU vs CPU collision detection for the narrow-phase of massive simulations with rust + bevy

# Narrow vs Broad Phase

As mentioned above this code is only for NARROW-PHASE of collision detection. If performance is an issue, you should first prioritize implementing a performant broad-phase to your collision detection as this is an easier way of making big performance gains. And for truly massive simulations a broad phase is required, because of the practical limits of narrow-phase collosion detection.

#### Limits of Narrow-Only collision detection

Using this program to test we can see that if you are using collision detection in a game if you are getting about 500k collisions per frame performance drops to between 10-20 fps under ideal conditions. So if you're game needs to handle that many collisions you have to implement some sort of broad-phase.

Even if you are running a scientific simulation and dont care about fps, you will still benefit greatly from implementing a broad-phase collision detection pass.

## Discussion

- Main reason GPU acceleration doesn't work as collision detection service is that number of collisions is unknown, but we have to pre-allocate memory when working with the GPU, leading to a lot of waste and slowdown
- Vs the CPU where we do not have to preallocate memory, so we only end up using the amount of memory necessary to hold the correct number of collisions

## How to improve GPU accelerated collision performance:

- If the maximum storage buffer size was much larger, this could be improved significantly
- Switch to integer positions and integer math instead of floating point
- If most or all of the simulation logic (movements and reactions to collisions) were moved to the GPU performance would improve and buffer size would not be a bottleneck
- run batches in parallel, since generally GPU is very underutilized, however this method requires some work to avoid stack overflows and memory shortages

## How to improve CPU accelerated collision performance:

- In the same way that moving more logic onto the GPU would improve performance by decreasing data transfer and memory allocation costs, inlining logic on the CPU side has the same benefits. This requires prior knowledge of the entire simulation, and leads to tightly coupled, non-reusable code. But the performance gains are very significant.
