//! A simple macro allowing for clean do-while loops in Rust.
//!
//! Basic usage:
//! ```rust
//! use do_while::do_while;
//!
//! let mut x = 0;
//! do_while! {
//!     do {
//!         x += 1;
//!     } while x < 10;
//! }
//! assert_eq!(x, 10);
//! ```
//!
//! For more advanced details and usage, see the macro-level documentation for [do_while].

/// A macro allowing for clean do-while loops.
///
/// The basic syntax is:
/// ```rust
/// use do_while::do_while;
/// # let condition = false;
/// # fn do_stuff() {}
///
/// do_while! {
///     do {
///         do_stuff();
///     } while condition;
/// }
/// ```
///
/// This expands to:
/// ```rust
/// # let condition = false;
/// # fn do_stuff() {}
/// while {
///     do_stuff();
///     condition
/// } {}
/// ```
/// The body of the do-while loop is placed inside the condition expression of the while loop,
/// meaning it runs before the condition is evaluated, and the actual body of the while loop is left
/// empty. The `do_while` macro allows for this to be expressed in a cleaner and more obvious fashion.
///
/// 'Do-while-do' loops, with code in _both_ the condition and the body of the expanded while loop,
/// are also possible.
/// ```rust
/// use do_while::do_while;
/// # let condition = false;
/// # fn do_stuff() {}
/// # fn do_more_stuff() {}
///
/// do_while! {
///     do {
///         do_stuff();
///     } while condition, do {
///         do_more_stuff();
///     }
/// }
/// ```
/// This expands to:
/// ```rust
/// # let condition = false;
/// # fn do_stuff() {}
/// # fn do_more_stuff() {}
/// while {
///     do_stuff();
///     condition
/// } {
///     do_more_stuff();
/// }
/// ```
/// Multiple loops within the same macro invocation are also possible.
///
/// ## Examples
///
/// Simple do-while loop:
/// ```rust
/// use do_while::do_while;
///
/// let mut x = 0;
/// do_while! {
///     do {
///         x += 1;
///     } while x < 10;
/// }
/// assert_eq!(x, 10);
/// ```
///
/// Custom list formatting with a do-while-do loop:
/// ```rust
/// use do_while::do_while;
///
/// let items = vec![1, 2, 3, 4];
/// let mut string = String::new();
///
/// let mut index: usize = 0;
/// do_while! {
///     do {
///         string.push_str(&items[index].to_string());
///         index += 1;
///     } while index < items.len(), do {
///         string.push_str(", ");
///     }
/// }
///
/// assert_eq!(string, "1, 2, 3, 4".to_string());
/// ```
///
/// Multiple loops at once:
/// Multiple do-while and do-while-do loops can be mixed and matched in the same macro invocation:
/// ```rust
/// use do_while::do_while;
///
/// let mut x = 0;
/// let mut y = 0;
///
/// let list = vec![5, 6, 7, 8];
/// let mut string = String::new();
/// let mut index: usize = 0;
///
/// do_while! {
///     do {
///         x += 1;
///     } while x < 10;
///
///     do {
///         y -= 1;
///     } while y > -20;
///
///     do {
///         string.push_str(&list[index].to_string());
///         index += 1;
///     } while index < list.len(), do {
///         string.push_str(", ");
///     }
/// }
///
/// assert_eq!(x, 10);
/// assert_eq!(y, -20);
/// assert_eq!(string, "5, 6, 7, 8".to_string());
/// ```
#[macro_export]
macro_rules! do_while {
    () => {};
    (do $body:block while $cond:expr; $( $others:tt )*) => {
            while { $body; $cond } {}
            do_while! { $( $others )* }
    };
    (do $body_before:block while $cond:expr, do $body_after:block $( $others:tt )*) => {
            while { $body_before; $cond } { $body_after; }
            do_while! { $( $others )* }
    };
}

#[cfg(test)]
mod tests {
    use crate::do_while;

    #[test]
    fn test_do_while() {
        // Simple do-while loop
        let mut x = 0;
        do_while! {
            do {
                x += 1;
            } while x < 10;
        }
        assert_eq!(x, 10);

        // Do-while-do loop
        let list = vec![1, 2, 3, 4];
        let mut string = String::new();
        let mut index: usize = 0;
        do_while! {
            do {
                string.push_str(&list[index].to_string());
                index += 1;
            } while index < list.len(), do {
                string.push_str(", ");
            }
        }
        assert_eq!(string, "1, 2, 3, 4".to_string());

        // Multiple do-while loops
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
    }
}