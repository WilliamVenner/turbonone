# turbonone (no_std)

Tiny macro for calling functions with `Option<T>` arguments.

## Example

```rust
fn my_function<T>(arg: Option<T>) {
    ...
}

my_function(None); // cannot infer type for type parameter `T` declared on the associated function `my_function`
my_function(turbonone!()); // Works!
my_function(Some(...)); // Works!
```
