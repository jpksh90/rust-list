use std::cmp;
use std::fmt;
use std::ops::Add;

#[derive(Debug, Clone)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

impl List {
    fn new() -> Self {
        Self::Nil
    }

    fn append(self, x: i32) -> List {
        match self {
            Self::Cons(t, l1) => {
                let t1 = l1.append(x);
                Cons(t, Box::new(t1))
            }
            Self::Nil => {
                Cons(x, Box::new(Nil))
            }
        }
    }

    fn concat(self, l2: List) -> List {
        match self {
            Self::Nil => l2,
            Self::Cons(t, t1) => {
                let new_list = t1.concat(l2);
                Self::Cons(t, Box::new(new_list))
            }
        }
    }

    fn push(self, x: i32) -> List {
        match self {
            Self::Cons(_, _) => {
                return Self::Cons(x, Box::new(self));
            }
            Self::Nil => Self::Cons(x, Box::new(Nil))
        }
    }

    fn find(self, x: i32) -> bool {
        match self {
            Self::Cons(v, l) => if v == x {true} else {l.find(x)},
            Self::Nil => false
        }
    }

    fn len(self) -> isize {
        match self {
            Self::Cons(_, l) => 1 + l.len(),
            Self::Nil => 0
        }
    }


    fn _as_mut_vec(self, v: &mut Vec<i32>) {
        match self {
            Self::Cons(h, t) =>  { 
                v.push(h);
                t._as_mut_vec(v);
            }
            Self::Nil => {
                return;
            }
        }
    }

    fn as_mut_vec(self) -> Vec<i32> {
        let mut v = Vec::new();
        self._as_mut_vec(&mut v);
        v
    }

    fn split(self, n: usize) -> (List, List) {
        let t = self.as_mut_vec();
        let (l,r) = t.split_at(n);
        (List::from(l.to_vec()), List::from(r.to_vec()))
    }

}


impl Add for List {
    type Output = List;

    fn add(self, rhs: Self) -> Self::Output {
        let l = self.concat(rhs);
        l
    }
}

impl std::convert::From<Vec<i32>> for List {
    fn from(vs: Vec<i32>) -> Self {
        let mut l = List::new();
        for v in vs {
            l = l.append(v);
        }
        l
    }
}

impl cmp::PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Cons(l0, l1), Self::Cons(r0, r1)) => l0 == r0 && l1 == r1,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}



impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cons(x1, t1) => { let rest = t1.to_string(); write!(f, "{x1},{rest}") }
            Nil => write!(f, "NIL" )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_nil() {
        let l1 = List::new().append(5);
        let l2 = List::from(vec![5]);
        assert_eq!(l1, l2);
    }

    #[test]
    fn append_not_nil() {
        let l1 = List::from(vec![1,2,3]);
        let l2 = List::from(vec![1,2,3,4]);
        let l3 = l1.append(4);
        assert_eq!(l2,l3);
    }

    #[test]
    fn concat_nil() {
        let l1 = List::new();
        let l2  = List::from(vec![1,2,3]);
        assert_eq!(l1 + l2, List::from(vec![1,2,3]))
    }

    #[test]
    fn concat_not_nil() {
        let l1 = List::from(vec![1,2]);
        let l2 = List::from(vec![3]);
        assert_eq!(l1+l2, List::from(vec![1,2,3]));
    }

    #[test]
    fn push_not_nil() {
        let l1 = List::from(vec![1,2]).push(0);
        assert_eq!(l1, List::from(vec![0,1,2]));
    }

    #[test]
    fn push_nil() {
        let l1 = List::new().push(0);
        assert_eq!(l1, List::from(vec![0]));
    }

    #[test]
    fn find_true() {
        let l1 = List::from(vec![1,2,3,4]);
        assert!(l1.find(1));
    }

    #[test]
    fn find_false() {
        let l1 = List::from(vec![1,2,3,4]);
        assert!(!l1.find(9));
    }

    #[test]
    fn len1() {
        assert_eq!(List::from(vec![1,2,3]).len(), 3);
    }

    #[test]
    fn len2() {
        assert_eq!(List::new().len(), 0);
    }

    #[test]
    fn mut_vec_not_nil() {
        let l1 = List::from(vec![1,2,3]);
        assert_eq!(l1.as_mut_vec(), vec![1,2,3]);
    }

    #[test]
    fn mut_vec_nil() {
        let l1 = List::new();
        assert_eq!(l1.as_mut_vec(), Vec::new());
    }
}

fn main() {
    println!("use 'cargo test' to check");
}
// fn main() {
//     let list1 = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
//     let list2 = Cons(4, Box::new(Nil));
//     let list3 = list2.append(5);
//     println!("{}", list3);
//     // let list2 = Cons(4, Box::new(Cons(5, Box::new(Cons(6, Box::new(Nil))))));
//     // println!("{:?}", list1);
//     // println!("{:?}", list2);

//     // let l3 = list1.concat(list2);
//     // println!("{}", l3);
//     // let l4 = l3.push_back(5);
//     // println!("{}", l4);
//     // // print_list(list);


// }
