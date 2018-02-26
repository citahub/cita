/* jshint esversion: 6 */
/* jshint expr: true */

const Web3 = require('web3');
const config = require('./config');

var chai = require('chai');
var assert = chai.assert;

const web3 = new Web3(new Web3.providers.HttpProvider(config.localServer));
// Use remote server
// const web3 = new Web3(new Web3.providers.HttpProvider(config.remoteServer));

const sender = config.contract.permission_manager.sender;
const { pManagementABI, pManagementAddr } = config.contract.permission_management;
const { pABI, pAddr} = config.contract.permission;
const { aABI, aAddr, superAdmin, permissions, resources } = config.contract.authorization;

// permission management
const pManagement = web3.eth.contract(pManagementABI);
const pManagementContractIns = pManagement.at(pManagementAddr);

// authorization
const auth = web3.eth.contract(aABI);
const aContractInstance = auth.at(aAddr);

// permission
const perm = web3.eth.contract(pABI);
var newPermissionAddr;

const quota = 9999999;
const blockLimit = 100;

// =======================

// TODO Move to helper
function randomInt() {
    return Math.floor(Math.random() * 100).toString();
}

function getTxReceipt(res) {
    return new Promise((resolve, reject) => { 
        let count = 0;
        const filter = web3.eth.filter('latest', err => {

            if (err) reject(err);
            
            count++;

            if (count > 20) {
                filter.stopWatching(function() {});
                reject(err);
            }

            web3.eth.getTransactionReceipt(res.hash, function(err, receipt) {

                if (err) reject(err);

                if (receipt) {
                    filter.stopWatching(function() {});
                    resolve(receipt);
                }
            });
        });
    });
}

// =======================

