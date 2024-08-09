# A Prototype Verus Project to Verify Asterinas
Currently include some simplified code from memory pages.
## How to run
Configure your verus settings following [build instructions](https://github.com/verus-lang/verus/blob/main/INSTALL.md) and run

`verus src/lib.rs`

## Project Structure

The project structure mainly follows the original structure of [Asterinas](https://github.com/asterinas/asterinas.git) with several modifications and augmentations:

```
src
│
├─mm
│  │  reexport.rs          //Re-export all relevant definitions in mm for proofs
│
├─proofs                   //Specifications and proofs of functions in mm
│
└─veriastlib               //Definitions used in proofs
```

