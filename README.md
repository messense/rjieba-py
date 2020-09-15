# rjieba-py

![CI](https://github.com/messense/rjieba-py/workflows/Master/badge.svg)
[![PyPI](https://img.shields.io/pypi/v/rjieba.svg)](https://pypi.org/project/rjieba)

[jieba-rs](https://github.com/messense/jieba-rs) Python binding.

## Installation

```bash
pip install rjieba
```

## Usage

```python
import rjieba


jieba = rjieba.Jieba()
print(jieba.cut('我们中出了一个叛徒'))
print(jieba.tag('我们中出了一个叛徒'))
```

## License

This work is released under the MIT license. A copy of the license is provided in the [LICENSE](./LICENSE) file.
