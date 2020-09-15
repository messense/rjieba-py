# -*- coding: utf-8 -*-
from .rjieba import Jieba


__all__ = [
    "Jieba",
    "cut",
    "cut_all",
    "cut_for_search",
    "tag",
    "tokenize",
]


_DEFAULT_JIEBA = Jieba()

cut = _DEFAULT_JIEBA.cut
cut_all = _DEFAULT_JIEBA.cut_all
cut_for_search = _DEFAULT_JIEBA.cut_for_search
tag = _DEFAULT_JIEBA.tag
tokenize = _DEFAULT_JIEBA.tokenize
