# Fractal Viewer

This is a mandelbrot set arbitaray precision fractal viewer written for fun bc they look cool.
It uses my own, probably incorrect fixed point number implementation in both rust and wgsl.
The color function just rotates the hue as the iterations go up, from red to green to blue and back to red.
Regions where it's unclear whether it diverges are drawn black, and updated if it finds divergence.
When you zoom in far enough, it will automatically update the fixed point precision and recompile the shader so the quality doesn't drop off; it will go as far as your GPU / wgpu lets it.
It also redraws all pixels every time you move or zoom.
There are basically no optimizations other than each iteration tries to do minimal fixed point operations, which does not include copying, but you can actually get pretty deep with reasonable draw time.

Controls:
 - WASD for movement
 - scroll to zoom
 - Q to take a snapshot

Snapshots will copy the current texture and let you view it as the new one generates, which is very important for your sanity when you zoom in really far; the undecided regions will be replaced with a darkened version of your snapshot, so you can still know where you are and move around.
