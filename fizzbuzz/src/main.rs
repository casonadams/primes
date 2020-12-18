/* fizz buzz
 *
 *
 * every 3rd word should be fizz
 * every 5th word should be buzz
 * every 15th word should be fizzbuzz
 */

fn main() {
    fizzbuzz(100);
}

fn fizzbuzz(n: i16) {
    for i in 0..=n {
        if i % 15 == 0 {
            println!("{}\tfizzbuzz", i);
        } else if i % 3 == 0 {
            println!("{}\tfizz", i);
        } else if i % 5 == 0 {
            println!("{}\tbuzz", i);
        } else {
            println!("{}", i);
        }
    }
}
