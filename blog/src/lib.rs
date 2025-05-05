pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content, 
            num_approvals: 0,
        }
    }
}
#[derive(Debug)]
pub struct PendingReviewPost {
    content: String,
    num_approvals: u32,
}


impl PendingReviewPost {
    pub fn approve(self) -> Result<Post,Self> {
        match self.num_approvals {
            1 => Ok(Post {
                content: self.content,
            }),
            _ => {
                Err(Self { content: self.content, num_approvals: self.num_approvals+1 })
            }
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    } 
}