# Rust Notes
## Table-of-Contents
- [Basic Commands](#basic-commands)
    - [Creating a New Project](#creating-a-new-project)
- [Datatypes](#datatypes)
- [Control Flow](#control-flow)
    - [Conditionals](#conditionals)
    - [Looping](#looping)
        - [For Loop](#for-loop)
        - [While Loop](#while-loop)
        - [Loop](#loop)
- [Functions](#functions)
- [Ownership](#ownership)
    - [Ownership Rules](#ownership-rules)
    - [Ownership Violation Fixes](#ownership-fixes)
- [Structs](#structs)
    - [Tuple Structs](#tuple_structs)
    - - [Methods](#methods)
- [Format Macro](#format-macro)
    - [Returning Strings from Functions](#format-macro-returning-strings)
- [Enums](#enums)
    - [Enums with Values](#enums-with-values)
    - [Pattern Matching](#enums-pattern-matching)
    
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

  ### Scalar Types

    #### Integer

    | Length | Signed | Unsigned | Range |
    |------|------|--------|-----|
    | 8-bit | i8 | u8 | -128 to 127 or 0 to 255 |
    | 16-bit | i16 | u16 | -32,768 to 32,767 or 0 to 65,535 |
    | 32-bit | i32 | u32 | -2,147,483,648 to 2,147,483,647 or 0 to 4,294,967,295 |
    | 64-bit | i64 | u64 | -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807 or 0 to 18,446,744,073,709,551,615 |

    ```rust
    // Integer with explicit type annotation
    let x: u32 = 42;

    // Integer with type inference
    let x = 42;    
    ```

    #### Float

    | Length | Signed | Unsigned |
    | ------ | ------ | -------- |
    | 32-bit | f32 | f64 |
    | 64-bit | f32 | f64 |

    ```rust
    // Float with explicit type annotation
    let x: f32 = 42.314;

    // Float with type inference
    let x = 42.314;    
    ```
    
    #### Boolean
    ```rust
    // Boolean with explicit type annotation
    let is_active: bool = true;

    // Boolean with type inference
    let has_permission = false;    
    ```
 
    #### Character
     ```rust
    // Character with explicit type annotation
    let x: char = 'R';

    // Character with type inference
    let emoji = '🚀';    
    ```

  ### Compound Types
    
    #### Tuple
    ```rust
    // Tuple with explicit type annotation
    let person_with_type: (&str, i32, bool) = ("Alice", 30, true);

    // Tuple with type inference
    let product_info = ("Laptop", 1200.0, 5); 

    // Printing all elements of a tuple
    println!("Person: {:?}", person_with_type);
    println!("Product Info: {:?}", product_info);

    // Accessing tuple elements
    let coordinates = (10, 25);

    // Access the first element
    let x = coordinates.0;

    // Access the second element
    let y = coordinates.1;

    // Print the coordinates
    println!("X-coordinate: {}", x);
    println!("Y-coordinate: {}", y);

    // Destructuring a Tuple
    let user_data = ("Bob", "bob@example.com", 42);

    // Destructuring the tuple into named variables
    let (name, email, age) = user_data;

    println!("Name: {}", name);
    println!("Email: {}", email);
    println!("Age: {}", age);

    // Returning a Tuple from a Function
    fn get_dimensions() -> (f64, f64) {
        (10.5, 20.0) // Returns a tuple of two f64 values
    }
    
    fn main() {
        let (width, height) = get_dimensions();
        println!("Width: {}, Height: {}", width, height);
    }
    
    ```
    
    #### Arrays
        - **Fixed Size**: Once declared, the size of an array cannot be changed.
        - **Homogeneous Type**: All elements in an array must be of the same data type.
        - **Stack Allocation**: Arrays are typically allocated on the stack, which can offer performance benefits.
        - **Compile-time Bounds Checking**: Rust performs bounds checking at compile time for array access, preventing out-of-bounds errors.
    ```rust
    // Array with explicit type and size
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // Array with type and size inferred by Rust
    let colors = ["red", "green", "blue"];

    // Array initialized with a default value for all elements
    let zeros: [u8; 10] = [0; 10]; 

    println!("Numbers: {:?}", numbers);
    println!("Colors: {:?}", colors);
    println!("Zeros: {:?}", zeros);

    // Accessing Array Elements
    let fruits = ["apple", "banana", "cherry"];

    // Accessing elements by index (0-based)
    println!("First fruit: {}", fruits[0]);
    println!("Second fruit: {}", fruits[1]);

    // Iterating through an Array
    let scores = [85, 92, 78, 95];

    // Using a for loop to iterate over elements
    for score in scores {
        println!("Score: {}", score);
    }

    // Iterating with index and value
    for (index, score) in scores.iter().enumerate() {
        println!("Index: {}, Value: {}", index, score);
    }

    // Mutable Arrays
    let mut data = [10, 20, 30];

    // Modifying elements in a mutable array
    data[0] = 15;
    data[2] = 35;

    println!("Modified data: {:?}", data);
    ```

    #### Slice Type
    - A special kind of reference that lets you access a contiguous sequence of elements in a collection.
        - For example, it can help in accessing a part of a string or part of a collection like tuples or arrays.
      ````rust
      let message = String::from("hello world");
      let hello = &message[0..5];
      println!("{}", hello);

      let array = [1, 2, 3, 4, 5];
      let slice = &array[1..4];
      for num in slice {
          println!("{)", num);
      }
      
      ````
 
<a name="casting"></a>
## Casting
```rust
fn main() {
    let pi:f32 = 3.14;
    let foo:i32 = pi as i32;
    
    let num:i32 = 5;
    let bar:f32 = num as f32;
    
    println!("{}", foo);
    println!("{}", bar);
}
```


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
        let arr = [10, 20, 30, 40, 50];
        for elem in arr {
            println!("{}", elem);
        }
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
  
<a name="functions"></a>
## Functions
- Functions start with the 'fn' keyword. 
- Rust naming conventions are that words that make up the function are lower-case and use snake-case formatting.
- Rust functions must always include data type for parameters.
  ```rust
  fn main() {
      let name = String::from("Alice");
      greet_user(name)
  }
  
  fn greet_user(name: String){
      println!("Hello {}, welcome to Rust Programming", name);
  }
  ```
  
- In Rust, lines that end with a semi-colon are called statements.
- Lines that do not end with a semi-colon are called expressions.
- We use expressions to return values from functions.
- Function headers specify that we want to return something and the return type using an '->' arrow and the return type.
  ```rust
  fn main() {
      let sum = calculate_sum(5, 10);
      println!("The sum is {}", sum);
  }

  fn calculate_sum(a:i32, b:i32) -> i32{
      let sum = a + b;
      sum
  }
  ```

<a name="ownership"></a>
## Ownership
- Set of rules for ***memory management***
- No garbage collection
- Memory is managed through a system of *ownership*
    - Set of rules that the compiler checks
    - If violated, the program won't compile
    - Ownership rules checked during compile time, ***Not Runtime***

<a name="ownership-rules"></a>
### Ownership Rules
- Each value in Rust has an exactly one owner
- There can be only one owner at a time
- When the owner goes out of scope, the value will be dropped
- Examples:
```rust
let s1 = String::from("hello");
let s2 = s1; // Ownership is transferred to s2 thus, s1 is no longer valid.
```

```rust
fn main() {
    let name = String::from("Rob");
    print_greeting(name); // Ownership of name is transferred to print_greeting() thus, name is no longer valid.
}

fn print_greeting(name: String) {
    println!("Welcome {name}");
}
```

<a name="ownership-fixes"></a>
### Ownership Violation Fixes
- **NOTE**: Transfer of ownership only occurs for complex types.
```rust
let i = 9;
let j = i; // No transfer of ownership takes place here!
println!("{}{}", i, j);
```

- Create a copy
```rust
let s1 = String::from("hello");
let s2 = s1.clone();
println!("{s1}, world!");
println!("{s2}, world!");
```

- Return ownership back to calling function
```rust
fn main() {
    let name = String::from("Rob");

    // Receive ownership back.
    let name = print_greeting(name);
    println!("{name}");
}

fn print_greeting(name: String) -> String {
    println!("Welcome {name}");
    name
}
```

- Use References
```rust
fn main() {
    let name = String::from("Rob");

    // Pass name by reference
    print_greeting(&name);
    println!("{name}");
}

fn print_greeting(name: &String){
    println!("Welcome {name}");
}
```

<a name="structs"></a>
## Structs
- Custom Data Type
- Package together multiple realted values in a meaningful group
- Structs are complex data types
    - Typically allocated from the heap
    - Subject to ownership rules
```rust
struct User {
    name: String,
    email: String,
    active: bool,
    sign_in_count: u64,
    date_of_birth: String,
}

fn print_user_message(user: &User){
    println!("Hello {}, you have signed in {} times.", user.name, user.sign_in_count);
}

fn main() {
    let user1 = User{
        name: String::from("Alice"),
        email: String::from("alice@example.com"),
        active: true,
        sign_in_count: 5,
        date_of_birth: String::from("1950-05-05"),
    };
    print_user_message(&user1);
}
```


<a name="tuple_structs"></a>
### Tuple Structs
- Structures in which you do not need to name each field.
```rust
// RGB Values
Struct Color (i32, i32, i32);

// x, y, z co-ordinates
Struct Point(f64, f64, f64);

fn main() {
    let red = Color(255, 0, 0);
    let p1 = Point(100.0, 25.5, 30.1);
}
```

<a name="methods"></a>
### Methods
- Functions associated with a struct
- Actions or operations used to perform an operation on a struct or using a struct
- For Example:
  - User
      - login, logout, print_user_info
  - Rectangle
      - calc_area, print_dimension,
  - Car
      - drive, refuel, service_vehicle
```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self)->u32 {
        self.width * self.height
    }
    
    fn print_dimensions(&self) {
        println!("Height {}, Width {}", self.height, self.width);
    }
}

fn main() {
    let rec1 = Rectangle{
        width: 25,
        height: 20,
    };
    
    let rec1_area = rec1.area();
    println!("Rectangle 1 Area: {}", rec1_area);
}
```

<a name="format-macro"></a>
## Format Macro
- A powerful tool for constructing strings.
- Particularly useful when you want to create a fromatted string for display or to return from a function.
- Similar to the println!() macro but returns the formatted string instead of printing it out.
- Advantages:
    - It allows for the creation of dynamic, formatted text without needing to concatenate strings manually.
    - It provides a clear and concise way to include variables in string output.
    - It returns a String, making it easy to store or return from functions.
- example:
```rust
fn main() {
    let name =  "Jane";
    let age = 30;
    let introduction = format!("Hi, my name is {} and I am {} years old.", name, age);
    println!("{}", introduction);
}
```
<a name="format-macro-returning-strings"></a>
### Returning Strings from Functions
- The format!() macro is sepecially useful when you need to build and return dynamic strings from functions.
```rust
fn create_greeting(name: &str, age: u8) -> String {
    format!("My name is {} and I am {} years old.", name, age)
}

fn main() {
    let introduction = create_greeting("Bob", 25);
    println!("{}", introduction);
}
```

<a name="enums"></a>
## Enums
- Allows you to create your own data-type by listing all possible variants.
```rust
enum TRafficLight {
    Red,
    Yellow,
    Green,
}
```

<a name="enums-with-values"></a>
### Enums with Values
```rust
enum Shape {
    Circle(f64), /* Radius */
    Rectangle(f64, f64), /* Length, Width */
    Square(i32), /* Side Length */
}
```

<a name="enums-pattern-matching"></a>
### Pattern Matching
- Must match each enum case.
```rust
fn calculate_area(shape: Shape) {
    match shape {
        Shape::Circle(radius) => println!("Area of circle is {}", 3.14 * radius * radius),
        Shape::Rectangle(width, height) => {
            let area = width * height;
            println!("Area of rectange is {}", area);
        },
        Shape::Square(side) => println!("Area of square is {}", side * side),
    }
}

fn main() {
    let light = TrafficLight::Red;
    let rect1 = Shape::Rectangle(30.0, 17.3);
    let square = Shape::Square(25);

    match light {
        TrafficLight::Red => println!("Stop!"),
        TrafficLight::Yellow => println!("Caution! Be Prepared to stop."),
        TrafficLight::Green => println!("Go!!"),
    }

    calculate_area(rect1);
    calculate_area(square);
}
```
