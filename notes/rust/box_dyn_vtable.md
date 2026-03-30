# Box<T>, dyn, and VTable in Rust

---

## 1. Box<T> — Smart Pointer for Heap Allocation

`Box<T>` value ko **heap** pe store karta hai, stack pe sirf ek pointer (8 bytes) rehta hai.

```
Stack:          Heap:
┌────────┐     ┌────────────┐
│ pointer │───>│ actual data │
│ 8 bytes │     └────────────┘
└────────┘
```

### Kab use karte hain?

### a) Large Data

Stack ka size limited hota hai (~1-8 MB). Bada data heap pe daalo:

```rust
let big = Box::new([0u8; 1_000_000]); // 1MB heap pe
// Move karo — sirf 8 byte pointer move hota hai, 1MB copy nahi
let moved = big;
```

### b) Recursive Types

Bina Box ke recursive type ka size infinite ho jaata hai:

```rust
// YE COMPILE NAHI HOGA:
enum BadList {
    Cons(i32, BadList),   // size = i32 + BadList = i32 + (i32 + BadList) = ... INFINITE
    Nil,
}

// Box lagao — ab size fixed hai: i32 (4 bytes) + pointer (8 bytes)
enum List {
    Cons(i32, Box<List>),
    Nil,
}
```

### c) Trait Objects (neeche detail mein)

---

## 2. dyn Keyword — Dynamic Dispatch

`dyn` ka matlab: method call **runtime** pe decide hoga, compile time pe nahi.

### Problem

```rust
trait Animal {
    fn speak(&self) -> &str;
}
struct Dog;
struct Cat;
impl Animal for Dog { fn speak(&self) -> &str { "Bhow!" } }
impl Animal for Cat { fn speak(&self) -> &str { "Meow!" } }
```

Vec mein sab elements ka size same chahiye. `Dog` aur `Cat` alag size ke hain:

```rust
// YE NAHI CHALEGA:
let animals: Vec<Animal> = vec![Dog, Cat]; // ERROR: size unknown at compile time
```

### Solution: Box<dyn Trait>

```rust
let animals: Vec<Box<dyn Animal>> = vec![
    Box::new(Dog),  // pointer (16 bytes) — same size
    Box::new(Cat),  // pointer (16 bytes) — same size
];

for a in &animals {
    println!("{}", a.speak()); // runtime pe decide hoga kaunsa speak()
}
```

---

## 3. VTable (Virtual Table) — Internal Mechanism

Jab `Box<dyn Animal>` likhte ho, Rust ek **fat pointer** banata hai:

```
Box<dyn Animal> = (data_ptr, vtable_ptr)
                   8 bytes  +  8 bytes  = 16 bytes
```

### VTable kya hai?

Compiler **compile time** pe har concrete type ke liye ek function pointer table banata hai:

```
Dog ki VTable:                          Cat ki VTable:
┌────────────────────────────────┐     ┌────────────────────────────────┐
│ speak → Dog::speak ka address  │     │ speak → Cat::speak ka address  │
│ drop  → Dog::drop  ka address  │     │ drop  → Cat::drop  ka address  │
│ size  → sizeof(Dog)            │     │ size  → sizeof(Cat)            │
│ align → alignof(Dog)           │     │ align → alignof(Cat)           │
└────────────────────────────────┘     └────────────────────────────────┘
```

### Memory Layout

```
Vec (stack):
┌──────────────────────────────────────┐
│ [0]: data_ptr ──────────> Heap: Dog data
│      vtable_ptr ────────> Dog_VTable
├──────────────────────────────────────┤
│ [1]: data_ptr ──────────> Heap: Cat data
│      vtable_ptr ────────> Cat_VTable
└──────────────────────────────────────┘
```

### Method Call Kaise Hota Hai?

```rust
animal.speak()
```

Internally (pseudocode):

```
1. animal.vtable_ptr pakdo        → Cat_VTable mila
2. vtable mein "speak" dhundho    → function pointer 0xB1
3. function call karo             → Cat::speak(&cat_data)
4. return "Meow!"
```

---

## 4. Static vs Dynamic Dispatch

### Static Dispatch (Generics / impl Trait)

```rust
fn make_sound<T: Animal>(a: &T) {
    a.speak(); // compile time pe pata hai kaunsa speak()
}
```

- Compiler har type ke liye **alag function** banata hai (monomorphization)
- Direct function call → **fast**, inlining possible
- Binary size bada hota hai (har type ka duplicate code)

### Dynamic Dispatch (dyn Trait)

```rust
fn make_sound(a: &dyn Animal) {
    a.speak(); // runtime pe vtable se lookup
}
```

- Sirf **ek function** banta hai
- Indirect call via vtable → **~1-2 ns slower**
- Binary size chhota rehta hai

### Comparison Table

| Feature          | Static (Generics)        | Dynamic (dyn)              |
|------------------|--------------------------|----------------------------|
| Decision time    | Compile time             | Runtime                    |
| Speed            | Fast (direct/inlined)    | Thoda slow (vtable lookup) |
| Mixed types      | Nahi (ek type at a time) | Haan (alag types ek mein)  |
| Binary size      | Bada                     | Chhota                     |
| Flexibility      | Kam                      | Zyada                      |

---

## 5. Rule of Thumb

| Situation                                         | Use                  |
|---------------------------------------------------|----------------------|
| Ek collection mein alag-alag types rakhne hain    | `Box<dyn Trait>`     |
| Performance critical hai, type fixed hai           | Generics / `impl T`  |
| Recursive data structure (linked list, tree)       | `Box<T>`             |
| Large data stack overflow se bachna hai            | `Box<T>`             |
| Function se koi bhi trait implementor return karna | `Box<dyn Trait>`     |

---

## 6. Key Takeaways

1. **Box<T>** = heap allocation, stack pe sirf 8-byte pointer
2. **dyn** = dynamic dispatch, runtime pe method decide hota hai
3. **VTable** = function pointers ki table, har type ke liye alag, compile time pe banti hai
4. **Fat pointer** = `(data_ptr + vtable_ptr)` = 16 bytes
5. **Static dispatch** fast hai lekin mixed types nahi kar sakta
6. **Dynamic dispatch** flexible hai lekin ~1-2 ns overhead per call
