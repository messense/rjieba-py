use pyo3::prelude::*;

#[pyclass(subclass)]
struct Jieba {
    jieba: jieba_rs::Jieba,
}

#[pymethods]
impl Jieba {
    #[new]
    fn new() -> Self {
        Self {
            jieba: jieba_rs::Jieba::new(),
        }
    }

    /// Cut the input text
    #[args(hmm = "true")]
    #[text_signature = "($self, text, hmm)"]
    fn cut<'a>(&self, py: Python, text: &'a str, hmm: bool) -> Vec<&'a str> {
        py.allow_threads(move || self.jieba.cut(text, hmm))
    }

    /// Cut the input text, return all possible words
    #[text_signature = "($self, text)"]
    fn cut_all<'a>(&self, py: Python, text: &'a str) -> Vec<&'a str> {
        py.allow_threads(move || self.jieba.cut_all(text))
    }

    /// Cut the input text in search mode
    #[args(hmm = "true")]
    #[text_signature = "($self, text, hmm)"]
    fn cut_for_search<'a>(&self, py: Python, text: &'a str, hmm: bool) -> Vec<&'a str> {
        py.allow_threads(move || self.jieba.cut_for_search(text, hmm))
    }

    /// Tag the input text
    #[args(hmm = "true")]
    #[text_signature = "($self, text, hmm)"]
    fn tag<'a>(&'a self, py: Python, text: &'a str, hmm: bool) -> Vec<(&'a str, &'a str)> {
        py.allow_threads(move || {
            self.jieba
                .tag(text, hmm)
                .into_iter()
                .map(|t| (t.word, t.tag))
                .collect()
        })
    }

    /// Tokenize
    #[args(mode = "\"default\"", hmm = "true")]
    #[text_signature = "($self, text, mode, hmm)"]
    fn tokenize<'a>(
        &self,
        py: Python,
        text: &'a str,
        mode: &str,
        hmm: bool,
    ) -> Vec<(&'a str, usize, usize)> {
        let tokenize_mode = if mode.to_lowercase() == "search" {
            jieba_rs::TokenizeMode::Search
        } else {
            jieba_rs::TokenizeMode::Default
        };
        py.allow_threads(move || {
            self.jieba
                .tokenize(text, tokenize_mode, hmm)
                .into_iter()
                .map(|t| (t.word, t.start, t.end))
                .collect()
        })
    }
}

#[pymodule]
fn rjieba(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Jieba>()?;
    Ok(())
}
