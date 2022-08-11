# Offchain worker beginner tutorial
This tutorial assumes you have basics of Rust and able to read and understand Rust code. But everything in this tutorial has been explained in-line.
* Offchain workers enable the runtime to communicate with outside world.
* The OCW is running on a node during block importation.
* OCW cannot mutate state by itself , it must submit a transaction in order to change the state.
* Runtime interfaces allow the sandboxed runtime environment to tap in native code and certain execution, hence OCW leverages this.


All the neccessary explanation on traits and types are in-line.
* Three functionalities are implemented and going to be implemented.
*
     1.Fecthing data externally
     2.Defining an extrinsic used in a transaction call
     3.Implementing a function for sending a signed transaction

## Steps to follow for this tutorial

1. Clone this template.
2. Modify lib.rs file by following the instruction
3. The solution is at tutorial.rs file
4. Run cargo test after the modification.

Any question go to [stack-exchange](https://substrate.stackexchange.com/questions).
or raise an issue here.
Personal account go to [twitter](https://twitter.com/LukambaMrisho).

