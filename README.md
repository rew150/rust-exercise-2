# rust-exercise-2

#### 1.อธิบายความแตกต่างของ str and String? และถ้าเราต้องการ access Slice ที่ชี้ไปยัง string จะต้องทำยังไง
- `String` is basically a pointer to string of bytes allocated in the heap. Its size can be known at compile time (size of that pointer + little overhead). It has a lot of utility functions to use.
- `str` is arbitrary-sized string slice. It is a primitive type.
- `str` is a dynamically sized type (unsized type) means that the size can only be known at runtime, bounded with trait `!Sized`, can only be passed to a type with `!Sized` or `?Sized` e.g. `&str`, `Box<str>`, `Rc<str>`, etc.
- If we want to access slice (`&str`) that's pointing to the string, we need to use `Deref::deref(self: &String) -> &str` (for example `s.deref()`)
- Or we can slice it like `&s[0..=4]`

#### 1.2 [T] กับ Vec<T> มีความสัมพันธ์กันอย่างไร
- `Vec<T>` is basically a pointer to an array of items in heap. Its size can be known at compile time (size of that pointer + little overhead). It has a lot of utility functions to use.
- `[T]` is arbitrary-sized slice of array.
- `[T]` is a dynamically sized type (unsized type). It can only be used in function with `!Sized` or `?Sized` e.g. `&[T]`, `Box<[T]>`, `Rc<[T]>`, etc.
- We can also slice vector or array like string.

#### 2. Static lifetime หมายความว่าอะไร
- `'static` means that the object live for the entire lifetime of the program

#### 3. เราจะ define explicit lifetime ได้อย่างไร และมี ประโยชน์ หรือ scope การใช้งานอย่างไร
- we can define lifetime variable `'a` in the same place where we define type variable `T`.
- `&'a str` means that this reference to string slice must live for at least `'a` (lifetime scope).
- reserved lifetime name can just be added like `&'static str` (it's not a variable).
- if the lifetime is not explicitly defined, sometimes compiler can infer it from context.
- reference and lifetime work together. Lifetime exists to ensure that reference is valid which can be proven at compile time
- if the compiler can't infer lifetime you need to explicitly define it. For example
```rust
// compiler can't infer lifetime of return time
// because it don't know where from l or r to
// take the lifetime parameter.
fn argmax_length<'a>(l: &'a str, r: &'a str) -> &'a str {
    if l.len() > r.len() { l } else { r }
}
```
- If your struct/trait contains referenced data, you need to explicitly define your lifetime
```rust
struct Hello<'a>(&'a str);

trait World<'a> {
    fn say_world(&self, world: &'a str);
}

impl<'a> World<'a> for Hello<'a> {
    fn say_world(&self, world: &'a str) {
        println!("{} {}", self.0, world);
    }
}
```
