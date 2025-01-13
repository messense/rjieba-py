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
    #[pyo3(signature = (text, hmm = true))]
    fn cut<'a>(&self, py: Python, text: &'a str, hmm: bool) -> Vec<&'a str> {
        py.allow_threads(move || self.jieba.cut(text, hmm))
    }

    /// Cut the input text, return all possible words
    #[pyo3(signature = (text,))]
    fn cut_all<'a>(&self, py: Python, text: &'a str) -> Vec<&'a str> {
        py.allow_threads(move || self.jieba.cut_all(text))
    }

    /// Cut the input text in search mode
    #[pyo3(signature = (text, hmm = true))]
    fn cut_for_search<'a>(&self, py: Python, text: &'a str, hmm: bool) -> Vec<&'a str> {
        py.allow_threads(move || self.jieba.cut_for_search(text, hmm))
    }

    /// Tag the input text
    #[pyo3(signature = (text, hmm = true))]
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
    #[pyo3(signature = (text, mode = "default", hmm = true))]
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
fn rjieba(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_class::<Jieba>()?;
    Ok(())
}
