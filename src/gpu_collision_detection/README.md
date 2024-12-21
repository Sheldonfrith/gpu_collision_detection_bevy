# GPU Collision Detection Documentation

# Batching

Batching is implemented to ensure that we don't exceed the maximum size on the buffers used to transfer between CPU and GPU.

We cannot simply split all of the collidable entities into batches and send each batch to the GPU, since then we would miss collisions happening across batches, for that reason we have the algorithm in "generate_batch_jobs" which ensures the GPU always sees every possible combination of collidable entities, while at the same time trying not to send anything uneccesary.

For now these batches are run sequentially, not in parallel. For applications where the GPU is not otherwise being utilized heavily collision detection performance can definitely be improved by running the batches in parallel on the GPU.

# Missing Collisions / max_detectable_collisions_scale variable

We have to allocate the buffer memory to receive results from the GPU without knowing how many results we are going to receive. We know the upper limit of the number of results we are going to receive is all possible combinations of input collidable entities. However reserving that much memory every time leads to huge reductions in performance. If we dont allocate enough memory, on the other hand, then collisions are silently dropped.

The "max_detectable_collisions_scale" variable is multiplied by the maximum theoretical possible memory size of the results, and is used to reserve LESS than the maximum amount of memory in order to improve performance. The correct value for that variable is very hard to determine. I have used manual testing to come up with a very rough function describing what that variable should be, but it still most of the time overshoots signicantly, reducing performance.

The variable is held in a Bevy resource so if you are using this code I encourage you to mutate that value yourself, since you will know a lot more about the number of expected collisions for your scenario and therefore guess much better how much memory will be needed for results.
