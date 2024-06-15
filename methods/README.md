# zkVM Methods

This directory contains the [zkVM] portion of the [RISC Zero] application.

### From Guest Code to Binary File

Code in the `methods/guest` directory will be compiled into one or more binaries.

Build configuration for the methods is included in `methods/build.rs`.

Each will have a corresponding image ID, which is a hash identifying the program.


[zkVM]: https://dev.risczero.com/zkvm
[RISC Zero]: https://www.risczero.com/
[guest programs]: https://dev.risczero.com/terminology#guest-program
[on-chain logic]: ../contracts/
[guest/src/bin]: ./guest/src/bin/
[Guest Code 101]: https://dev.risczero.com/zkvm/developer-guide/guest-code-101
[RISC Zero examples]: https://github.com/risc0/tree/v0.18.0/examples
