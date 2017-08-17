### 安装

项目使用[Sphinx](http://sphinx-doc.org/) 来构建文档。在已经安装[Python](https://www.python.org/)的基础上，安装[Sphinx](http://sphinx-doc.org/latest/install.html):

```shell
$ pip install sphinx sphinx-autobuild
$ pip install sphinx_rtd_theme
```

### 约定

* 文档源文件以RST格式保存在`docs/source`目录下。
* 文档主目录是 index.rst，增加新文档文件需要更新主目录文件。

### 构建

每次更新文档需要重新构建html文档

```shell
make html
```

