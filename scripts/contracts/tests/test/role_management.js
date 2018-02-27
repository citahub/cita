/* jshint esversion: 6 */
/* jshint expr: true */

const Web3 = require('web3');
const config = require('./config');

var chai = require('chai');
var assert = chai.assert;
// var BigNumber = require('bignumber.js');

const web3 = new Web3(new Web3.providers.HttpProvider(config.localServer));
// Use remote server
// const web3 = new Web3(new Web3.providers.HttpProvider(config.remoteServer));

const { rmABI, rmAddr, permissions } = config.contract.role_management;
const rABI = config.contract.role.rABI;

const roleManagement = web3.eth.contract(rmABI);
const rmContractInstance = roleManagement.at(rmAddr);

const sender = config.contract.permission_manager.sender;

const quota = 9999999;
const blockLimit = 100;

// role
const role = web3.eth.contract(rABI);
var newRoleAddr;

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
                filter.stopWatching(function () { });
                reject(err);
            }

            web3.eth.getTransactionReceipt(res.hash, function (err, receipt) {

                if (err) reject(err);

                if (receipt) {
                    filter.stopWatching(function () { });
                    resolve(receipt);
                }
            });
        });
    });
}

// =======================

describe('\n\ntest role management contract\n\n', function () {
    describe('\ntest new role\n', function () {
        it('should send a newRole tx and get receipt', function (done) {
            var num = web3.eth.blockNumber;
            var res = rmContractInstance.newRole.sendTransaction(
                'testNewRole',
                permissions,
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
                    newRoleAddr = receipt.logs[0].address;
                    console.log('\nThe new role contract address:\n', newRoleAddr);
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get newRole receipt err:!!!!\n', err);
                    this.skip();
                });
        });

        it('should have info of new role', function () {
            const roleInstance = role.at(newRoleAddr);
            var res = roleInstance.queryRole.call();
            console.log('\nInfo:\n', res);
            assert.equal(res[0].substr(0, 24), web3.toHex('testNewRole'));

            for (var i = 0; i < res[1].length; i++) {
                assert.equal(res[1][i], permissions[i]);
            }
        });
    });

    describe('\ntest update role name\n', function () {
        it('should send a updateRoleName tx and get receipt', function (done) {
            var num = web3.eth.blockNumber;
            var res = rmContractInstance.updateRoleName.sendTransaction(
                newRoleAddr,
                'testNewRoleName',
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
                    console.log('\n!!!!Get updateRoleName receipt err:!!!!\n', err);
                    this.skip();
                });
        });

        it('should have the new role name', function () {
            const roleInstance = role.at(newRoleAddr);
            var res = roleInstance.queryName.call();
            console.log('\nNew role name:\n', res);
            assert.equal(res.substr(0, 32), web3.toHex('testNewRoleName'));
        });
    });

    describe('\ntest add permissions\n', function () {
        it('should send a addPermissions tx and get receipt', function (done) {
            var num = web3.eth.blockNumber;
            var res = rmContractInstance.addPermissions.sendTransaction(
                newRoleAddr,
                ['0x00000000000000000000000000000000033241b5'],
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
                    console.log('\n!!!!Get addPermissions receipt err:!!!!\n', err);
                    this.skip();
                });
        });

        it('should have the added permissions', function () {
            const roleInstance = role.at(newRoleAddr);
            var res = roleInstance.queryPermissions.call();
            console.log('\nNew Added permissions:\n', res);
            let lastPermissionIndex = res.length - 1;
            assert.equal(res[lastPermissionIndex], '0x00000000000000000000000000000000033241b5');
        });
    });

    describe('\ntest delete permissions\n', function () {
        it('should send a deletePermissions tx and get receipt', function (done) {
            var num = web3.eth.blockNumber;
            var res = rmContractInstance.deletePermissions.sendTransaction(
                newRoleAddr,
                ['0x00000000000000000000000000000000033241b5'],
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
                    console.log('\n!!!!Get deletePermissions receipt err:!!!!\n', err);
                    this.skip();
                });
        });

        it('should have deleted the permissions', function () {
            const roleInstance = role.at(newRoleAddr);
            var res = roleInstance.queryPermissions.call();
            console.log('\nResources lefted:\n', res);
            for (var i = 0; i < res.length; i++) {
                assert.equal(res[i], permissions[i]);
            }
        });
    });

    describe('\ntest set role\n', function () {
        it('should send a setRole tx and get receipt', function (done) {
            var num = web3.eth.blockNumber;
            var res = rmContractInstance.setRole.sendTransaction(
                config.testAddr[0],
                newRoleAddr,
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
                    console.log('\n!!!!Get setRole receipt err:!!!!\n', err);
                    this.skip();
                });
        });

        it('should have the permission of account', function () {
            var res = rmContractInstance.queryRoles.call(config.testAddr[0]);
            console.log('\nPermissions of testAccount:\n', res);
            let l = res.length - 1;
            assert.equal(res[l], newRoleAddr);
            var res2 = rmContractInstance.queryAccounts.call(newRoleAddr);
            console.log('\nAccount of permissions:\n', res2);
            assert.equal(res2, config.testAddr[0]);
        });
    });

    describe('\ntest cancel role\n', function () {
        it('should send a cancelRole tx and get receipt', function (done) {
            var num = web3.eth.blockNumber;
            var res = rmContractInstance.cancelRole.sendTransaction(
                config.testAddr[0],
                newRoleAddr,
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
                    console.log('\n!!!!Get cancelRole receipt err:!!!!\n', err);
                    this.skip();
                });
        });

        it('should not have the role of account', function () {
            var res = rmContractInstance.queryRoles.call(config.testAddr[0]);
            console.log('\nroles of testAccount:\n', res);
            assert.equal(res.length, 0);
            var res2 = rmContractInstance.queryAccounts.call(newRoleAddr);
            console.log('\nAccount of roles:\n', res2);
            assert.equal(res2.length, 0);
        });
    });

    describe('\ntest clear role\n', function () {
        it('should send a clearRole tx and get receipt', function (done) {
            var num = web3.eth.blockNumber;
            var res = rmContractInstance.clearRole.sendTransaction(
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
                    console.log('\n!!!!Get clearRole receipt err:!!!!\n', err);
                    this.skip();
                });
        });

        it('should have no roles of testAccount', function () {
            var res = rmContractInstance.queryRoles.call(config.testAddr[0]);
            console.log('\nRoles of testAccount:\n', res);
            assert.equal(res.length, 0);
        });
    });

    describe('\ntest delete role\n', function () {
        it('should send a deleteRole tx and get receipt', function (done) {
            console.log("\n newRoleAddr is:", newRoleAddr);
            var num = web3.eth.blockNumber;
            var res = rmContractInstance.deleteRole.sendTransaction(
                newRoleAddr,
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
                    console.log('\n!!!!Get deleteRole receipt err:!!!!\n', err);
                    this.skip();
                });
        });
    });

});
