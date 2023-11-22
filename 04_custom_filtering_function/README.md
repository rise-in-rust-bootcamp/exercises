# Hands-on Task - Building a Custom Filtering Function in Rust

The goal is to implement a beginner-friendly solution that avoids using closures to simplify the understanding of the code.

## Steps

1. Open the main.rs file in a text editor.
2. Define a struct called FilterCondition with a single field of the desired type for filtering.
3. Implement a method called is_match on the FilterCondition struct that takes a reference to an item of the same type as the filter condition and returns a boolean indicating whether the item matches the condition.
4. Define a function called custom_filter that takes a collection (e.g., a vector) and a reference to a FilterCondition object as arguments. The function should iterate over the elements in the collection and return a new collection containing only the elements that match the filter condition.
5. In the main function, create a collection (e.g., a vector) with some elements and initialize a FilterCondition object with the desired value.
6. Call the custom_filter function with the collection and the FilterCondition object, storing the result in a new variable.
7. Print the filtered result to the console.
8. Compile and run the program to test its functionality.

## Checklist

1. Create a new Rust project and open the main.rs file.
2. Define the FilterCondition struct with the desired type for filtering.
3. Implement the is_match method on the FilterCondition struct.
4. Define the custom_filter function to filter elements based on the condition.
5. Create a collection and a FilterCondition object in the main function.
6. Call the custom_filter function and store the result.
7. Print the filtered result to the console.
8. Compile and run the program to test its functionality.

## How to Run

Run the program:

```sh
cargo run
```

Run unit tests

```sh
cargo test
```
