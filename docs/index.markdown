---
# Feel free to add content and custom Front Matter to this file.
# To modify the layout, see https://jekyllrb.com/docs/themes/#overriding-theme-defaults
layout: home
title: "Massive Scale Collision Detection with Bevy"
---

# Massive-Scale Narrow-Phase Collision Detection with GPU Acceleration in Bevy

#### _For simulations and games_

##### By [Sheldon Frith](https://sheldonfrith.com)

# What is this?

An example codebase showing implementation of GPU accelerated collision detection (narrow-phase only) and standard CPU collision detection. Designed to realistically test the performance differences between each technique. You can run the code yourself and see the difference, or read this article where I discuss the results of my comparison testing.

## See the code:

- link to github repo here with github icon

# TLDR

GPU acceleration is not hard to implement and can provide major performance increases over standard CPU collision detection. Significant improvements start at around 15k collisions per "frame" ("frame" = "iteration"/"step" if you are running a simulation and not a game) and reaching up to a 50% performance improvement for between 40k and 200k collisions per frame, with the improvements plateauing after that point. Current version of the code shows slight decreases in improvements for larger scales (25% at 10 million collisions per frame), but there are optimizations that can be done to the algorithm which would likely eliminate that decline with scale.

## Who this is useful for

- People using [Rust](https://www.rust-lang.org/) for...
  - Game development
  - Simulations

Especially if you are already using the [Bevy engine](https://bevyengine.org/), although the code can be adapted to work with any rust codebase.

## Rationale for Creating

I needed performant collision detection for a much larger scale than normal (hundreds of thousands of simultaneously colliding entities at least). I tried popular existing collision detection solutions (like Avian, Rapier) but my brief testing indicated they probably weren't built for massive simulations like I was working with (although they work great for most game applications). And I only needed collision detection, not a physics engine.

# Caveats

- This technique will probably not provide benefits for web-based applications that do not have low level GPU access.
- If you are using this for a videogame that already has very intensive graphics, there might not be enough extra capacity on the GPU to handle this method.

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
