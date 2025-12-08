# rpgtool
---

A tool designed to convert rxdata/rvdata/rvdata2 files to text formats!!
Actually, this tool can handle converting *any* marshal data, not just RPG Maker data, but `rpgtool` is designed with RPG Maker in mind.

This project contains two binaries- `rpgtool` and `marshalconvert`. 
`marshalconvert` is a general purpose tool for converting marshal to any of this project's supported formats,
whereas `rpgtool` is a suite of tools for working with RPG Maker projects.

Currently supports converting to `JSON`, `Yaml`, and `Ron`!

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

# Representation

Due to format limitations, `rpgtool` and `marshalconvert` have a special representation for different Ruby types. This representation uses `$` tags to distinguish between objects, hashes, and structs!

For example, if you want to represent an object, you need to do it like this:
```json
{
  "$object": {
    "class": "MyClass",
    "fields": {
      "@foo": true,
      "@bar": 400,
      "@baz": [0, 1, 2, 3, 4]
    }
  }
}
```
You need to specify the class of the object as well as its fields. Instance variables should be prefixed with an `@` (like in Ruby)!
Structs are represented the same way, but use `$struct` instead of `$object`.

Non `@`-prefixed fields are allowed and valid butg are usually hidden from Ruby.
Encoding is handled like this- any encodings are stored with `E`. (See Instance for a JSON representation of this.)
Ascii strings have no `E` field, UTF-8 strings have `E` set to `true`, and everything else has `E` set to a string.

### Symbols
```json
{ "$symbol": "a_symbol" }
```

### Hashes
```json
{ "$hash": { "a": 0, "b":" abcd" } }
```

### Userdata 
Classes that have opted to be serialized with custom binary data are represented like this. Color/Table/Tone from RGSS all do this!
```json
{ "$userdata": { "class": "Table", "data": [0,1,2,3] } }
```

### Instance
Instance is how marshal represents types like strings with instance variables.
This is usually how you'll see non-UTF-8 strings represented!
```json
{ "$instance": { "value": "a string", "fields": { "E": true } } }
```

### Regex
```json
{ "$regex": { "data": "[0-9]*", "flags": 0 } }
```

### Classes/Modules
```json
{ "$class": "MyClass" }
{ "$module": "MyModule" }
```

### Extended
Any objects extended by modules at runtime are represented like this.
```json
{ "$extended": { "module": "MyModule", "value": 1234 } }
```

### Userclass/Usermarshal/Data
Not really sure what all of these are for, but these are objects that have been serialized as another class.
```json
{ "$userclass": { "module": "MyClass", "value": 1234 } }
```
`$userclass`/`$userdata`/`$cdata` are all represented like this!