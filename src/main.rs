#![allow(unused)]
use std::{cell::RefCell, fmt::{Debug, Display, Result}, ops::Deref, rc::{Rc, Weak}};
use List::{Coin, Nil};

// =========== usage of Box<T> =============

#[derive(Debug)]
enum List {
    Coin(i32, Box<List>),
    Nil,
}

#[derive(Debug)]
struct Mybox<T: Debug>(T);

impl <T: Debug> Mybox<T> {
    fn new(t: T)-> Self {
        Mybox(t)
    }
}

impl<T: Debug> Deref for Mybox<T>{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0    
    }    
}

impl<T> Drop for Mybox<T> where T: Debug {
    fn drop(&mut self) {
        println!("{:#?} going to Jannat", self.0);
    }
}

impl <T> Display for Mybox<T> 
where T: Display + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(f, "{}", self.0)
    }
}

fn test_deref_coercion(item: &str) {
    println!("Hello {}", item);
}

// ====== usage of RC<T> ===========

enum Chain {
    Node(usize, Rc<Chain>),
    Nil,
}

impl Drop for Chain {
    fn drop(&mut self) {
        println!("dropping... ");
    }
}

use Chain::{
    Node,
    Nil as Cnil
};

// =========== usages of RefCell ===============
pub trait Messager {
    fn send(&self, msg: &str);    
}

struct Whatsapp;
impl Messager for Whatsapp {
    fn send(&self, msg: &str) {
        println!("{}", msg);
    }
}

struct LimitTracker<'a, T: Messager> {
    messager: &'a T,
    value: usize,
    max: usize,
}

impl <'a, T> LimitTracker<'a, T>
where T: Messager,
{
    fn new(msger: &'a T , max: usize) -> Self {
        LimitTracker {
            messager: msger,
            value: 0,
            max,
        }
    }

    pub fn send_val(&mut self, val: usize) {
        self.value = val;

        let percent = self.value as f64 / self.max as f64;
        println!("percent: {}", percent);
        if percent >= 1.0 {
            self.messager.send("Error: You are over your quota!");
        } else if percent >= 0.9 {
            self.messager.send("Urgent warning: You've used up over 90% of your quota!")
        } else if percent >= 0.75 {
            self.messager.send("Urgent warning: You've used up over 75% of your quota!")
        } else {
            self.messager.send("Try again!");
        }
    }
}

// =================== RefCell & Rc<T> combination =========

use Series::{Block, Nil as Nall};

#[derive(Debug)]
enum Series {
    Block(i32, RefCell<Rc<Series>>),
    Nil,   
}

impl Series {
    fn tail(&self) -> Option<&RefCell<Rc<Series>>> {
        match self {
            Block(_, item) => Some(item),
            Nall => None,
        }
    }    
}
#[derive(Debug)]
struct TreeNode {
    value: i32,
    parent: RefCell<Weak<TreeNode>>,
    children: RefCell<Vec<Rc<TreeNode>>>,
}




fn main() {
    println!("Dynamic Memory allocation with Box \n");

    let b = Coin(15, Box::new(Coin(10, Box::new(Nil))));

    println!("b => {:?}", b);
    println!("===== Hands on the Deref ======= ");
    let my_box = Mybox::new(String::from("Goldy Bhai"));

    test_deref_coercion(&my_box);
    
    drop(my_box);
    
    let my_box = Mybox::new(String::from("Ramu"));

    test_deref_coercion(&my_box);
    println!("==== usage of RC<T> ====");
    let a = Rc::new(Node(5, Rc::new(Node(10, Rc::new(Cnil)))));
    println!("a has reference count {}", Rc::strong_count(&a));

    let b = Node(15, Rc::clone(&a));
    println!("a has reference count {}", Rc::strong_count(&a));
    {
        let c = Node(20, Rc::clone(&a));
        println!("a has reference count {}", Rc::strong_count(&a));
    }

    println!("==== usage of RefCell<T> ===="); 
    println!("a has reference count {}", Rc::strong_count(&a));
    let whatsapp = Whatsapp {};
    let mut tracker = LimitTracker::new(&whatsapp, 25);
    tracker.send_val(20);
    let leaf1 = Rc::new(TreeNode {
        value: 10,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf1),
        Rc::weak_count(&leaf1),
    );

    let leaf2 = Rc::new(TreeNode {
        value: 20,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(TreeNode {
        value: 100,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf1)]),
    });

    println!(
        "branch leaf strong = {}, weak = {}",
        Rc::strong_count(&branch),
        Rc::weak_count(&branch),
    );

    println!("leaf parent = => {:?}", leaf1.parent.borrow().upgrade());

    *leaf1.parent.borrow_mut() = Rc::downgrade(&branch);
    println!(
        "  branch leaf strong = {}, weak = {}",
        Rc::strong_count(&branch),
        Rc::weak_count(&branch),
    );

    println!("leaf parent = => {:?}", leaf1.parent.borrow().upgrade());
    println!("leaf parent = => {:?}", leaf1.parent.borrow().upgrade());

    println!("\n ====={:?}", branch);

    branch.children.borrow_mut().push(Rc::clone(&leaf2));

    println!("\n =====>>{:?}", branch);

    *leaf2.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("leaf parent = => {:?}", leaf2.parent.borrow().upgrade());
    println!("\n =====>>{:?}", branch);

    println!(
        "leaf2 leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf2),
        Rc::weak_count(&leaf2),
    );


    
    println!("\nBye !!"); 
    
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl crate::Messager for MockMessenger {
        fn send(&self, message: &str) {
            let mut one_borrow = self.sent_messages.borrow_mut();
            let mut two_borrow = self.sent_messages.borrow_mut();

            one_borrow.push(String::from(message));
            two_borrow.push(String::from(message));
            
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.send_val(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}