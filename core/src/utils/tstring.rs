use heapless::{String, Vec};

#[derive(Clone, Copy)]
pub struct TString<const CAP: usize> {
    length: usize,
    content: [u8; CAP],
}

impl<const CAP: usize> Default for TString<CAP> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const CAP: usize> TString<CAP> {
    pub fn new() -> Self {
        let content = [0_u8; CAP];
        Self { content, length: 0 }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Self {
        let mut content = [0_u8; CAP];
        content[0..s.len()].copy_from_slice(s.as_bytes());
        Self {
            content,
            length: s.len(),
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn to_string(&self) -> String<CAP> {
        let mut v = Vec::<u8, CAP>::new();
        v.extend_from_slice(&self.content[0..self.length]).unwrap();
        // self.content is proven utf8, so unsafe is ok
        unsafe { String::from_utf8_unchecked(v) }
    }

    pub fn from_string(&mut self, s: String<CAP>) {
        self.content[0..s.len()].copy_from_slice(s.as_bytes());
        self.length = s.len();
    }

    pub fn as_str(&self) -> &str {
        // self.content is proven utf8, so unsafe is ok
        unsafe { core::str::from_utf8_unchecked(&self.content[0..self.length]) }
    }
}

impl<const CAP: usize> PartialEq for TString<CAP> {
    fn eq(&self, other: &TString<CAP>) -> bool {
        self.content[0..self.length] == other.content[0..other.length]
    }
}
