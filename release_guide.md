# Release Guide

TBD

----------------------

# 发版指引

代码仓库拥有两个主要分支：

* `develop`: nightly build.
* `master`: production release.

基本的发版流程是当 `develop` 的代码可以作为一个版本发布时，所有代码的改动都需要合并回 `master` 分支，然后标记新的版本号。
其中的一些细节将会在下面章节阐述。

具体流程遵循以下几个步骤：

1. [准备分支](#准备分支)
2. [测试及更新日志](#测试及更新日志)
3. [合并分支](#合并分支)
4. [发布](#发布)
5. [其他](#其他)

## 准备分支

### 版本的类型

使用语义化版本，遵循 [Semantic Versioning]

摘录部分如下：

> 版本格式：主版本号.次版本号.修订号，版本号递增规则如下：
>
> 1. 主版本号：当你做了不兼容的 API 修改，
> 2. 次版本号：当你做了向下兼容的功能性新增，
> 3. 修订号：当你做了向下兼容的问题修正。
> 先行版本号及版本编译元数据可以加到“主版本号.次版本号.修订号”的后面，作为延伸。

### 创建发布分支

***新分支命名遵循 `release-*` 格式***

首先获取最新的代码，具体操作如下：

```
git fetch origin
```

其中 `origin` 代表 `CITA` 官方的代码仓库地址，可通过 `git remote -v` 查看。

流程分为版本发布（主版本号 x 及次版本号 y）以及修订补丁发布（修订号 z）。

#### 版本发布

从最新 `develop` 创建新的分支 `release-x.y.0`，具体操作如下：

```
git checkout origin/develop -b release-x.y.0
```

#### 修订补丁发布

从需要修订版本的 `tag`(`vx.y.0`) 创建新的分支 `release-x.y.1`，具体操作如下：

```
git checkout tags/vx.y.0 -b release-x.y.1
```

### 推送分支

把创建的发布分支 `release-x.y.z` 推送到官方仓库，具体操作如下：

```
git push origin release-x.y.z
```

### 设置分支保护

联系管理员对发布分支 `release-x.y.z` 设置分支保护。

## 测试及更新日志

广播内部邮件：

* 告知测试团队对发布分支进行测试
* 告知工具链团队对新版本就行适配

测试过程中出现的 Bug 修复合并入发布分支 `release-x.y.z`。

当测试通过时，修改更新日志，更新版本发布日期等信息。

## 合并分支

### 合并入 `master` 分支

通过 `pull request` 把发布分支 `release-x.y.z` 合并入 `master` 分支。

### 版本 tag

***新版本 tag 命名遵循 `vx.y.z` 格式***

1. 更新 `master` 分支代码，具体操作如下：

```
git checkout origin/master && git pull
```

2. 创建 `tag`，具体操作如下：

```
git tag -a vx.y.z -m 'vx.y.z'
```

3. 推送 `tag` 到代码仓库，具体操作如下：

```
git push origin vx.y.z
```

### 合并入 `develop` 分支

***过渡分支命名为 `merge-master-to-develop`***

把 `master` 分支合并入 `develop` 分支，由于分支保护，创建一个中间分支 `merge-master-to-develop` 通过 `pull request` 合并。

1. 更新 `master` 分支代码，具体操作如下：

```
git checkout origin/master && git pull
```

2. 创建 `merge-master-to-develop`，具体操作如下：

```
git checkout -b merge-master-to-develop
```

3. 推送分支到代码仓库，具体操作如下：

```
git push origin merge-master-to-develop
```

4. 合并入 `develop`

通过 `pull request` 把过渡分支 `merge-master-to-develop` 合并入 `develop` 分支。

## 发布

### 文件打包

***压缩包使用 .tar.gz 后缀***

新版本发布需要打包以下文件：

* 源码 `cita_src.tar.gz`
* 三种不同算法的发布件：
    - `cita_secp256k1_sha3.tar.gz`
    - `cita_ed25519_blake2b.tar.gz`
    - `cita_sm2_sm3.tar.gz`

### 发版公告

1. 起草一份发版公告，简述新版本的内容及升级提示，包括中文和英语，最后附上更新日志。
2. 把发布需要的文件上传。
3. 发布新版本。

### 邮件广播

撰写新版本邮件并广播。

### CITAHub talk 发帖

在[信息版]发布新版本主题贴。

### 清理分支

***如果设置了分支保护，联系管理员手动删除***

1. 清理发布分支，具体操作如下：

```
git push --delete origin release-x.y.z
```

2. 清理过渡分支，具体操作如下：

```
git push --delete origin merge-master-to-develop
```

### CITAHub Docs 版本更新

[CITAHub Docs] 延后三天进行版本更新，期间进行文档的测试和补充。

## 其他

* 若发布之后，需要修改发版公告，交由发版人操作。

[CITAhub Docs]: https://docs.citahub.com/zh-CN/welcome
[Semantic Versioning]: https://semver.org/
[信息版]: https://talk.citahub.com/c/9-category
