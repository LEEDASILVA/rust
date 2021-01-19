/*
## lunch_queue

### Instructions

You will have to create an API to organize a queue of people.

Using the given code create the following functions:

- `new` that will initialize the `Queue`.
- `add` that receives the person's information, so it can be added to the `Queue`
- `rm`, that will remove the person that finished ordering their food.
  The method for organizing the manipulation of a data structure should be a
  FIFO (first in first out) process. This function should return a tuple wrapped in an `Option`
  with the information of that person
- `search`, that return a tuple with the information of a given person `id`.

You must also create a type called `Link` this will be the connection of the structures `Queue` and `Person`.
Do not forget that this will be a recursion type and it must point to `None` if there is no persons.


### Expected Function

```rust
pub struct Queue {
    pub node: Link,
}

pub type Link =

pub struct Person {
    pub id: i32,
    pub name: String,
}

impl Queue {
    pub fn new() -> Queue {}
    pub fn add(&mut self, t: String, name: String) {}
    pub fn remove_worker(&mut self) -> Option<String> {}
    pub fn search_worker(&self) -> Option<(String, String)> {}
}
```

### Usage

Here is a program to test your function

```rust
fn main() {
    let mut list = Queue::new();
    list.add(String::from("Marie"), 20);
    list.add(String::from("Monica"), 15);
    list.add(String::from("Ana"), 5);
    list.add(String::from("Alice"), 35);
    println!("{:?}", list);

    println!("{:?}", list.search("Marie"));
    println!("{:?}", list.search("Monica"));
    println!("{:?}", list.search("Ana"));
    println!("{:?}", list.search("Alice"));
    println!("{:?}", list.search("someone"));

    println!("removed {:?}", list.rm());
    println!("list {:?}", list);
}
```

And its output:

```console
student@ubuntu:~/[[ROOT]]/test$ cargo run
Queue { node: Some(Person { name: "Alice", discount: 35, next_person: Some(Person { name: "Ana", discount: 5, next_person: Some(Person { name: "Monica", discount: 15, next_person: Some(Person { name: "Marie", discount: 20, next_person: None }) }) }) }) }
Some(("Marie", 20))
Some(("Monica", 15))
Some(("Ana", 5))
Some(("Alice", 35))
None
removed Some(("Marie", 20))
list Queue { node: Some(Person { name: "Alice", discount: 35, next_person: Some(Person { name: "Ana", discount: 5, next_person: Some(Person { name: "Monica", discount: 15, next_person: None }) }) }) }
student@ubuntu:~/[[ROOT]]/test$
```

### Notions

- https://doc.rust-lang.org/rust-by-example/custom_types/enum.html
- https://doc.rust-lang.org/book/ch15-01-box.html
- https://doc.rust-lang.org/std/option/
- https://doc.rust-lang.org/book/ch15-01-box.html
*/

#[derive(Debug, Clone)]
pub struct Queue {
    pub node: Link,
}

pub type Link = Option<Box<Person>>;

#[derive(Debug, Clone)]
pub struct Person {
    pub name: String,
    pub discount: u32,
    pub next_person: Link,
}

impl Queue {
    pub fn new() -> Queue {
        Queue { node: None }
    }

    pub fn add(&mut self, name: String, discount: u32) {
        let new_node = Box::new(Person {
            name,
            discount,
            next_person: self.node.take(),
        });
        self.node = Some(new_node);
    }
    pub fn invert_queue(&mut self) {
        let mut q = Queue::new();
        recursion_inv(&self.clone().node, &mut q);
        *self = q;
    }

    pub fn rm(&mut self) -> Option<(String, u32)> {
        if self.clone().node.as_ref().is_none() {
            return None;
        }
        let mut q = Queue::new();
        let result = recursion_rm(&self.clone().node, &mut q);
        *self = q;
        self.invert_queue();
        return Some(result);
    }

    pub fn search(&self, s :&str) -> Option<(String, u32)> {
        recursion(&self.clone().node, s.to_string())
    }
}
fn recursion(node: &Link, s: String) -> Option<(String, u32)> {
    if node.as_ref().is_none() {
        return None;
    }
    let a = node.as_ref().unwrap();
    if a.name == s {
        return Some((a.name.clone(), a.discount.clone()));
    }
    return recursion(&node.as_ref().unwrap().next_person, s);
}

