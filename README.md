# do-while
A simple Rust macro for wriing clean 'do-while' loops.

## Examples
A standard do-while loop:
```rust
let mut x = 0;
do_while! {
    do {
        x += 1;
    } while x < 10;
}
assert_eq!(x, 10);
```

'Do-while-do' loops are also supported, running a block of code before the condition is evaluated
and another block after the condition is evaluated. This is useful for things like formatting lists:
```rust
let items = vec![1, 2, 3, 4];
let mut string = String::new();

let mut index: usize = 0;
do_while! {
    do {
        string.push(items[index].to_string);
        index += 1;
    } while index < items.len(), do {
        string.push(", ");
    }
}

assert_eq!(string, "1, 2, 3, 4".to_string());
```

Multiple do-while and do-while-do loops can be mixed and matched in the same macro invocation:
```rust
let mut x = 0;
let mut y = 0;

let list = vec![5, 6, 7, 8];
let mut string = String::new();
let mut index: usize = 0;

do_while! {
    do {
        x += 1;
    } while x < 10;

    do {
        y -= 1;
    } while y > -20;

    do {
        string.push_str(&list[index].to_string());
        index += 1;
    } while index < list.len(), do {
        string.push_str(", ");
    }
}

assert_eq!(x, 10);
assert_eq!(y, -20);
assert_eq!(string, "5, 6, 7, 8".to_string());
```