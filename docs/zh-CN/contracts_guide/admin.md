# 管理员管理合约接口

<h2 class="hover-list">Admin Management</h2>

* [admin](#admin)
* [isAdmin](#isAdmin)
* [update](#update)

***

### admin

查询当前的管理员账户地址

* 参数

    空

* 返回值

    `address` - 管理员地址

### isAdmin

判断账户是否是管理员

* 参数

    `address` - 待判断的管理员地址

* 返回值

    `bool` - 是管理员则为真，反之则反

### update

更新管理员账户

* 参数

    `address` - 待更新的管理员地址

* 返回值

    `bool` - 更新成功则为真，反之则反
