#![allow(dead_code)]
#![allow(unused_imports)]

use std::cmp::Ordering;

/// Basic compare using standard library enum
fn compare(n: i32, m:i32) -> Ordering {
    if n < m {
        Ordering::Less
    } else if n > m {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

// Declare our own enum
enum Pet {
    Orca,
    Giraffe,
    Dog,
    Cat
}

// use our own
use self::Pet::*;

// In this enum, we provide the numbers instead of the standard
enum HttpStatus {
    Ok = 200,
    NotModified = 304,
    NotFound = 404
}
/// Implements converting from integer to enum
///
/// This works this way but not explicitly
fn http_status_from_u32(n: u32) -> Option<HttpStatus> {
    match n {
        200 => Some(HttpStatus::Ok),
        304 => Some(HttpStatus::NotModified),
        404 => Some(HttpStatus::NotFound),
        _ => None
    }
}

#[test]
fn test_generic_enums() {
    // Rust stores the numbers in the smallest numbers that will fit, generally 1byte
    use std::mem::size_of;
    assert_eq!(size_of::<Ordering>(), 1);
    assert_eq!(size_of::<HttpStatus>(), 2);
    assert_eq!(size_of::<Pet>(), 1);

    // We can cast the enums as a C style int if needed
    assert_eq!(HttpStatus::Ok as i32, 200);

    // Test conversion
    let status = http_status_from_u32(404);
    assert_eq!(status.unwrap() as u32, 404);
}

// We can let the compiler implement the basics for us
#[derive(Copy, Clone, Debug, PartialEq)]
enum TimeUnit {
    Seconds, Minutes, Hours, Days, Months, Years
}

// and implement functions just like we expect
impl TimeUnit {
    /// Return the plural noun for this time unit
    fn plural(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years"
        }
    }

    /// Return the singular noun for this time unit.
    fn singular(self) -> &'static str {
        self.plural().trim_right_matches('s')
    }
}

#[test]
fn test_timeunit() {
    let plural = TimeUnit::Seconds.plural();
    assert_eq!(plural, "seconds");
    let singular = TimeUnit::Months.singular();
    assert_eq!(singular, "month");
}

// Timestamps with data

/// A timestamp that has been delibeartly rounded off, so our program
/// says "6 months ago" instead of "Feb 9th, 2016 at 9:42pm"
#[derive(Copy, Clone, Debug, PartialEq)]
enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32)
}

fn rough_time_to_english(rt: RoughTime) -> String {
    match rt {
        RoughTime::InThePast(unit, 1) =>
            format!("a {} ago", unit.singular()),
        RoughTime::InThePast(units, count) =>
            format!("{} {} ago", count, units.plural()),
        RoughTime::JustNow =>
            format!("just now"),
        RoughTime::InTheFuture(unit, 1) =>
            format!("a {} from now", unit.singular()),
        RoughTime::InTheFuture(units, count) =>
            format!("{} {} from now", count, units.plural())
    }
}

#[test]
fn test_rough_time_to_english() {
    let one_day_ago = RoughTime::InThePast(TimeUnit::Days, 1);
    let two_years_ago = RoughTime::InThePast(TimeUnit::Years, 2);
    let right_now = RoughTime::JustNow;
    let one_second_away = RoughTime::InTheFuture(TimeUnit::Seconds, 1);
    let two_months_away = RoughTime::InTheFuture(TimeUnit::Months, 2);

    assert_eq!("a day ago",
               rough_time_to_english(one_day_ago));
    assert_eq!("2 years ago",
               rough_time_to_english(two_years_ago));
    assert_eq!("just now",
               rough_time_to_english(right_now));
    assert_eq!("a second from now",
               rough_time_to_english(one_second_away));
    assert_eq!("2 months from now",
               rough_time_to_english(two_months_away));
}

/// A rich enum to encapsulate JSON values
///
/// Notice the Object or Array types, which are cool.
/*
enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>)
}
*/


/// Data Structure for a Binary Tree
// An ordered collection of `T`s
enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>)
}

//A part of a BinaryTree.
struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>
}

#[test]
fn test_build_basic_tree() {
    use self::BinaryTree::*;

    let jupiter_tree = NonEmpty(Box::new(TreeNode {
        element: "Jupiter",
        left: Empty,
        right: Empty
    }));

    let mercury_tree = NonEmpty(Box::new(TreeNode {
        element: "Mercury",
        left: Empty,
        right: Empty
    }));

    let mars_tree = NonEmpty(Box::new(TreeNode {
        element: "Mars",
        left: jupiter_tree,
        right: mercury_tree
    }));

    let uranus_tree = NonEmpty(Box::new(TreeNode {
        element: "Uranus",
        left: Empty,
        right: Empty
    }));

    let _tree = NonEmpty(Box::new(TreeNode {
        element: "Saturn",
        left: mars_tree,
        right: uranus_tree
    }));
}

