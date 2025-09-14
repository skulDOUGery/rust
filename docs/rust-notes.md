# Rust Notes
## Table-of-Contents
- [Basic Commands](#basic-commands)
    - [Creating a New Project](#creating-a-new-project)
- [Control Flow](#control-flow)
    - [Conditionals](#conditionals)
    - [Looping](#looping)
        - [For Loop](#for-loop)
        - [While Loop](#while-loop)
        - [Loop](#loop)

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

<a name="control-flow"></a>
## Control Flow

<a name="conditionals"></a>
### Conditionals
```rust
fn main() {
    let number = 35;
    if number % 5 == 0 && number % 3 == 0 {
        println!("{} is a TriQuint", number);
    } else if number % 6 == 0 && number % 4 == 0 {
        println!("{} is a HexaQuad", number);
    } else {
        println!("{} is just another number", number);
    }
}
```

```rust
fn main() {
    let is_weekend:bool = true;
    let activity = if is_weekend {"Go Hiking"} else {"Go to work"};
    println!("{} is today's activity", activity);
}
```

<a name="looping"></a>
### Looping

<a name="for-loop"></a>
#### For Loop
- Used to iterate over a collection such as an array.
    ````rust
    fn main() {
    
    }
    ````

<a name="while-loop"></a>
#### While Loop
- Used to loop until a condition is met.
    ```rust
    fn main() {
        let mut counter = 10;
        while counter > 0 {
            println!("Countdown: {}", counter);
            counter -= 1;
        }
        println!("LIFT OFF!");
    }
    ```
<a name="loop"></a>
#### Loop
- Used to specify an infinite loop.
- The `break` statement can be used to exit the loop.
  ```rust
  fn main() {
      let mut index = 1;
      loop {
          index += 1;
          println!("Index: {}", index);

          if index == 100 {
              println!("Max index reached");
              break;
      }
  }
  ```

