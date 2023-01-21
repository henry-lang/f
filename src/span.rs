#[derive(Debug, Copy, Clone)]
pub struct Span(pub usize, pub usize);

impl Span {
    pub fn union(self, other: Self) -> Self {
        Self(self.0.min(other.0), self.1.max(other.1))
    }

    pub fn len(&self) -> usize {
        self.1 - self.0
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Spanned<T> {
    pub value: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn new(value: T, span: Span) -> Self {
        Self { value, span }
    }

    pub fn empty(value: T) -> Self {
        Self {
            value,
            span: Span(0, 0),
        }
    }
}
