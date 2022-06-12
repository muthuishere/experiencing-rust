## Experiencing Rust - Day 1


Started Writing the first line of code in rust today. I want to do a bit of challenging code and wants to be Test driven. I took a problem of Balanced Parentheses.


### Tests
  Rust has a built in testing framework. It expects you to write tests for your code in the same file as your source. This is totally new to me.But it seems good in a way as we dont have to switch editors to see where the test code is

Since the testing framework is bundled . They does the job but they are not very expressive. This is the same with  assertions as well. 


### Variable Declaration
    let is used to declare a variable. , By default all variables are immutable.


For String Rust has two types of String
- String - a struct which allow you to dynamically update
- &str - a character pointer , which we known from C & immutable



```rust

    let s: String = "hello".into();; 
    
    let s2: &str = "hello";
```

I felt we need to use &str whe
I always wanted to learn systems language, which has functional abstractions well written.
