# 自动执行合约接口

<h2 class="hover-list">Auto Exec</h2>

* [register](#register)
* [autoExec](#autoExec)
* [contAddr](#contAddr)

### register

注册自动执行合约，只能管理员调用，新注册的地址会覆盖旧地址。

* 参数

    `address` - 待注册的自动执行合约地址

* 返回值

    空

### autoExec

仅供底层调用的接口，不对用户开放。

* 参数

    空

* 返回值

    空

### contAddr

* 参数

    空

* 返回值

    `address` - 已注册的自动执行合约地址
