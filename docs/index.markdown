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

1. An example codebase showing implementation of GPU accelerated collision detection (narrow-phase only) and standard CPU collision detection. Designed to realistically test the performance differences between each technique. You can run the code yourself and see the difference, here is the repo:
   [![Repo](https://img.shields.io/badge/github-%23121011.svg?style=for-the-badge&logo=github&logoColor=white)](https://github.com/Sheldonfrith/gpu_accelerated_collision_detection)

2. A report on the results of comparison testing using the above codebase. Here is the report...

# TLDR

- GPU acceleration can provide major performance increases over CPU-based collision detection.

- Significant improvements start at around 15k collisions per frame/iteration/step and reaching up to a 50% performance improvement for between 40k and 200k collisions per frame, with the improvements plateauing after that point.
- Current version of the code shows slight decreases in improvements for larger scales (25% at 10 million collisions per frame), but there are optimizations that can be done to the algorithm which would likely eliminate that decline with scale.

## Who this is useful for

- People using [Rust](https://www.rust-lang.org/) for...
  - Game development
  - Simulations

Especially if you are already using the [Bevy engine](https://bevyengine.org/), although the code can be adapted to work with any rust codebase.

## Rationale for Creating

I needed performant collision detection for a much larger scale than normal; hundreds of thousands of simultaneously colliding entities, at least. I tried popular existing collision detection solutions, but my brief testing indicated that their performance was unacceptable for the scales I required.

# Performance Results:

### Raw Frame Time:

![Full Frame Time vs Collisions per Frame Comparison Graph](/assets/images/FullFTvsCPF.png)

And here is a zoomed version to show the critical point where GPU acceleration becomes valuable:
![Zoomed Frame Time vs Collisions per Frame Comparison Graph](/assets/images/ZoomedFTvsCPF.png)

### % Frame Time Reduction using GPU:

![Full Frame Time Reduction vs Collisions per Frame Graph](/assets/images/FullFTRvsCPF.png)
_Note the logarithmic scale of the x axis ABOVE._

And here is a zoomed version to show in more detail the point where GPU acceleration becomes valuable (NOT log scale):
![Zoomed Frame Time Reduction vs Collisions per Frame Graph](/assets/images/ZoomedFTRvsCPF.png)

# Caveats:

- This technique will probably not provide benefits for web-based applications that do not have low level GPU access.
- If you are using this for a videogame that already has _very_ intensive graphics, there might not be enough extra capacity on the GPU to handle this method. However for most games the extra GPU usage shouldn't be an issue. Collision detection for 160k collisions per frame, for example, used only about 7% of my GPU capacity (RTX 3070 laptop version).

## Narrow vs Broad Phase

See [narrow vs broad phase](https://developer.mozilla.org/en-US/docs/Games/Techniques/2D_collision_detection#collision_performance).

This technique is only for **narrow-phase** of collision detection. If performance is an issue, you should first prioritize implementing a performant broad-phase to your collision detection as this is an easier way of making big performance gains. And for truly massive simulations a broad phase is required, because of the practical limits of narrow-phase collosion detection.

## Practical Upper Limits on Collisions/Frame

- around 200-300k collisions per frame for videogame applications. Total collisions can be much higher if you also implement broad-phase filtering (see above).
- tens or hundreds of millions of collisions per frame, limited only by the RAM available for storing all of the collision pairs.

# Further Performance Improvements

- The main waste with GPU based collision detection is having to pre-allocate a lot of memory which we don't actually end up using, since we don't know ahead of time the number of collisions that will be detected. Testing indicates anything that can be done to pre-estimate the number of collisions that will be detected yields large performance improvements (this is the purpose of the `max_detectable_collisions_scale` variable in the GPU code, but a lot of improvements can still be made to that part of the code).
- A major bottleneck is the render device's maximum storage buffer size. If there is a way to safely increase this buffer size limit, performance can be dramatically improved. (This may be hardware limited, I haven't had the time to look into it yet.)
- Switch to integer positions and integer math instead of floating point. This requires client code to use integer positions, which is less convenient, which is why the code currently uses floating point positions.
- For simulations, running batches in parallel may be possible as a method to utilize more of the GPU.

## Inlining

You may notice that simply combining the collision processing with the collision detection can dramatically improve the CPU algorithm's speed so that it is actually faster than the GPU method. However we have to keep in mind that **the same thing can be said for the GPU algorithm**. If we also put collision processing directly onto the GPU we will also gain dramatic performance improvements.

If you are trying to get the absolute best possible performance in your application you will probably have to use this strategy, but otherwise you should avoid it because it creates highly coupled, difficult to maintain code.

I have not done this for either CPU or GPU because I want this test to be representative of the general case, where we don't know what exactly the client is going to do with the collisions detected.
