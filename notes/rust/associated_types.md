# Associated Types vs Generics

---

## Generic Trait — multiple implementations possible

```rust
trait Summary<T> {
    fn summarize(&self) -> T;
}

// Ek struct ke liye MULTIPLE implementations — confusing
impl Summary<i32> for Numbers { ... }
impl Summary<String> for Numbers { ... }

// Caller ko specify karna padta hai kaunsa chahiye
let result: i32 = n.summarize();
```

---

## Associated Type — ek type per implementation, fixed

```rust
trait Summary {
    type Output;                          // placeholder — implementor decide karega
    fn summarize(&self) -> Self::Output;
}

impl Summary for Numbers {
    type Output = i32;                    // Numbers ka Output HAMESHA i32
    fn summarize(&self) -> i32 {
        self.data.iter().sum()
    }
}

impl Summary for Words {
    type Output = String;
    fn summarize(&self) -> String {
        self.data.join(" ")
    }
}
```

---

## iter() kyun chahiye sum() ke liye?

```
Vec<i32>  → collection hai, methods limited hain
    │
    .iter()
    │
Iterator<Item = &i32>  → .sum(), .map(), .filter() sab yahan milte hain
```

`sum()` sirf iterator pe kaam karta hai, Vec pe directly nahi.
`join()` slice method hai toh Vec pe seedha lagta hai.

---

## Comparison

| | Associated Type | Generic |
|---|---|---|
| Implementations per type | **Ek** | Multiple |
| Caller ko specify karna | Nahi | Haan (`Summary<i32>`) |
| Use kab | Output type fixed hai | Multiple conversions possible |
| Example | `Iterator { type Item }` | `From<T>` |
