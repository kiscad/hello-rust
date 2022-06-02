you can override rust's choice of in-memory representation by adding a #[repr] attribute to the enum.

casting a c stylee enum to an integer is allowed.

however, casting in the other direction, from the integer to the enum, is not. Unlike C, rust guarantees that an enum value is only ever one of the values spelled out in the enum declaration.

the more intersting sort of rust enum is the kind whose variants hold data. we will show how these are stored in memory, how to make them generic by adding type parameters, and how to build complex data strucutres from enums.

## Enums with Data

Some programs always need to display full dates and tmes down to the milliseconds, but for most applications, it's more user friently do use a rough approximation, like two months ago. we can write an enum to help with that, using the enum defined earlier.

## Enums in Memory

In memory, enums with data are stored as a small integer tag, plus enough memory to hold all the fields of the largest variant. The tag field is for Rust's internal use. It tells which constructor created the value and therefore which fields it has.

Rust makes no promises about enum layout, however, in order to leave the door open for future optimizations.In some cases, it would be possible to pack an enum more efficiently than the figure suggests. for instance, some generic structs can be stored without a tag at all, as we will see later.

## Rich Data Structures Using Enums

enums are also useful for quickly implementing tree-like data structures. For example, suppose a rust rpgoram needs to work with arbitrary json data. in memory, any json document can be represented as a value f this rust type.

one unobvious detail is that rust can eliminate the tag field of Option<T> when the type T is a reference, Box, or other smart pointer type. Since none of those pointer types is allowed to be zero, Rust can represent Option<Box<i32>>, say, as a single machine word: 0 for None, and non zero for Some pointer. This makes such Option types close analogues to C or C++ pointer values that could be null. The difference is that rust's type system requires you to check that an option is Some before you can use its contents. This effectively eliminates null pointer dereferences.

Generic data structures can be built with just


