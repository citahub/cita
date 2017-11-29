## 功能

通过MQ向Chain模块来发送Block来对Chain持续测试

## 方法

1. 启动Chain进程，或者启动单独一个节点
2. 使用命令进行压力测试
3. 注意调整交易配额和交易数，防止block超出block gas limit
4. 在node目录下启动测试进程，这样可以正确的使用.env文件
5. 修改节点目录下的chain.json，将权限检查和配额检查改成false