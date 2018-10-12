# 紧急制动合约接口

<h2 class="hover-list">Emergency brake</h2>

* [setState](#setState)

***

### setState

开启紧急制动模式。

* Parameters

    `bool state` - state

* Returns

    `None`

* Example

```shell
$ cita-cli scm EmergencyBrake setState \
    --state true \
    --admin-private 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
    --url http://127.0.0.1:1337
```
