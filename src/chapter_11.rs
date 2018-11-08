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



//Let's implement a Trait
/*
/// A trait for characters, items, and scenery -
/// anything in the game world that is visible on Screen.
trait Visible {
    ///Render this object on a given canvas.
    fn draw(&self, canvas: & Canvas);

    /// Return true if clicking at (x,y) should
    /// select this object.
    fn hit_test(&self, x: i32, y: i32) -> bool;
}
impl Broom {
    /// Helper function used by Broom:draw() below.
    fn broomstick_range(&self) -> Range<i32> {
        self.y - self.height - 1 .. self.y
    }
}

impl Visible for Broom {
    fn draw(&self, canvas: &mut Canvas) {
        for y in self.y - self.height -1 .. self.y {
            canvas.write_at(self.x, y, '|');
        }
        canvas.write_at(self.x, self.y, 'M');
    }

    fn hit_test(&self, x: i32, y: i32) -> bool {
        self.x == x;
        && self.y - self.height -1 <= y
        && y <= self.y
    }
}
*/
/*
/// A Writer that ignores whatever data you write to it.
pub struct Sink;

use std::io::Result;

impl Write for Sink {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        // Claim to have been successfully written the whole buffer.
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}


/// Define a trait for other people's types
trait IsEmoji {
    fn is_emoji(&self) -> bool;
}

impl IsEmoji for char {
    fn is_emoji(&self) -> bool{
        false //let's assume nothing is an emoji
    }
}

#[test]
fn test_is_emoji() {
    assert_eq!('$'.is_emoji(), false);
}


use std::io::self;
// you can even use a generic impl to add an extension trait to a whole family of types at once
/// Trait for values to which you can send HTML
trait WriteHtml{
    fn write_html(&self, &HtmlDocument) -> io::Result<()>;
}

impl<W: Write> WriteHtml for W {
    fn write_html(&mut self, html: &HtmlDocument) -> io::Result<()> {
       // Do stuff
       Ok(())
    }
}

// Subtraits
///Someone in the game world, either the player or some other thing
trait Creature: Visible {
    fn position(&self) -> (i32, i32);
    fn facing(&self) -> Direction;
    ...
}

// every type that implements the Creature trait also need to implement Visible trait.
impl Visible for Broom {
    ....
}

impl Creature for Broom {
   ....
}



// Staic Methods
// traits can include static methods and constuctors unlike most interfaces in other languages
trait StringSet {
    fn new() -> Self;
        where Self: Sized;  //This allows static methods saying trait objects don't need to support this particular method

    fn from_slice(strings: &[str]) -> Self;
        where Self: Sized;

    fn contains(&self, string: &str) -> bool;

    fn add(&mut self, string: &str);
}

//create sets of two hypothetical types that impl StringSet;
let set1 = SortedStringSet::new();
let set2 = HashedStringSet::new();

// In generic code, it's the same, except the type is often a type variable
/// Return the set of words in 'document' that aren't in the 'wordlist'.
fn unkonwn_words<S: StringSet>(document: &Vec<String>, wordlist: &S) -> S {
    let mut unknowns = S::new();
    for word in document {
        if !wordlist .contains(word) {
            unknowns.add(word);
        }
    }
    unknowns
}
*/

//Implement a function that does generic dot product with traits
use std::ops::{Add, Mul};

/// Generic dot product calculator
fn dot<N>(v1: &[N], v2: &[N]) -> N
    where N: Add<Output=N> + Mul<Output=N> + Default + Copy
{
    let mut total = N::default(); //All numerical types default will equal 0
    for i in 0 .. v1.len() {
        total = total + v1[i] * v2[i];
    }
    total
}

#[test]
fn test_dot() {
    assert_eq!(dot(&[1,2,3,4], &[1,1,1,1]), 10);
    assert_eq!(dot(&[53.0, 7.0], &[1.0, 5.0]), 88.0);
}

// Test macbook commit
