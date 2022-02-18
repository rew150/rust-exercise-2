# rust-exercise-2

#### 1.อธิบายความแตกต่างของ str and String? และถ้าเราต้องการ access Slice ที่ชี้ไปยัง string จะต้องทำยังไง
- `String` is basically a pointer to string of bytes allocated in the heap. Its size can be known at runtime (size of that pointer + little overhead).
- `str` is arbitrary-sized string slice. It is a primitive type.
- `str` is a dynamically sized type (unsized type) means that the size can only be known at runtime, bounded with trait `!Sized`, can only be passed to a type with `!Sized` or `?Sized` e.g. `&str`, `Box<str>`, `Rc<str>`, etc.
- If we want to access slice (`&str`) that's pointing to the string, we need to use `Deref::deref(self: &String) -> &str` (for example `s.deref()`)
