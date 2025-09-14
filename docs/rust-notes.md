# Rust Notes
## Table-of-Contents
- [Basic Commands](#basic-commands)
    - [Creating a New Project](#creating-a-new-project)

<a name="basic-commands"></a>
## Basic Commands

<a name="creating-a-new-project"></a>
### Creating a New Project
```
$ cargo new <new-project>
```

<a name="running-a-project"></a>
### Running a Project
```
$ cargo run
```

<a name="datatypes"></a>
## Datatypes
- Statically Typed Language
- Scalar
    - Integer
| Length | Signed | Unsigned | Range |
|------|------|--------|-----|
| 8-bit | i8 | u8 | -128 to 127 or 0 to 255 |
| 16-bit | i16 | u16 | -32,768 to 32,767 or 0 to 65,535 |
| 32-bit | i32 | u32 | -2,147,483,648 to 2,147,483,647 or 0 to 4,294,967,295 |
| 64-bit | i64 | u64 | -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807 or 0 to 18,446,744,073,709,551,615 |

    - Float
    | Length | Signed | Unsigned |
    | ------ | ------ | -------- |
    | 32-bit | f32 | f64 |
    | 64-bit | f32 | f64 |
    
    - Boolean
    - Character
- Compound
    - Tuple
    - Arrays
