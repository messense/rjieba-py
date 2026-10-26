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
clear = _DEFAULT_JIEBA.clear
suggest_freq = _DEFAULT_JIEBA.suggest_freq
add_word = _DEFAULT_JIEBA.add_word
has_word = _DEFAULT_JIEBA.has_word
