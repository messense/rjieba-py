use pyo3::prelude::*;
use std::sync::Mutex;

#[pyclass(subclass)]
struct Jieba {
    jieba: Mutex<jieba_rs::Jieba>,
}

#[pymethods]
impl Jieba {
    #[new]
    fn new() -> Self {
        Self {
            jieba: Mutex::new(jieba_rs::Jieba::new()),
        }
    }

    /// Add word to the dictionary
    #[pyo3(signature = (word, freq = 0, tag = None))]
    fn add_word<'a>(
        &self,
        py: Python,
        word: &'a str,
        freq: Option<usize>,
        tag: Option<&str>,
    ) -> usize {
        py.allow_threads(move || self.jieba.lock().unwrap().add_word(word, freq, tag))
    }

    /// Cut the input text
    #[pyo3(signature = (text, hmm = true))]
    fn cut<'a>(&self, py: Python, text: &'a str, hmm: bool) -> Vec<&'a str> {
        py.allow_threads(move || self.jieba.lock().unwrap().cut(text, hmm))
    }

    /// Cut the input text, return all possible words
    #[pyo3(signature = (text,))]
    fn cut_all<'a>(&self, py: Python, text: &'a str) -> Vec<&'a str> {
        py.allow_threads(move || self.jieba.lock().unwrap().cut_all(text))
    }

    /// Cut the input text in search mode
    #[pyo3(signature = (text, hmm = true))]
    fn cut_for_search<'a>(&self, py: Python, text: &'a str, hmm: bool) -> Vec<&'a str> {
        py.allow_threads(move || self.jieba.lock().unwrap().cut_for_search(text, hmm))
    }

    /// Tag the input text
    #[pyo3(signature = (text, hmm = true))]
    fn tag<'a>(&'a self, py: Python, text: &'a str, hmm: bool) -> Vec<(String, String)> {
        py.allow_threads(move || {
            self.jieba
                .lock()
                .unwrap()
                .tag(text, hmm)
                .into_iter()
                .map(|t| (t.word.to_string(), t.tag.to_string()))
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
                .lock()
                .unwrap()
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
