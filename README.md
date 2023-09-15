## What is this?

This is a very simple ray-tracer that uses the Möller–Trumbore intersection algorithm to create a scene made of triangles.

It uses the minifb library to create a window and draw to it and take keyboard input.

Heres an example render of the famous Utah Teapot, which is contained in the model file in the main directory and can be viewed by typing `cargo run model` in the main project directory and waiting several minutes.

![Utah Teapot render](example_output.png "The Utah Teapot as rendered by this application")

## How do you use it?

This application uses a propriety model format sort of like an .obj file, where you define vertices like:

`vertex <x> <y> <z>`

And triangles like

`triangle <vertex 1> <vertex 2> <vertex 3> <red> <green> <blue> <specular>`

Where `x`, `y` and `z` are floats representing a co-ordinate in 3d space, and `vertex 1`, `vertex 2` and `vertex 3` are 1-based indices of the 3 vertices that make up the triangle with respect to the order they appeared in the file. `red`, `green` and `blue` are integers between 0 and 255 that make up an RGB color value the triangle will be coloured with, and `specular` is a float indicating the specular lighting intensity of this triangle.

Run the program with `cargo run <file>`.

## Misc

The algorithm is incredibly slow, as for each pixel we 'cast' a ray for which we check every single triangle for an intersection, it can take minutes to draw the teapot example above. I'm probably going to optimise it to use something like an octree or a k-d tree to optimise the process of choosing triangles to test for intersection.

Texture mapping using barycentric coordinates would also be a good next step.

## Credit

['Computer Graphics From Scratch' for the basic principles](https://nostarch.com/computer-graphics-scratch)

['Geometry For Programmers' for more advanced 3D math](https://www.manning.com/books/geometry-for-programmers)

[This wikipedia article on the triangle intersection algorithm](https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm)