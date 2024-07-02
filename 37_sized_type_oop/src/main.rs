// In this example, it shows an example to implement an object oriented programming in rust.
// However, rust highly recommands to use a type system but not in this way.

// Generic assumes that function arguments are sized.
// Thereforem following is a default option for generic functions.
fn foo_sized<T: Sized>(fixed_size_data: &T) {}

// If function assumes that value may not be sized, ?Sized can be used instead of it.
// In such case, some type of pointer-like type is required.
// In this case, reference has been used for it.
// Notice that it still works with sized type.
fn foo_may_not_sized<T: ?Sized>(fixed_size_data: &T) {}

// State trait has three functions that changes a state to another state.
trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

// Trait object can be used as a type.
// However, trait doesn't gurantee a size of it.
// Therefore, it requires a dynamic sized type.
// To denote it, dyn keyword has been used.
// Option has been used to make it invalid when it shifted.
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    // It generates a Draft state
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn content(&self) -> &str {
        // Delegates a function to content function in object's State trait.
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        // it changes self to None after take() happens.
        if let Some(s) = self.state.take() {
            // Again, state changes with deligation.
            self.state = Some(s.request_review())
        }
    }
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            // Here deligation happens again.
            self.state = Some(s.approve())
        }
    }
}

// Each state have own implementation for each deligated functions.
struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}
struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
