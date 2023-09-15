## What is this?

This is a very simple ray-tracer that uses the Möller–Trumbore intersection algorithm to create a scene made of triangles.

Heres an example render of the famous Utah Teapot (contained in the model file in the main directory)

![Utah Teapot render](example_output.png "The Utah Teapot as rendered by this application")

This application uses a propriety model format sort of like an .obj file, where you define vertices like:

`vertex <x> <y> <z>`

And triangles like

`triangle <vertex 1> <vertex 2> <vertex 3> <red> <green> <blue> <specular>`

Where `x`, `y` and `z` are floats representing a co-ordinate in 3d space, and `vertex 1`, `vertex 2` and `vertex 3` are 1-based indices of the 3 vertices that make up the triangle with respect to the order they appeared in the file. `red`, `green` and `blue` are integers between 0 and 255 that make up an RGB color value the triangle will be coloured with, and `specular` is a float indicating the specular lighting intensity of this triangle.