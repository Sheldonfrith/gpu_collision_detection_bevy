# GPU Collision Detection Documentation

The key thing to talk about is batching.

Batching is implemented to ensure that we don't exceed the maximum size on the buffers used to transfer between CPU and GPU.

We cannot simply split all of the collidable entities into batches and send each batch to the GPU, since then we would miss collisions happening across batches, for that reason we have the algorithm in "generate_batch_jobs" which ensures the GPU always sees every possible combination of collidable entities, while at the same time trying not to send anything uneccesary.
