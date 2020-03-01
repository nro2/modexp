Nick Robinson
CS 410P Rust Programming Winter 2020
1/12/2020
HW 1

Starting this assignment, I took the GCD example straight from the book and modified it to work with `modexp()`. It was helpful as a guide/model
and made my first adventure into Rust a little easier.  I used the method for taking in command line arguments that is mentioned in the book,
which was helpful because it panics if the inputs are negative, which was helpful for testing.  I needed to slightly modify parts of it,
like the GCD took multiple arguments while `modexp()` was limited to exactly 3, so I took out the for loop and tested for the correct input.

The `modexp()` function itself was pretty straight forward.  I mainly just translated the pseudocode prrovided and added some asserts.  I 
wrote an assert in the function that checked that all of the arguments passed in were smaller than the maximum possible 32-bit number, by 
using the `u64::from(u32::max_value())` suggested by Bart.  

For testing, I wrote a few unit tests to check that the correct answer was given with the input provided in the example. I tested that the right 
output occurs when `x == 0` and when `y == 0`. The program automatically panics when the numbers are negative or there are not enough inputs, so 
I did not need to test that.  I also tested that an input that is too large panics.

I ran clippy and format to check that there were no errors.

The assignment went well.  It did not take too long, and the error messages from Rust are **EXTREMELY** helpful.  I don't know that I would have been
able to figure out some of the errors if the messages from compiling weren't as explicit.  I did have an issue where I was going out of bounds
on my inputs, because I was a little confused about the `vector[0]` being the name, so I used inputs `vector[1], vector[2], vector[3]` which was throwing
index out of bounds errors.  I eventually figured that out and it was the last hurdle before my program functioned correctly.