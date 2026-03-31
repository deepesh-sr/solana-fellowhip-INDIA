# Processing Pipeline with Trait Objects

---

## Concept

`Box<dyn Trait>` se dynamic pipeline bana sakte ho — har step alag type, ek Vec mein.

```rust
trait Step {
    fn process(&self, data: &str) -> String;
}

struct Uppercase;
struct AddExclaim;
struct Wrap;

impl Step for Uppercase {
    fn process(&self, data: &str) -> String { data.to_uppercase() }
}
impl Step for AddExclaim {
    fn process(&self, data: &str) -> String { format!("{}!", data) }
}
impl Step for Wrap {
    fn process(&self, data: &str) -> String { format!("[{}]", data) }
}
```

---

## run_pipeline — fold se chain karo

```rust
fn run_pipeline(input: &str, steps: &[Box<dyn Step>]) -> String {
    steps.iter().fold(input.to_string(), |acc, s| s.process(&acc))
}
```

### fold kaise kaam karta hai

```
initial acc = "hello"

Step 1 (Uppercase):   acc = Uppercase.process("hello")   = "HELLO"
Step 2 (AddExclaim):  acc = AddExclaim.process("HELLO")  = "HELLO!"
Step 3 (Wrap):        acc = Wrap.process("HELLO!")        = "[HELLO!]"

return "[HELLO!]"
```

`fold` = starting value lo, har item pe function lagao, result ko next iteration mein pass karo.

---

## &[Box<dyn Step>] kya hai?

```rust
fn run_pipeline(input: &str, steps: &[Box<dyn Step>]) -> String
//                           ^^^^^^^^^^^^^^^^^^^^
//                           slice reference — Vec ka borrow
```

`&[T]` = slice — Vec ya array ka reference. Ownership nahi leta, sirf read karta hai.

```rust
let steps: Vec<Box<dyn Step>> = vec![...];
run_pipeline("hello", &steps);   // &Vec auto-coerces to &[...]
```

---

## Runtime Flexibility

Steps runtime pe decide ho sakte hain:

```rust
let mut steps: Vec<Box<dyn Step>> = vec![];

if config.uppercase {
    steps.push(Box::new(Uppercase));
}
if config.exclaim {
    steps.push(Box::new(AddExclaim));
}
// Dynamic! Compile time pe pata nahi kitne ya kaunse steps honge
```

Yahi trait objects ka real power hai.
