// In the Rust implementation of the state pattern, each state is their own type.
// This allows us to skip duplication of behaviour and disallow invalid states.
// For example, it is impossible to get the content of a `DraftPost`, because that function is not implemented.
// A `DraftPost` must first be transformed to a `PendingReviewPost` by calling the `request_review()` method.
// Then, that must be transformed to the `Post` with content by calling the `approve()` method.
pub struct Post {
    content: String
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new()
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

pub struct DraftPost {
    content: String
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content
        }
    }
}

pub struct PendingReviewPost {
    content: String
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content
        }
    }
}