fn recursion_rm(node: &Link, q: &mut Queue) -> (String, u32) {
    let a = node.as_ref().unwrap();
    if !a.next_person.is_none() { 
        q.add(a.name.clone(), a.discount.clone());
        return recursion_rm(&node.as_ref().unwrap().next_person, q);
    } else {
        return (a.as_ref().name.clone(), a.as_ref().discount.clone());
    }
}

fn recursion_inv(node: &Link, q: &mut Queue) {
    let a = node.as_ref();
    if !a.is_none() {
        q.add(a.unwrap().name.clone(),a.unwrap().discount.clone());
        return recursion_inv(&node.as_ref().unwrap().next_person, q);
    }
}

// Example :
fn main() {
    let mut list = Queue::new();
    list.add(String::from("Marie"), 20);
    list.add(String::from("Monica"), 15);
    list.add(String::from("Ana"), 5);
    list.add(String::from("Alice"), 35);
    println!("{:?}", list);
    
    // output:
    // Queue { node: Some(Person { name: "Alice", discount: 35, next_person: Some(Person { name: "Ana", discount: 5, next_person: Some(Person { name: "Monica", discount: 15, next_person: Some(Person { name: "Marie", discount: 20, next_person: None }) }) }) }) }
    println!("{:?}", list.search("Marie"));
    println!("{:?}", list.search("Alice"));
    println!("{:?}", list.search("someone"));
    // output:
    // Some(("Marie", 20))
    // Some(("Alice", 35))
    // None
    
    println!("removed {:?}", list.rm());
    println!("removed {:?}", list.rm());
    println!("list {:?}", list);
    list.invert_queue();
    println!("inverted list {:?}", list);
    // output:
    // removed Some(("Marie", 20))
    // list Queue { node: Some(Person { name: "Alice", discount: 35, next_person: Some(Person { name: "Ana", discount: 5, next_person: Some(Person { name: "Monica", discount: 15, next_person: None }) }) }) }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_new() {
//         let list = Queue::new();
//         assert!(list.grade.is_none());
//     }

//     #[test]
//     fn test_one_worker() {
//         let mut list = Queue::new();
//         list.add_worker(String::from("CEO"), String::from("Marie"));
//         list.remove_worker();
//         assert!(list.grade.is_none());
//     }

//     #[test]
//     fn test_two_workers() {
//         let mut list = Queue::new();
//         list.add_worker(String::from("CEO"), String::from("Marie"));
//         list.add_worker(String::from("Manager"), String::from("Monica"));
//         list.remove_worker();

//         assert_eq!(list.grade.as_ref().unwrap().worker_type, "CEO");
//         assert_eq!(list.grade.as_ref().unwrap().worker_name, "Marie");
//     }

//     #[test]
//     fn test_more_workers() {
//         let mut list = Queue::new();
//         list.add_worker(String::from("CEO"), String::from("Marie"));
//         list.add_worker(String::from("Manager"), String::from("Monica"));
//         list.add_worker(String::from("Normal Person"), String::from("Ana"));
//         list.add_worker(String::from("Normal Person"), String::from("Alice"));
//         list.remove_worker();

//         assert_eq!(list.grade.as_ref().unwrap().worker_type, "Normal Person");
//         assert_eq!(list.grade.as_ref().unwrap().worker_name, "Ana");

//         list.remove_worker();
//         list.remove_worker();
//         assert_eq!(list.grade.as_ref().unwrap().worker_type, "CEO");
//         assert_eq!(list.grade.as_ref().unwrap().worker_name, "Marie");
//     }

//     #[test]
//     fn test_search() {
//         let mut list = Queue::new();
//         list.add_worker(String::from("CEO"), String::from("Marie"));
//         list.add_worker(String::from("Manager"), String::from("Monica"));
//         list.add_worker(String::from("Normal Person"), String::from("Ana"));
//         list.add_worker(String::from("Normal Person"), String::from("Alice"));

//         assert_eq!(
//             list.search_worker().unwrap(),
//             (String::from("Alice"), String::from("Normal Person"))
//         );

//         list.remove_worker();
//         assert_eq!(
//             list.search_worker().unwrap(),
//             (String::from("Ana"), String::from("Normal Person"))
//         );

//         list.remove_worker();
//         assert_eq!(
//             list.search_worker().unwrap(),
//             (String::from("Monica"), String::from("Manager"))
//         );

//         list.remove_worker();
//         assert_eq!(
//             list.search_worker().unwrap(),
//             (String::from("Marie"), String::from("CEO"))
//         );
//     }
// }
