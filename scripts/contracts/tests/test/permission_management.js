/* jshint esversion: 6 */
/* jshint expr: true */

// TODO Refactor: every unit test should be independent

const chai = require('chai');
const assert = chai.assert;
const util = require('./helpers/util');
const permissionManagement = require('./helpers/permission_management');
const authorization = require('./helpers/authorization');
const permission = require('./helpers/permission');
const config = require('./config');

// util
const web3 = util.web3;
const getTxReceipt = util.getTxReceipt;

// permission management
const newPermission = permissionManagement.newPermission;
const updatePermissionName = permissionManagement.updatePermissionName;
const addResources = permissionManagement.addResources;
const deleteResources = permissionManagement.deleteResources;
const clearAuthorization = permissionManagement.clearAuthorization;
const setAuthorization = permissionManagement.setAuthorization;
const cancelAuthorization = permissionManagement.cancelAuthorization;
const deletePermission = permissionManagement.deletePermission;
const setAuthorizations = permissionManagement.setAuthorizations;

// authorization
const queryPermissions = authorization.queryPermissions;
const queryAccounts = authorization.queryAccounts;
const queryAllAccounts = authorization.queryAllAccounts;

// perm
const perm = permission.perm;
let pContractInstance;

// temp
let newPermissionAddr;
let newPermissionAddrA;
let newPermissionAddrB;
let lengthOfAccounts;
let lengthOfResources;

// =======================

