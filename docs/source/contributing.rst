贡献代码
====================================
 
CITA是一个开源项目，任何人都可以参与CITA并贡献代码。

以下是参与CITA并贡献的具体流程：

**1. 复制CITA到自己的仓库中 (Fork repository)**    

首先，进入CITA的Github主页，点击右上角的Fork按钮将CITA复制到自己的Github仓库。

Fork完成后，进入自己的Github主页查看是否已经存在CITA，若存在，说明Fork成功。

**2. 创建新的分支 (Create new branch)**

从自己的Github主页中进入CITA仓库，点击右侧的Clone or download按钮，复制HTTPS链接。

然后在本地终端使用 ``git clone <上一步复制的链接>`` 命令将该项目下载到本地。

下载成功后，使用 ``git checkout -b <new branch name>`` 命令创建新分支并切换到该分支。

分支名应尽量简洁并能体现出该分支完成的工作。

**3. 提交修改 (Commit changes)**

这时就可以在这条分支下贡献自己的代码或对CITA进行修改完善，然后通过 ``git commit -m "commit message"`` 将修改提交到本地git仓库中(在提交之前，首先通过git add命令添加修改文件到暂存区)。

新增的代码编码风格参照项目主分支风格，尽量保持于主分支编码风格相同。

通常来说，每次提交应该是原子性的并且修改要容易阅读，尽量避免将文本格式的修改和代码位置的转移与实际的修改代码混淆。

提交信息（Commit message）应该尽量简短精确，并且应该加上修改或新增的文件所在包名作为前缀。

**4. 将修改上传到你的仓库 (Push changes to your fork)**

在本地提交修改并确认无误之后，便可以使用 ``git push origin <branch name>`` 命令上传这些修改到自己的仓库。

**5. 创建并提交PR (Create pull request)**

在同步完成后，即可通过Create pull request按钮将该分支发送给该项目的维护者等待被合并。如果你的Pull request修复了在Issues中描述的问题，可以使用 `special magic keyword <https://help.github.com/articles/closing-issues-via-commit-messages/>`_ 引用该Issue为参考
  
一个Pull request应该只关注于一件事情，如添加功能，修补bug，重构代码。尽量避免包含这些不同的事情的混杂的Pull request，因为这些Pull request一般内容较多且非常复杂，维护者阅读或者贡献者回过头来查看的时候会造成许多麻烦。 

当你的PR被维护者接受并准备合并时，你可能会被要求rebase你的提交，rebase的流程如下所示：

.. code-block:: sh
    :linenos:

    git checkout <your branch name>
    git rebase -i HEAD~n
    # n is normally the number of commits in the pull
    # set commits from 'pick' to 'squash', save and quit
    # on the next screen, edit/refine commit messages
    # save and quit
    git push -f
    # (force push to GitHub)

在添加一个新功能的时候，必须考虑到对该功能的长期维护，在提交一个新功能的时候，你必须考虑清楚自己是否会在将来长期维护它（包括修复bug），那些在未来没有得到有效维护的功能，可能会被项目的维护者移除
  
重构的PR不应该改变原先这部分代码的行为（就算是这部分代码中存在bug），如果要修复bug，应该在另外一个Pull request中提出。



**总的来说，所有的Pull request应该：**

- 有明确的方向，修复一个显而易见的bug或者优化项目的一个特性（如模块化重构）

- 再次查看时应该清晰易读

- 在合适的地方有单元测试

- 遵循主分支编码风格

- 不要破坏已有的测试套件

CITA 是采用Rust语言编写而成，若要在本机上编译并调试源码，请参考 `安装依赖 <dependencies.html>`_

以上步骤，如有对git命令不熟悉的，请参考 `git使用手册 <https://git-scm.com/doc>`_
