fn main() {
    /*
        loop

        An infinite loop that repeats a block of code until explicitly stopped

        The difference between while true {} loop is that "loop" can return values
    */

    // This is an infinite loop. We don't explicitly stopped the loop
    // It will run infinitely, I mean, until your system stops or crashed, or you can stopped the loop manually on terminal by Ctrl + C

    // loop {
    //     print!("A"); // If we don't tell the loop to stop it will run system stop or crash
    // }

    // Let's write another loop that stops when it has printed "A" 99 times and it will not print 50th "A"

    let mut count = 0;
    loop {
        if count == 100 {
            break; // Exit the loop
        }
        if count == 49 {
            println!("I won't print 50th A");
            count += 1;
            continue; // Skip this iteration
        }
        count += 1;
        println!("A => {count}")
    }

    // As we said before, loops can return values

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // Exit the loop and return the value
        }
    };

    println!("The result is {result}"); // 20

    // "break" and "continues" applies to the most-inner loop. If we have multiple nested loops we can label the loops
    // '<loopName>

    let mut count = 0;

    'counting_up: loop {
        println!("Count : {count}");
        let mut remaining = 10;

        loop {
            println!("Remaining : {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count: {count}");
}