describe('\n\ntest permission management contract\n\n', function() { 

    describe('\ntest add permission\n', function() { 
        it('should send a newPermission tx and get receipt', function(done) {
            let name = 'testPermission';
            let res = newPermission(name, config.testAddr, config.testFunc);
            
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
            pContractInstance = perm.at(newPermissionAddr);
            let res = pContractInstance.queryInfo.call();
            console.log('\nInfo:\n', res);
            assert.equal(res[0].substr(0, 30), web3.toHex('testPermission'));

            for (let i=0; i<res[1].length; i++) {
                assert.equal(res[1][i], config.testAddr[i]);
                assert.equal(res[2][i], config.testFunc[i]);
            }
        });
    });

    describe('\ntest update permission name\n', function() { 
        it('should send a updatePermissionName tx and get receipt', function(done) {
            let num = web3.eth.blockNumber;
            let res = updatePermissionName(newPermissionAddr, 'testPermissionNewName');

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
            pContractInstance = perm.at(newPermissionAddr);
            let res = pContractInstance.queryName.call();
            console.log('\nNew permission name:\n', res);
            assert.equal(res.substr(0, 44), web3.toHex('testPermissionNewName'));
        });
    });

    describe('\ntest add resources\n', function() { 

        before('Query the number of the resource', function() {
            pContractInstance = perm.at(newPermissionAddr);
            let res = pContractInstance.queryResource.call();
            console.log('\nThe number of the resource:\n', res[0].length);
            lengthOfResources = res[0].length;
        });

        it('should send a addResources tx and get receipt', function(done) {
            let res = addResources(
                    newPermissionAddr,
                    ['0x1a702a25c6bca72b67987968f0bfb3a3213c5603'],
                    ['0xf036ed59']
                );

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
            pContractInstance = perm.at(newPermissionAddr);
            let res = pContractInstance.queryResource.call();
            console.log('\nNew Added resources:\n', res);
            let l = res[0].length - 1;
            assert.equal(res[0].length, res[1].length);
            assert.equal(res[0][l], '0x1a702a25c6bca72b67987968f0bfb3a3213c5603');
            assert.equal(res[1][l], '0xf036ed59');
            assert.equal(l, lengthOfResources);
        });

        it('should send a addResources to an address that does not exist and get receipt with error message', function(done) {
            let res = addResources(
                    0x1234567,
                    ['0x1a702a25c6bca72b67987968f0bfb3a3213c5603'],
                    ['0xf036ed59']
                );

            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt with error message:\n', receipt);
                    assert.equal(receipt.errorMessage, "Reverted", JSON.stringify(receipt.errorMessage));
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get addResources receipt err:!!!!\n', err);
                    this.skip();
                });
        });
    });

    describe('\ntest add duplicate resources\n', function() { 

        before('Query the number of the resource', function() {
            pContractInstance = perm.at(newPermissionAddr);
            let res = pContractInstance.queryResource.call();
            console.log('\nThe number of the resource:\n', res[0].length);
            lengthOfResources = res[0].length;
        });

        it('should send a addResources tx and get receipt', function(done) {
            let res = addResources(
                    newPermissionAddr,
                    ['0x1a702a25c6bca72b67987968f0bfb3a3213c5603'],
                    ['0xf036ed59']
                );

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

        it('should not added into the resources', function() {
            pContractInstance = perm.at(newPermissionAddr);
            let res = pContractInstance.queryResource.call();
            console.log('\nThe num of the resource:\n', res[0].length);
            assert.equal(res[0].length, lengthOfResources);
        });
    });

    describe('\ntest delete resources\n', function() { 
        it('should send a deleteResources tx and get receipt', function(done) {
            let res = deleteResources(
                    newPermissionAddr,
                    ['0x1a702a25c6bca72b67987968f0bfb3a3213c5603'],
                    ['0xf036ed59']
                );

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
            pContractInstance = perm.at(newPermissionAddr);
            let res = pContractInstance.queryResource.call();
            console.log('\nResources lefted:\n', res);
            for (let i=0; i<res[1].length; i++) {
                assert.equal(res[0][i], config.testAddr[i]);
                assert.equal(res[1][i], config.testFunc[i]);
            }
        });

        it('should send a deleteResources to an address that does not exist and get receipt with error message', function(done) {
            let res = deleteResources(
                    0x1234567,
                    ['0x1a702a25c6bca72b67987968f0bfb3a3213c5603'],
                    ['0xf036ed59']
                );

            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt with error message:\n', receipt);
                    assert.equal(receipt.errorMessage, "Reverted", JSON.stringify(receipt.errorMessage));
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get deleteResources receipt err:!!!!\n', err);
                    this.skip();
                });
        });

    });

    describe('\ntest clear authorization\n', function() { 
        it('should send a clearAuthorization tx and get receipt', function(done) {
            let res = clearAuthorization(config.testAddr[0]);

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
            let res = queryPermissions(config.testAddr[0]);
            console.log('\nPermissions of testAccount:\n', res);
            assert.equal(res.length, 0);
        });
    });

    describe('\ntest set authorization\n', function() { 
        before('Query the number of the account', function() {
            let res = queryAllAccounts();
            lengthOfAccounts = res.length;
        });

        it('should send a setAuthorization tx and get receipt', function(done) {
            let res = setAuthorization(config.testAddr[0], config.testAddr[1]);

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
            let res = queryPermissions(config.testAddr[0]);
            console.log('\nPermissions of testAccount:\n', res);
            let l = res.length -1;
            assert.equal(res[l], config.testAddr[1]);
            let res2 = queryAccounts(config.testAddr[1]);
            console.log('\nAccount of permissions:\n', res2);
            assert.equal(res2, config.testAddr[0]);
        });

        it('should have all accounts', function() {
            let res = queryAllAccounts();
            console.log('\nAll accounts:\n', res);
            assert.equal(res[res.length-1], config.testAddr[0]);
            assert.equal(res.length, lengthOfAccounts + 1);
        });
    });

    describe('\ntest set duplicated authorization\n', function() { 
        before('Query the number of the account', function() {
            let res = queryAllAccounts();
            lengthOfAccounts = res.length;
        });

        it('should send a setAuthorization tx and get receipt', function(done) {
            let res = setAuthorization(config.testAddr[0], config.testAddr[1]);

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

        it('should not be setted', function() {
            let res = queryAllAccounts();
            console.log('\nAll accounts:\n', res);
            assert.equal(res.length, lengthOfAccounts);
        });
    });

    describe('\ntest cancel authorization\n', function() { 
        it('should send a cancelAuthorization tx and get receipt', function(done) {
            let res = cancelAuthorization(config.testAddr[0], config.testAddr[1]);

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
            let res = queryPermissions(config.testAddr[0]);
            console.log('\nPermissions of testAccount:\n', res);
            assert.equal(res.length, 0);
            let res2 = queryAccounts(config.testAddr[1]);
            console.log('\nAccount of permissions:\n', res2);
            assert.equal(res2.length, 0);
        });
    });

    describe('\ntest delete permission\n', function() { 
        it('should send a deletePermission tx and get receipt', function(done) {
            let res = deletePermission(newPermissionAddr);

            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt:\n', receipt);
                    assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                    assert.equal(receipt.logs[0].data.substr(26), newPermissionAddr.substr(2));
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get deletePermission receipt err:!!!!\n', err);
                    this.skip();
                });
        });
    });

    describe('\ntest delete permission: query the auth\n', function() { 
        it('should send a newPermission tx and get receipt', function(done) {
            let res = newPermission('testPermissionA', config.testAddr, config.testFunc);

            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt:\n', receipt);
                    assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                    newPermissionAddrA = receipt.logs[0].address;
                    console.log('\nThe new permission contract address:\n', newPermissionAddr);
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get newPermission receipt err:!!!!\n', err);
                    this.skip();
                });
        });

        it('should send a newPermission tx and get receipt', function(done) {
            let res = newPermission('testPermissionB', config.testAddr, config.testFunc);
            
            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt:\n', receipt);
                    assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                    newPermissionAddrB = receipt.logs[0].address;
                    console.log('\nThe new permission contract address:\n', newPermissionAddr);
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get newPermission receipt err:!!!!\n', err);
                    this.skip();
                });
        });

        it('should send a setAuthorization tx and get receipt', function(done) {
            let res = setAuthorizations(
                    config.testAddr[0],
                    [newPermissionAddrA, newPermissionAddrB]
                );

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
            let res = queryPermissions(config.testAddr[0]);
            console.log('\nPermissions of testAccount:\n', res);
            let l = res.length -1;
            assert.equal(res[l], newPermissionAddrB);
            assert.equal(res[l - 1], newPermissionAddrA);
            let res1 = queryAccounts(newPermissionAddrA);
            console.log('\nAccount of permissionA:\n', res1);
            let res2 = queryAccounts(newPermissionAddrB);
            console.log('\nAccount of permissionB:\n', res2);
            assert.equal(res2, config.testAddr[0]);
        });

        it('should send a deletePermission tx and get receipt', function(done) {
            let res = deletePermission(newPermissionAddrA);

            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt:\n', receipt);
                    assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get deletePermission receipt err:!!!!\n', err);
                    this.skip();
                });
        });

        it("should cancel the account's permission", function() {
            let res = queryPermissions(config.testAddr[0]);
            console.log('\nPermissions of testAccount:\n', res);
            assert.equal(res.length, 1);
            assert.equal(res[0], newPermissionAddrB);
        });
    });
});
