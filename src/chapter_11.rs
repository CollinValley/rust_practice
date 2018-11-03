//traits and generics

use std::io::Write;

/// A hello function that uses traits
fn say_hello(out: &mut Write) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}

/*
Note the above will work on any value that implements the Write trait

For example
use std::fs::File;
let mut local_file = File::create("hello.txt");
say_hello(&mut local_file)?; //works

let mut bytes = vec![];
say_hello(&mut bytes)?; //also works
assert_eq!(bytes, b"hello world\n");
*/

#[test]
fn test_say_hello() {
    let mut bytes = vec![];
    say_hello(&mut bytes).unwrap(); //also works
    assert_eq!(bytes, b"hello world\n");
}

/// A comparasion that uses generics
fn min<T: Ord>(value1: T, value2: T) -> T {
    if value1 <= value2 {
        value1
    } else {
        value2
    }
}

#[test]
fn test_min() {
    let val1 = 10;
    let val2 = 20;
    let val3 = min(val1,val2);
    assert_eq!(val3, 10);
}


//get a reference to a writer function
// This is how we get a trait object
/*
let mut buf: Vec<u8> = vec![];
let writer: &mut Write = &mut buf;
 */

// You create trait objects easily by creating boxes of traits, since those have constant size

//Traits are evaluated at run time, generics are turned into typed functions at compile time.  Trade size of code for speed and optimizations


/*
//Let's implement a Trait

/// A trait for characters, items, and scenery -
/// anything in the game world that is visible on Screen.
trait Visible {
    ///Render this object on a given canvas.
    fn draw(&self, canvas: & Canvas);

    /// Return true if clicking at (x,y) should
    /// select this object.
    fn hit_test(&self, x: i32, y: i32) -> bool;
}

impl Visible for 
*/
