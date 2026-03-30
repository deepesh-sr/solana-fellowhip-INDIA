// ========================================
// BOX<T> - Smart Pointer for Heap Allocation
// ========================================

// -----------------------------------------
// EXAMPLE 1: Basic Box - Stack vs Heap
// -----------------------------------------
fn basic_box() {
    // Normal variable - stack pe stored
    let stack_val = 42;

    // Box::new() - value heap pe jayegi, stack pe sirf pointer rahega
    let heap_val = Box::new(42);

    println!("Stack value: {}", stack_val);
    println!("Heap value: {}", heap_val); // auto-deref hota hai, seedha use karo

    // Manually dereference bhi kar sakte ho
    let inner = *heap_val;
    println!("Dereferenced: {}", inner);

    // heap_val yahan drop hoga — heap memory automatically free
}

// -----------------------------------------
// EXAMPLE 2: Box with large data
// -----------------------------------------
// Bada struct stack pe copy karna expensive hai
// Box se sirf pointer move hota hai (8 bytes)
fn large_data_box() {
    let big_array = Box::new([0u8; 1_000_000]); // 1MB heap pe gaya
    println!("Big array length: {}", big_array.len());

    // Move karo — sirf pointer move hota hai, 1MB copy nahi hoti
    let moved = big_array;
    println!("Moved array length: {}", moved.len());
    // big_array ab use nahi kar sakte — ownership move ho gayi
}

// -----------------------------------------
// EXAMPLE 3: Box ke bina Recursive Type NAHI ban sakta
// -----------------------------------------
// Ye code compile nahi hoga agar uncomment karo:
//
// enum BadList {
//     Cons(i32, BadList),   // ERROR: infinite size!
//     Nil,
// }
//
// Rust ko har type ka size compile time pe pata hona chahiye.
// BadList ka size = i32 + BadList = i32 + (i32 + BadList) = ... INFINITE!

// Box lagao — ab size fixed hai: i32 + pointer (8 bytes)
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn recursive_type_demo() {
    use List::{Cons, Nil};

    // List: 1 -> 2 -> 3 -> Nil
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("Linked List: {:?}", list);

    // List traverse karte hain
    let mut current = &list;
    loop {
        match current {
            Cons(val, next) => {
                print!("{} -> ", val);
                current = next;
            }
            Nil => {
                println!("Nil");
                break;
            }
        }
    }
}

// -----------------------------------------
// EXAMPLE 4: Trait Objects with Box (Dynamic Dispatch)
// -----------------------------------------
// Jab tumhe different types ko ek hi collection mein rakhna ho
trait Animal {
    fn speak(&self) -> &str;
    fn name(&self) -> &str;
}

struct Dog {
    name: String,
}
impl Animal for Dog {
    fn speak(&self) -> &str {
        "Bhow Bhow!"
    }
    fn name(&self) -> &str {
        &self.name
    }
}

struct Cat {
    name: String,
}
impl Animal for Cat {
    fn speak(&self) -> &str {
        "Meow!"
    }
    fn name(&self) -> &str {
        &self.name
    }
}

fn trait_object_demo() {
    // Vec<Box<dyn Animal>> — different types ek vector mein!
    // "dyn" = dynamic dispatch (runtime pe decide hoga kaunsa method call ho)
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog { name: String::from("Tommy") }),
        Box::new(Cat { name: String::from("Billu") }),
        Box::new(Dog { name: String::from("Moti") }),
    ];

    for animal in &animals {
        println!("{} says: {}", animal.name(), animal.speak());
    }
}

// -----------------------------------------
// EXAMPLE 5: Box with Pattern Matching
// -----------------------------------------
#[derive(Debug)]
enum Expr {
    Num(f64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

fn eval(expr: &Expr) -> f64 {
    match expr {
        Expr::Num(n) => *n,
        Expr::Add(a, b) => eval(a) + eval(b),
        Expr::Mul(a, b) => eval(a) * eval(b),
    }
}

fn expression_tree_demo() {
    // Expression: (2 + 3) * 4
    let expr = Expr::Mul(
        Box::new(Expr::Add(
            Box::new(Expr::Num(2.0)),
            Box::new(Expr::Num(3.0)),
        )),
        Box::new(Expr::Num(4.0)),
    );

    println!("Expression: {:?}", expr);
    println!("Result: (2 + 3) * 4 = {}", eval(&expr));
}

// -----------------------------------------
// MAIN - Sab examples run karo
// -----------------------------------------
pub fn run() {
    println!("=== 1. Basic Box ===");
    basic_box();

    println!("\n=== 2. Large Data Box ===");
    large_data_box();

    println!("\n=== 3. Recursive Type (Linked List) ===");
    recursive_type_demo();

    println!("\n=== 4. Trait Objects (Dynamic Dispatch) ===");
    trait_object_demo();

    println!("\n=== 5. Expression Tree ===");
    expression_tree_demo();
}