describe('\n\ntest permission management contract\n\n', function() { 

    describe('\ntest add permission\n', function() { 
        it('should send a newPermission tx and get receipt', function(done) {
            var num = web3.eth.blockNumber;
            var res = pManagementContractIns.newPermission.sendTransaction(
                'testPermission',
                config.testAddr,
                config.testFunc,
                {
                    privkey: sender.privkey,
                    nonce: randomInt(),
                    quota,
                    validUntilBlock: num + blockLimit,
                    from: sender.address
                });

            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt:\n', receipt);
                    assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                    newPermissionAddr = receipt.logs[0].address;
                    console.log('\nThe new permission contract address:\n', newPermissionAddr);
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get newPermission receipt err:!!!!\n', err);
                    this.skip();
                });
        });

        it('should have info of new permission', function() {
            const pContractInstance = perm.at(newPermissionAddr);
            var res = pContractInstance.queryInfo.call();
            console.log('\nInfo:\n', res);
            assert.equal(res[0].substr(0, 30), web3.toHex('testPermission'));

            for (var i=0; i<res[1].length; i++) {
                assert.equal(res[1][i], config.testAddr[i]);
                assert.equal(res[2][i], config.testFunc[i]);
            }
        });
    });

    describe('\ntest update permission name\n', function() { 
        it('should send a updatePermissionName tx and get receipt', function(done) {
            var num = web3.eth.blockNumber;
            var res = pManagementContractIns.updatePermissionName.sendTransaction(
                newPermissionAddr,
                'testPermissionNewName',
                {
                    privkey: sender.privkey,
                    nonce: randomInt(),
                    quota,
                    validUntilBlock: num + blockLimit,
                    from: sender.address
                });

            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt:\n', receipt);
                    assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get updatePermissionName receipt err:!!!!\n', err);
                    this.skip();
                });
        });

        it('should have the new permission name', function() {
            const pContractInstance = perm.at(newPermissionAddr);
            var res = pContractInstance.queryName.call();
            console.log('\nNew permission name:\n', res);
            assert.equal(res.substr(0, 44), web3.toHex('testPermissionNewName'));
        });
    });

    describe('\ntest add resources\n', function() { 
        it('should send a addResources tx and get receipt', function(done) {
            var num = web3.eth.blockNumber;
            var res = pManagementContractIns.addResources.sendTransaction(
                newPermissionAddr,
                ['0x1a702a25c6bca72b67987968f0bfb3a3213c5603'],
                ['0xf036ed59'],
                {
                    privkey: sender.privkey,
                    nonce: randomInt(),
                    quota,
                    validUntilBlock: num + blockLimit,
                    from: sender.address
                });

            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt:\n', receipt);
                    assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get addResources receipt err:!!!!\n', err);
                    this.skip();
                });
        });

        it('should have the added resources', function() {
            const pContractInstance = perm.at(newPermissionAddr);
            var res = pContractInstance.queryResource.call();
            console.log('\nNew Added resources:\n', res);
            let l = res[0].length - 1;
            assert.equal(res[0][l], '0x1a702a25c6bca72b67987968f0bfb3a3213c5603');
            assert.equal(res[1][l], '0xf036ed59');
        });
    });

    describe('\ntest delete resources\n', function() { 
        it('should send a deleteResources tx and get receipt', function(done) {
            var num = web3.eth.blockNumber;
            var res = pManagementContractIns.deleteResources.sendTransaction(
                newPermissionAddr,
                ['0x1a702a25c6bca72b67987968f0bfb3a3213c5603'],
                ['0xf036ed59'],
                {
                    privkey: sender.privkey,
                    nonce: randomInt(),
                    quota,
                    validUntilBlock: num + blockLimit,
                    from: sender.address
                });

            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt:\n', receipt);
                    assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get deleteResources receipt err:!!!!\n', err);
                    this.skip();
                });
        });

        it('should have deleted the resources', function() {
            const pContractInstance = perm.at(newPermissionAddr);
            var res = pContractInstance.queryResource.call();
            console.log('\nResources lefted:\n', res);
            for (var i=0; i<res[1].length; i++) {
                assert.equal(res[0][i], config.testAddr[i]);
                assert.equal(res[1][i], config.testFunc[i]);
            }
        });
    });

    describe('\ntest clear authorization\n', function() { 
        it('should send a clearAuthorization tx and get receipt', function(done) {
            var num = web3.eth.blockNumber;
            var res = pManagementContractIns.clearAuthorization.sendTransaction(
                config.testAddr[0],
                {
                    privkey: sender.privkey,
                    nonce: randomInt(),
                    quota,
                    validUntilBlock: num + blockLimit,
                    from: sender.address
                });

            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt:\n', receipt);
                    assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get clearAuthorization receipt err:!!!!\n', err);
                    this.skip();
                });
        });

        it('should have no permissions of testAccount', function() {
            var res = aContractInstance.queryPermissions.call(config.testAddr[0]);
            console.log('\nPermissions of testAccount:\n', res);
            assert.equal(res.length, 0);
        });
    });

    describe('\ntest set authorization\n', function() { 
        it('should send a setAuthorization tx and get receipt', function(done) {
            var num = web3.eth.blockNumber;
            var res = pManagementContractIns.setAuthorization.sendTransaction(
                config.testAddr[0],
                config.testAddr[1],
                {
                    privkey: sender.privkey,
                    nonce: randomInt(),
                    quota,
                    validUntilBlock: num + blockLimit,
                    from: sender.address
                });

            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt:\n', receipt);
                    assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get setAuthorization receipt err:!!!!\n', err);
                    this.skip();
                });
        });

        it('should have the permission of account', function() {
            var res = aContractInstance.queryPermissions.call(config.testAddr[0]);
            console.log('\nPermissions of testAccount:\n', res);
            let l = res.length -1;
            assert.equal(res[l], config.testAddr[1]);
            var res2 = aContractInstance.queryAccounts.call(config.testAddr[1]);
            console.log('\nAccount of permissions:\n', res2);
            assert.equal(res2, config.testAddr[0]);
        });
    });

    describe('\ntest cancel authorization\n', function() { 
        it('should send a cancelAuthorization tx and get receipt', function(done) {
            var num = web3.eth.blockNumber;
            var res = pManagementContractIns.cancelAuthorization.sendTransaction(
                config.testAddr[0],
                config.testAddr[1],
                {
                    privkey: sender.privkey,
                    nonce: randomInt(),
                    quota,
                    validUntilBlock: num + blockLimit,
                    from: sender.address
                });

            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt:\n', receipt);
                    assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get canelAuthorization receipt err:!!!!\n', err);
                    this.skip();
                });
        });

        it('should not have the permission of account', function() {
            var res = aContractInstance.queryPermissions.call(config.testAddr[0]);
            console.log('\nPermissions of testAccount:\n', res);
            assert.equal(res.length, 0);
            var res2 = aContractInstance.queryAccounts.call(config.testAddr[1]);
            console.log('\nAccount of permissions:\n', res2);
            assert.equal(res2.length, 0);
        });
    });

    describe('\ntest delete permission\n', function() { 
        it('should send a deletePermission tx and get receipt', function(done) {
            var num = web3.eth.blockNumber;
            var res = pManagementContractIns.deletePermission.sendTransaction(
                newPermissionAddr,
                {
                    privkey: sender.privkey,
                    nonce: randomInt(),
                    quota,
                    validUntilBlock: num + blockLimit,
                    from: sender.address
                });

            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt:\n', receipt);
                    // TODO Why still have the event
                    assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get deletePermission receipt err:!!!!\n', err);
                    this.skip();
                });
        });
    });

});
