# qrust

1. `parse`: Compiles `Qrust.g4` into a node visitor using antlr4rust
2. `ast`: Creates `ASTVisitor`, exhausts the tree and creates a base untyped AST with `ASTNode`
3. `typed`: Travels across the untyped AST and applies type TypeAlgebra rules to each node recursively, generated `TypedNode`
4. `clif`: Travels the typed AST and and transforms it into clif using cranelift
5. `compile`: Compiles the generated clif code into an executable binary

`run` serves as an entrypoint to the program, taking an input file and sending it through the pipeline.

Current progress:

- [X] `parse`
- [X] `ast`
- [X] `typed`
- [ ] `clif`
- [ ] `compile`

Next step: Finish implementing the `builder` calls for the basic AST Node types.

Feature checklist:

- [ ] Functions
- [ ] Actual sized integers and floats (currently, everything is 64 bit) (see clif/src/type_layouts/float.rs and integer.rs)
- [ ] ...
