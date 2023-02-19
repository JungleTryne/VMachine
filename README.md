# Toy Virtual Machine

This is a toy virtual machine with fon neyman architecture.

## How to run it?
The machine consumes an image file which it used as a virtual memory device.
```bash
cargo run <PATH_TO_THE_IMAGE>
```
You can find an example image in `images` folder. For example, you can run 
`images/hello_world.bin` to print "Hello world to the terminal.

## What is the architecture?
To learn about all the instructions available, refer to 
[this doc](docs/instructions.md)