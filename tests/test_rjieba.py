# -*- coding: utf-8 -*-
import rjieba as jieba


def test_cut():
    ret = jieba.cut('')
    assert ret == []

    ret = jieba.cut('南京市长江大桥')
    assert ret == ['南京市', '长江大桥']


def test_cut_all():
    ret = jieba.cut_all('')
    assert ret == []

    ret = jieba.cut_all('南京市长江大桥')
    assert ret == ['南', '南京', '南京市', '京', '京市', '市', '市长', '长', '长江', '长江大桥', '江', '大', '大桥', '桥']


def test_cut_for_search():
    ret = jieba.cut_for_search('')
    assert ret == []

    ret = jieba.cut_for_search('南京市长江大桥')
    assert ret == ['南京', '京市', '南京市', '长江', '大桥', '长江大桥']


def test_tag():
    ret = jieba.tag('')
    assert ret == []

    ret = jieba.tag('南京市长江大桥')
    assert len(ret) == 2
    assert ret[0] == ('南京市', 'ns')
    assert ret[1] == ('长江大桥', 'ns')


def test_tag_with_slash():
    ret = jieba.tag('/ .')
    assert len(ret) == 3
    assert ret[0] == ('/', 'x')
    assert ret[1] == (' ', 'x')
    assert ret[2] == ('.', 'x')


def test_tokenize():
    ret = jieba.tokenize('')
    assert ret == []

    ret = jieba.tokenize('南京市长江大桥')
    assert len(ret) == 2
    assert ret[0] == ('南京市', 0, 3)
    assert ret[1] == ('长江大桥', 3, 7)

    ret = jieba.tokenize('南京南京')
    assert len(ret) == 2
    assert ret[0] == ('南京', 0, 2)
    assert ret[1] == ('南京', 2, 4)

    ret = jieba.tokenize('南京市长江大桥', mode='search')
    assert len(ret) == 6
    assert ret[0] == ('南京', 0, 2)
    assert ret[1] == ('京市', 1, 3)
    assert ret[2] == ('南京市', 0, 3)
    assert ret[3] == ('长江', 3, 5)
    assert ret[4] == ('大桥', 5, 7)
    assert ret[5] == ('长江大桥', 3, 7)
