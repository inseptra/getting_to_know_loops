

fn main() {
    let mut x_pos = 3.0; // no point in using a decimal as were only using Whole numbers
    loop {
        println!("Updating yours x position {}", x_pos);
        x_pos -= 1.0; // Shorthand for: x_pos = x_pos - 1.0;
        // About comparing floats:
        // You usually *don’t* want to compare floating-point numbers with == (in my opinion)
        // unless you know the exact values.
        // Some values (like 3.0, 2.0, 1.0, 0.0) are perfectly representable in
        // binary floating-point, so subtracting 1.0 each loop is safe.
        // But many other float operations produce tiny rounding errors.
        // That means a number you expect to be 0.0 might end up as
        // 0.0000000000000001 instead, and == would fail.
        // When exact equality matters, it’s safer to:
        //  use integers for counting/steps, or
        // check “closeness” instead of exact equality.

        // What I would do:
        // Avoid using floats when you're doing precise counting or equality checks.
        // For something like x_pos, an integer type (i32, i64, etc.) is more reliable.
        // Do your stepping/subtraction using the integer. That way the values are exact
        // and comparisons like == work perfectly.
        // Only convert to a float when you actually need floating-point math or want
        // to display the value. Casting at the end keeps everything predictable.


        if x_pos == 0.0 { // note that you must be able to take you value and subtract it to get 0
            break; // so we set a condition when x = 0 it ends the loops and print to the term
        }
    }
    println!(" you have reset your x pos to zero : {} ", x_pos); // this is are value that we wanted
}
