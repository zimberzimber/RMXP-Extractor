# rpgtool
---

A tool designed to convert rxdata/rvdata/rvdata2 files to text formats!!
Actually, this tool can handle converting *any* marshal data, not just RPG Maker data, but `rpgtool` is designed with RPG Maker in mind.

This project contains two binaries- `rpgtool` and `marshalconvert`. 
`marshalconvert` is a general purpose tool for converting marshal to any of this project's supported formats,
whereas `rpgtool` is a suite of tools for working with RPG Maker projects.

# UTF-8

Currently, this project is designed to work with UTF-8. 
It won't choke on non-UTF-8 strings, but it'll try and interpret them as UTF-8 anyway!

# Is it flawless?

**No.** But it is very close.
This project is backed by `alox-48`, which produces identical marshal as Ruby in most cases. 
However, `alox-48` does not serialize object links, which is where the two differ. `alox-48` can still deserialize them just fine though!

Object links are marshal's way of handling shared ownership. 
If the same object appears multiple times in marshal data, Ruby will serialize any repeat occurences as a pointer to the object.
This breaks reference cycles and allows for complex ownership structures.

Fortunately, RPG Maker data doesn't really rely on object links. Shared ownership does come up sometimes though!