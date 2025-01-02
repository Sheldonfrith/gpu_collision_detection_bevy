# GPU-Accelerated Collision Detection

### By [Sheldon Frith](https://sheldonfrith.com)

_Rust + [Bevy](https://bevyengine.org/) implementation of GPU accelerated collision detection and standard CPU collision detection (narrow phases only), designed to **realistically test the performance differences between each technique**._

![Video of Program Running](/project/assets/images/CollisionsGifCropped.gif)

# Table of Contents:

1. [How to run](#how-to-run)
2. [Who is this for?](#who)
3. [Performance Testing Results](#results)
4. [Rationale](#rationale)
5. [Caveats](#caveats)
6. [Further Improvements](#improvements)

<a id="how-to-run"></a>

# How to Run:

1. Install the latest nightly version of Rust with rustup
2. Clone the repo
3. `Cargo run --release` from the "project" directory, to test that it compiles properly
4. Go through the "run_tests.ipynb" jupyter notebook (requires python 3) to replicate my full comparison tests.

<a id="who"></a>

# Who this is useful for?

People using [Rust](https://www.rust-lang.org/) for...

- 🎮 Game development
- 🧬 Simulations

... needing faster ⏩ collision detection.

Especially if you are already using the [Bevy engine](https://bevyengine.org/), although the code can be adapted to work with any rust codebase.

<a id="results"></a>

# Performance Results:

## TLDR

GPU acceleration can provide major performance increases over CPU-based collision detection ${\textsf{\color{lightgreen}starting at around 15k collisions per frame}}$ and higher.

## % Frame Time Reduction using GPU:

![Full Frame Time Reduction vs Collisions per Frame Graph](/project/assets/images/FullFTRvsCPF.png)
_Note the logarithmic scale of the x axis ABOVE._

And here is a zoomed version to show in more detail the point where GPU acceleration becomes valuable (NOT log scale):
![Zoomed Frame Time Reduction vs Collisions per Frame Graph](project/assets/images/ZoomedFTRvsCPF.png)

## Raw Frame Time:

![Full Frame Time vs Collisions per Frame Comparison Graph](/project/assets/images/FullFTvsCPF.png)

Slightly more zoomed-in:
![Medium Zoomed Frame Time vs Collisions per Frame Comparison Graph](project/assets/images/MediumFTvsCPF.png)

And here is a fully zoomed-in version to show the critical point where GPU acceleration becomes valuable:
![Zoomed Frame Time vs Collisions per Frame Comparison Graph](project/assets/images/ZoomedFTvsCPF.png)

<a id="rationale"></a>

# Rationale 💡

I needed performant collision detection for a much larger scale than normal; hundreds of thousands of simultaneously colliding entities, at least. I tried popular existing collision detection solutions, but my brief testing indicated that their performance was unacceptable for the scales I required.

<a id="caveats"></a>

# Caveats:

## Not Suitable For:

- ❌ **Web-based applications**, because they don't have low level GPU access.
- ❌ Games with **very high existing GPU usage**. However,
  ${\textsf{\color{lightgreen}However, for most games the extra GPU usage is small enough to not be a problem:}}$ Collision detection for 160k collisions per frame, for example, used only about **7% of GPU capacity** (RTX 3070 laptop version).

## Narrow vs Broad Phase

See [narrow vs broad phase](https://developer.mozilla.org/en-US/docs/Games/Techniques/2D_collision_detection#collision_performance).

This technique is only for **narrow-phase** of collision detection. If performance is an issue, you should first prioritize implementing a performant broad-phase to your collision detection as this is an easier way of making big performance gains. And for truly massive simulations a broad phase is required, because of the practical limits of narrow-phase collosion detection.

## Practical Upper Limits on Collisions/Frame

- around 200-300k collisions per frame for videogame applications. Total collisions can be much higher if you also implement broad-phase filtering (see above).
- tens or hundreds of millions of collisions per frame, limited only by the RAM available for storing all of the collision pairs.

<a id="improvements"></a>

# Further Performance Improvements

- The main waste with GPU based collision detection is having to pre-allocate a lot of memory which we don't actually end up using, since we don't know ahead of time the number of collisions that will be detected. Testing indicates anything that can be done to pre-estimate the number of collisions yields large performance improvements (this is the purpose of the `max_detectable_collisions_scale` variable in the GPU code, but a lot of improvements can still be made to that part of the code).
- A major bottleneck is the render device's maximum storage buffer size. If there is a way to safely increase this buffer size limit, performance can be dramatically improved. (This may be hardware limited, I haven't had the time to look into it yet.)
- Switch to integer positions and integer math instead of floating point. This requires client code to use integer positions, which is less convenient, which is why the code currently uses floating point positions.
- For truly massive simulations, running batches in parallel in order to utilize more of the GPU may be possible.

## Inlining

You may notice that simply combining the collision processing with the collision detection can dramatically improve the CPU algorithm's speed so that it is actually faster than the GPU method. However we have to keep in mind that **the same thing can be said for the GPU algorithm**. If we also put collision processing directly onto the GPU we will also gain dramatic performance improvements.

If you are trying to get the absolute best possible performance in your application you will probably have to use this strategy, but otherwise you should avoid it because it creates highly coupled, difficult to maintain code.

**I have not done this, for either CPU or GPU, because I want this test to be representative of the general case, where we don't know what exactly the client is going to do with the collisions detected.**
