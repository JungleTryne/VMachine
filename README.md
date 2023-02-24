# Toy Virtual Machine

This is a toy virtual machine with von Neumann architecture.

## How to run it?
The machine consumes an image file which it used as a virtual memory device.
```bash
cargo run <PATH_TO_THE_IMAGE>
```

You can find an example image in `images` folder. For example, you can run 
`images/hello_world.bin` to print "Hello world" to the terminal.

The image **changes** during execution of the machine. It is recommended to
store a backup of the image before executing the machine.

## What is the architecture of the machine?
To learn about all the instructions and registers available, refer to 
[this doc](docs/instructions.md)
