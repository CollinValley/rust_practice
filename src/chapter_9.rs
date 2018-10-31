
/// A first in, first-out queue of characters
pub struct Queue {
    older: Vec<char>, //older elements, eldest last.
    younger: Vec<char> // younger elements, youngest last.
}

impl Queue {

    ///Constructor
    pub fn new() -> Queue {
        Queue {older: Vec::new(), younger: Vec::new() }
    }

    /// Push a character onto the back of a queue
    pub fn push(&mut self, c: char) {
        self.younger.push(c);
    }

    /// Pop a character off the front of a queue. Return `Some(c)` if there
    /// was a character to pop, or `None` if the queue was empty
    pub fn pop(&mut self) -> Option<char> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }

            // Bring elements in younger queue over to older, and put them in
            // the promised order
            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }

        // Now older is guaranteed to have something. Vec's pop method
        // already returns an Option, so we're set.
        self.older.pop()
    }

    /// Returns true if the queue is empty
    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }

    /// Splits queue into two queues
    pub fn split(self) -> (Vec<char>, Vec<char>) {
        (self.older, self.younger)
    }
}

#[test]
fn test_queue() {
    let mut q = Queue { older: Vec::new(), younger: Vec::new() };

    q.push('0');
    q.push('1');
    assert_eq!(q.pop(), Some('0'));

    q.push('&');
    assert_eq!(q.pop(), Some('1'));
    assert_eq!(q.pop(), Some('&'));
    assert_eq!(q.pop(), None);

    //Test is_empty
    q.push('a');
    assert!(!q.is_empty());
    assert_eq!(q.pop(), Some('a'));
    assert!(q.is_empty());

    //Test split
    q.push('P');
    q.push('D');
    assert_eq!(q.pop(), Some('P'));
    q.push('X');
    let (older, younger) = q.split();
    // q is now uninitialized,
    assert_eq!(older, vec!['D']);
    assert_eq!(younger, vec!['X']);

    //Test constructor
    let mut q = Queue::new();
    q.push('*');
    assert_eq!(q.pop(), Some('*'));
}

/// A first in, first-out queue of characters
pub struct GQueue<T> {
    older: Vec<T>, //older elements, eldest last.
    younger: Vec<T> // younger elements, youngest last.
}

impl<T> GQueue<T> {

    ///Constructor
    pub fn new() -> Self {
        GQueue {older: Vec::new(), younger: Vec::new() }
    }

    /// Push a character onto the back of a queue
    pub fn push(&mut self, t: T) {
        self.younger.push(t);
    }

    /// Pop a thing off the front of a queue. Return `Some(t)` if there
    /// was a thing to pop, or `None` if the queue was empty
    pub fn pop(&mut self) -> Option<T> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }

            // Bring elements in younger queue over to older, and put them in
            // the promised order
            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }

        // Now older is guaranteed to have something. Vec's pop method
        // already returns an Option, so we're set.
        self.older.pop()
    }

    /// Returns true if the queue is empty
    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }

    /// Splits queue into two queues
    pub fn split(self) -> (Vec<T>, Vec<T>) {
        (self.older, self.younger)
    }
}


#[test]
fn test_gqueue() {
    let mut c = GQueue::<char>::new(); //Use turbofish notation to specify queue type

    let mut q = GQueue::new(); //Most of the time you can use the compiler to determine type though
    let mut r = GQueue::new();

    q.push("CAD"); // apparently a GQueue<&'static str>
    r.push(0.74);  // a GQueue<f64>

    q.push("BTC");
    r.push(2737.7);

    assert!(c.is_empty());
    assert!(!q.is_empty());
    assert!(!r.is_empty());

    c.push('C');
}


struct Extrema<'elt> {
    greatest: &'elt i32,
    least: &'elt i32
}

fn find_extrema<'s>(slice: &'s [i32]) -> Extrema<'s> {
    let mut greatest = &slice[0];
    let mut least = &slice[0];

    for i in 1..slice.len() {
        if slice[i] < *least { least = &slice[i]; }
        if slice[i] > *greatest { greatest = &slice[i]; }
    }
    Extrema {greatest, least }
}

#[test]
fn test_find_extrema() {
    let a = [0, -3, 15, 48];
    let e = find_extrema(&a);
    assert_eq!(*e.least, -3);
    assert_eq!(*e.greatest, 48);
}


// Deriving common traits for stuct types

/// A struct with traits already predefined by the compiler.
///
/// Allows us to use the == and != and = operators just like we
/// expect. Note these are automatically public for obvious reasons
#[derive(Copy, Clone, Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64
}

// Interior mutability

//Nothing really to add here, remember Cell and RefCell for opening files and storing a smart
//reference to the file. Easier to modify something like a log file which normally should not
// be modifiable
