# php-analyzer


# Analysis strategy

The analysis is performed as a a multi-iteration scan of the source code. 

The strategy is to emit determinate issues as early as possible, making 
it possible for users of the the analyser to provide quicker feedback.

We attempt to report problems as close to the root-cause as possible.

## Actual implementation

The steps is as follows
1. Parsing (tree sitter)
2. AST Conversion (convert from general tree-sitter-tree to a native typed rust tree)
3. First pass
4. Second pass
5. Iterative third passes
6. Forth pass
## 1. Parsing and 2. Conversion

First a file is parsed using tree-sitter-php and a concrete syntax tree is constructed.

This is converted into an internal abstract-syntax-tree-representation in rust.

Then this three is scanned trough multiple passes. Each pass performs a specified part of the analysis

## 3. Round one

The first pass is analyzing all basic "local" knowledge.

It can check that a single node is not in violation of any rules, naming conventions and so forth.
Rejecting usage of certain constructs.

This can not rely on any contextual knowledge, as this is probably not available.

This will register all declared classes, method, functions, constants and similar.

## 4. Second pass
When all type-declarations are found, we can emit errors for all usages of unknown basic types

## 5. Iterative third pass (was Round two)

This does more context- and state-aware analysis to more precisely determine
types of variables and return-types.

This pass might be ran multiple times until the symbol_data-stabilizes, or an a probability of an oscilation is detected

## 6. Forth pass

The final pass is used to emit violations using the most precise type-information we got from the previous analysis

The forth pass can not modify the symbol-table
