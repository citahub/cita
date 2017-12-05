## CITA message flow chart

### CITA

#### before auth
*detail at [ CITA doc](http://cita.readthedocs.io/zh_CN/latest/transaction_process.html)*

```
                                tx into chain
-------------------------------------------------------------------------------------------------

+-----+  1.send   +---------+  2.forward   +-----------+  3.2.broadcast   +---------+
| APP |  ------>  | jsonRPC |  --------->  | consensus |  ------------->  | network |
+-----+           +---------+              +-----------+                  +---------+
   ^                |  ^                      |  |             
   |  4.reply       |  |    3.1.reply         |  |            
   -----------------   -----------------------   |             
                                                 |  5.package    +-------+  6.process  +---------+
                                                 ------------->  | chain |  -------->  | rocksDB |
                                                                 +-------+             +---------+
                                                    
```

#### after auth

```
                                                            node0 (tx into chain)                                               node1                                
---------------------------------------------------------------------------------------------------------                   -----------              
                                                                                                                                    
                                               // reply when reveive the tx from the remote                                
                    ---------------------------------------------------------------------------------                                   
                    |                                                                               ^
                    |  ----------------------------------------------                               ^                                    
                    |  |               3.1.reply                    ^                               ^                                    
                    |  |                                            |                               |                                    
+-----+  0.send   +---------+  1.forward   +------+  2.2.send  +-----------+    3.2.send      +---------+   broadcast:tx   +---------+               
| APP |  ------>  | jsonRPC |  --------->  | auth |  ------->  | consensus |  ------------->  | network |  <------------>  | network | ...
+-----+           +---------+              +------+            +-----------+                  +---------+                  +---------+               
   ^                | ^  ^                   | ^ ^                    ^                               ^                                    
   |  4.reply       | ^  |     2.1.reply     | ^ | // tx form remote  |            4.brocadcast tx    |                                    
   ------------------ ^  --------------------- ^ ---------------------|--------------------------------                                   
                      ^                        ^                      |                                                                      
                      ^                        |             6.2.sync | 5.package      +-------+  6.1.process                                        
                      |                        |                      -------------->  | chain |  ---->                                            
                      |                        |                                       +-------+      |                                    
                      |                        |       6.3.sync tx hash                  |  |         |                                        
                      |                        -------------------------------------------  |         |                                        
                      |                                                                     |     +---------+                                 
                      -----------------------------------------------------------------------     | rocksDB |                                 
                                                6.4.reply                                         +---------+                                 
```
**timing diagram**

*detail at [ CITA design doc](https://bitbucket.org/cryptape/cita_doc/src/36fa05f169cce18ce7597c511a3a48363f257698/documents/cita%E6%9E%B6%E6%9E%84%E8%AE%BE%E8%AE%A1.pdf?at=master&fileviewer=file-view-default)*

```
message timing diagram 
----------------------

+-----+          +---------+             +------+                   +-----------+         +-------+              +---------+               +-----------+
| APP |          | jsonRPC |             | auth |                   | consensus |         | chain |              | network |               | other_node|
+-----+          +---------+             +------+                   +-----------+         +-------+              +---------+               +-----------+
   | -----new_tx----> |                      |                            |                   |                        |                          |              
   |                  | --------new_tx-----> |-------------+              |                   |                        |                          |              
   |                  |                      |   tx in pool and check     |                   |                        |                          |              
   |                  |                      |<------------+              |                   |                        |                          |              
   |                  | <-------error------- |                            |                   |                        |                          |              
   | <----error------ |                      | ----------broadcast tx------------------------------------------------> |                          |              
   |                  |                      |                            |                   |                        | ----flooding new tx----> |-----+              
   |                  |                      |                            |                   |                        |                          |  reflect     
   |                  |                      |                            |                   |                        | <---flooding new tx----- |<----+         
   |                  |                      | <---------receive new_tx----------------------------------------------- |                          |              
   |                  |                      | ----------package_tx-----> |---------+         |                        |                          |              
   |                  |                      |                            | consensus txs     |                        |                          |              
   |                  |                      |                            |---------+         |                        |                          |              
   |                  |                      |                            | ----new_block---> |-----+                  |                          |              
   |                  |                      |                            |                   | add block              |                          |              
   |                  |                      |                            |                   |<----+                  |                          |              
   |                  |                      | <----------tx hash---------------------------- |                        |                          |              
   |                  |                      |                            |                   |                        |                          |              
   |                  |                      |                            | <---new status--- | ---broadcast status--> |                          |              
   |                  |                      |                            |                   |                        | --flooding new status--> |-----+              
   | ----request----> |                      |                            |                   |                        |                          |     | 
   |                  | -----get receipt----------------------------------------------------> |                        |                          |  reflect
   |                  | <----receipt--------------------------------------------------------- |                        |                          |     |         
   | <---receipt----- |                      |                            |                   |                        | <--flooding new status-- |<----+         
   |                  |                      |                            |                   | <-----new status------ |                          |              
   |                  |                      |                            |                   |-------+                |                          |              
   |                  |                      |                            |                   |  if need sync          |                          |              
   |                  |                      |                            |                   |<------+                |                          |              
   |                  |                      |                            |                   | ---request block-----> |                          |              
   |                  |                      |                            |                   |                        | --request block--------> |-----+              
   |                  |                      |                            |                   |                        |                          |  reflect            
   |                  |                      |                            |                   |                        | <-response block-------- |<----+              
   |                  |                      |                            |                   | <--response block----- |                          |              
   |                  |                      |                            |                   | -----+                 |                          |              
   |                  |                      |                            |                   |  add block             |                          |              
   |                  |                      |                            |                   | <----+                 |                          |              
   -                  -                      -                            -                   -                        -                          -              
```

* tx from the local
* tx from the remote
* check tx
* block from the local
* block from the remote
* auth 
*detail at [ CITA-224](https://cryptape.atlassian.net/browse/CITA-244)*
    1. 交易池移入到auth模块;
    2. 新的proposal中的交易的打包由交易池完成，并通过rabbitmq发送到consensus模块，然后由consensus模块继续后续的共识操作;
    3. jsonrpc模块进行新交易的打包发送，auth模块在收到新交易时，进行统一的验证签名操作，同时写入到交易池;
    4. 同时改变原有的network将新交易发送到consensus的数据流向，改变为由auth模块接收自network的消息新交易数据.

```
auth flow chart: undo
---------------
```
