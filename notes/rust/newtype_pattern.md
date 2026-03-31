# Newtype Pattern — Orphan Rule ka Solution

---

## Orphan Rule

Trait implement karne ke liye **ya toh trait tumhara ho, ya type tumhara ho**. Dono foreign nahi ho sakte.

```rust
// ❌ Display bhi std ka, Vec bhi std ka
impl fmt::Display for Vec<i32> { ... }
```

**Kyun?** Agar do alag crates dono `Display for Vec<i32>` implement karein toh conflict — compiler ko nahi pata kaunsa use kare.

---

## Solution: Newtype Wrapper

Apna tuple struct banao jo andar foreign type rakhe:

```rust
struct Numbers(Vec<i32>);   // Numbers TUMHARA type hai

impl fmt::Display for Numbers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = self.0          // self.0 = andar ka Vec<i32>
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "{}", result)
    }
}
```

---

## self aur self.0 kya hai?

```rust
let cs = CommaSeparated(vec![1, 2, 3]);
format!("{}", cs);
//            ^^ ye internally fmt(&cs, ...) mein jaata hai
//               self = &cs = &CommaSeparated(vec![1, 2, 3])
//               self.0 = vec![1, 2, 3]
```

Tuple struct mein fields ka naam nahi hota — index se access:

```rust
struct Pair(i32, String);
let p = Pair(42, "hello".to_string());
p.0  // 42
p.1  // "hello"
```

---

## Newtype ke Uses

| Use case | Example |
|---|---|
| Foreign trait on foreign type | `impl Display for Vec<i32>` → wrap in newtype |
| Type safety | `Meters(f64)` vs `Kilometers(f64)` — galti se mix nahi hoga |
| Semantic meaning | `UserId(u64)` vs plain `u64` — code readable |
| Hide implementation | Limited API expose, andar ka type hidden |

Zero runtime cost — compiler wrapper optimize away kar deta hai.
