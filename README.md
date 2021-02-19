# rjieba-py

![CI](https://github.com/messense/rjieba-py/workflows/CI/badge.svg)
[![PyPI](https://img.shields.io/pypi/v/rjieba.svg)](https://pypi.org/project/rjieba)

[jieba-rs](https://github.com/messense/jieba-rs) Python binding.

## Installation

```bash
pip install rjieba
```

## Usage

```python
import rjieba


print(rjieba.cut('我们中出了一个叛徒'))
print(rjieba.tag('我们中出了一个叛徒'))
```

## Performance

Running on MacBook Pro (15-inch, 2018) 2.2 GHz 6-Core Intel Core i7

```python
In [1]: import jieba

In [2]: import cjieba

In [3]: import rjieba

In [4]: jieba.initialize()
Building prefix dict from the default dictionary ...
Loading model from cache /var/folders/8d/h3lyjgz14296j_lw7chgf5hc0000gp/T/jieba.cache
Loading model cost 0.695 seconds.
Prefix dict has been built successfully.

In [5]: cjieba.initialize()

In [6]: with open('../jieba-rs/examples/weicheng/src/weicheng.txt') as f:
   ...:     txt = f.read()
   ...:

In [7]: %timeit list(jieba.cut(txt))
1.1 s ± 10.6 ms per loop (mean ± std. dev. of 7 runs, 1 loop each)

In [8]: %timeit cjieba.cut(txt)
225 ms ± 3.95 ms per loop (mean ± std. dev. of 7 runs, 1 loop each)

In [9]: %timeit rjieba.cut(txt)
106 ms ± 2.01 ms per loop (mean ± std. dev. of 7 runs, 10 loops each)
```

## License

This work is released under the MIT license. A copy of the license is provided in the [LICENSE](./LICENSE) file.
