# fluent-asserter

<p align="center">
  <img width=300 src="https://cdn.jsdelivr.net/gh/mirind4/dmoka-cdn/images/fluent-asserter-logo-white-bg.png" />
</p>


## Introduction

A library to write test assertions with a fluent interface. Writing clean tests is as important as writing clean production code.
This library contains test asserters for many kind of Rust types to produce clean assertions in our automated tests.
It also helps to enhance the Test-Driven Development (TDD) experience, resulting in clean, ergonomic and maintainable tests.

## Usage

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
fluent-asserter = "0.1.0"
```

Then import the asserters via the prelude

```rust
use fluent_asserter::prelude::*;
```

Now you should be able to write test assertions with a fluent syntax in your tests. In the next sections, I show you how.

## String and string slice assertions

You can write string assertions for both String and str slices

```rust
#[test]
fn string_assertions() {
    assert_that!("Life tastes great!").is_equal_to("Life tastes great!");
    assert_that!("Life tastes great!").contains("great");
    assert_that!("Life tastes great!").starts_with("Life");
    assert_that!("Life tastes great!").ends_with("!");
    assert_that!("Life tastes great!").is_not_empty();
    assert_that!("Life tastes great!").has_length(18);
    assert_that!("Life tastes great!").contains_any(&["Life", "awesome"]);
    assert_that!("Life tastes great!").contains_all(&["Life", "tastes", "great!"]);
}
 ```

## Number assertions

 ```rust
#[test]
fn number_assertions() {
    assert_that!(21).is_equal_to(21);
    assert_that!(21).is_smaller_than(22);
    assert_that!(21).is_smaller_than_or_equal_to(21);
    assert_that!(21).is_greater_than(20);
    assert_that!(21).is_in_range(21,31);
    assert_that!(21).is_not_in_range(10,20);
    assert_that!(3.14159).is_approx_equal(3.142, 0.001);
}
 ```

 ## Boolean assertions

 ```rust
#[test]
fn boolean_assertions() {
    assert_that!(true).is_true();
    assert_that!(false).is_false();
}
 ```

 ## Panic assertions

 ```rust
 #[test]
 fn panic_assertions() {
     assert_that_code!(|| panic!("An error occurred!"))
         .panics()
         .with_message("An error occurred!");

     assert_that_code!(|| println!("Life tastes great!")).does_not_panic();
 }
 ```

 ## Iterator assertions

 ```rust
 #[test]
 fn iterator_assertions() {
     assert_that!(vec!["tasty", "delicious", "lovely"]).is_equal_to(vec!["tasty", "delicious", "lovely"]);
     assert_that!(vec!["tasty", "delicious", "lovely"]).contains("delicious");
     assert_that!(vec!["tasty", "delicious", "lovely"]).contains_all(&["tasty", "delicious", "lovely"]);
     assert_that!(vec!["tasty", "delicious", "lovely"]).has_count(3);
     assert_that!(vec!["tasty", "delicious", "lovely"]).does_not_contain_any(&["awesome", "amazing"]);
     assert_that!(vec!["tasty", "delicious", "lovely"]).is_not_empty();
 }
 ```

 ## Iterator assertion for structs

 ```rust
 #[derive(Clone)]
 struct Person {
     name: String,
     age: i32,
 }

 #[test]
 fn iterator_assertion_for_struct() {
     let people: Vec<Person> = vec![
         Person {
             name: String::from("Daniel"),
             age: 32,
         },
         Person {
             name: String::from("Jimmy"),
             age: 45,
         },
     ];

     assert_that!(people).satisfies_respectively(with_asserters!(
             |person1: &Person| {
                 assert_that!(&person1.name).is_equal_to(&String::from("Daniel"));
                 assert_that!(&person1.age).is_equal_to(&32);
             },
             |person2: &Person| {
                 assert_that!(&person2.name).is_equal_to(&String::from("Jimmy"));
                 assert_that!(&person2.age).is_equal_to(&45);
             }
         ));
 }
 ```

## Hashmap assertions

```rust
#[test]
fn hashmap_assertions() {
    let mut hash_map = HashMap::<String, String>::new();
    assert_that!(&hash_map).is_empty();

    hash_map.insert(String::from("key"), String::from("value"));
    assert_that!(&hash_map).has_length(1);
    assert_that!(&hash_map).is_not_empty();
    assert_that!(&hash_map).contains_key(&String::from("key"));
    assert_that!(&hash_map).does_not_contain_key(String::from("key2"));
}
```


## Option assertions

```rust
#[test]
fn option_assertions() {
    let option = Option::Some("Winner!");
    assert_that!(option).is_some();
    assert_that!(option).is_some_with_value("Winner!");

    let none = Option::<i32>::None;
    assert_that!(none).is_none();
}
```

## Result assertions

```rust
#[test]
pub fn result_assertions() {
    let result : Result<i32,i32> = Ok(3);
    assert_that!(&result).is_ok();
    assert_that!(&result).is_ok_with_value(3);

    let error : Result<i32,String> = Err(String::from("error message"));
    assert_that!(&error).is_error();
    assert_that!(&error).is_error_with_value(String::from("error message"));
}
```

## Clear and concise error messages
 
In case of a failing assertion, the error message is clear and on the point, containing all the information relating to the domain subject.

```rust
#[test]
fn test() {
   let string_variable = String::from("Hello Rust!");

   assert_that!(string_variable).is_equal_to(String::from("Hello C#!"));
}
```

This test produces the following assertion error message:

```doc
Expected string_variable to be "Hello C#!", but was "Hello Rust!".
```
