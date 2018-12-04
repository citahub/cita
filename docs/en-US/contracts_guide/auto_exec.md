# 自动执行合约接口

<h2 class="hover-list">Auto Exec</h2>

* [register](#register)
* [autoExec](#autoExec)
* [contAddr](#contAddr)

### register

注册自动执行合约，只能管理员调用，新注册的地址会覆盖旧地址。

* Parameters

    `address` - The address to be setted

* Returns

    None

### autoExec

仅供底层调用的接口，不对用户开放。

* Parameters

    None

* Returns

    None

### contAddr

* Parameters

    None

* Returns

    `address` - The address of the registerd contract
