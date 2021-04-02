const SEPARATOR: &str = "/";

#[derive(Clone, Debug)]
pub struct Url {
    segments: Vec<String>,
}

impl Url {
    pub fn new<S: Into<String>>(root: S) -> Self {
        Self {
            segments: vec![root.into()],
        }
    }

    pub fn join<S: Into<String>>(mut self, segment: S) -> Self {
        self.segments.push(segment.into());
        self
    }

    pub fn as_string(&self) -> String {
        self.segments.join(SEPARATOR)
    }

    pub fn local_as_string(&self) -> String {
        format!(
            "{}{}",
            SEPARATOR,
            self.segments
                .iter()
                .cloned()
                .skip(1)
                .collect::<Vec<String>>()
                .join(SEPARATOR)
        )
    }
}

#[allow(clippy::from_over_into)]
impl Into<String> for Url {
    fn into(self) -> String {
        self.as_string()
    }
}
