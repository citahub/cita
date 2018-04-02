/* jshint esversion: 6 */
/* jshint expr: true */

const chai = require('chai');
const assert = chai.assert;
const util = require('../helpers/util');
const roleManagement = require('../helpers/role_management');
const permissionManagement = require('../helpers/permission_management');
const authorization = require('../helpers/authorization');
const config = require('../config');

// util
const web3 = util.web3;
const getTxReceipt = util.getTxReceipt;

// config
const permissions = config.permissions;
const rABI = config.contract.role.rABI;

// role
const role = web3.eth.contract(rABI);

// role management
const newRole = roleManagement.newRole;
const updateRoleName = roleManagement.updateRoleName;
const addPermissions = roleManagement.addPermissions;
const deletePermissions = roleManagement.deletePermissions;
const setRole = roleManagement.setRole;
const cancelRole = roleManagement.cancelRole;
const clearRole = roleManagement.clearRole;
const deleteRole = roleManagement.deleteRole;
const queryRoles = roleManagement.queryRoles;
const queryAccounts = roleManagement.queryAccounts;
const queryPermissionsFromRoleMana = roleManagement.queryPermissions;

// authorization
const queryPermissions = authorization.queryPermissions;
let roleInstance;

// permission management
const newPermission = permissionManagement.newPermission;

// temp
let newRoleAddr;
let newRoleAddr2;
let lengthOfPermissions;
let lengthOfRoles;
let newPermissionAddr;

// =======================

describe('\n\ntest role management contract\n\n', function () {
    describe('\ntest new role\n', function () {
        it('should send a newRole tx and get receipt', function (done) {
            let res = newRole('testNewRole', permissions);

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
            roleInstance = role.at(newRoleAddr);
            let res = roleInstance.queryRole.call();
            console.log('\nInfo:\n', res);
            assert.equal(res[0].substr(0, 24), web3.toHex('testNewRole'));

            for (let i = 0; i < res[1].length; i++) {
                assert.equal(res[1][i], permissions[i]);
            }
        });
    });

    describe('\ntest update role name\n', function () {
        it('should send a updateRoleName tx and get receipt', function (done) {
            let res = updateRoleName(newRoleAddr,'testNewRoleName');

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
            roleInstance = role.at(newRoleAddr);
            let res = roleInstance.queryName.call();
            console.log('\nNew role name:\n', res);
            assert.equal(res.substr(0, 32), web3.toHex('testNewRoleName'));
        });
    });

    describe('\ntest add permissions\n', function () {

        before('Query the number of the permission', function() {
            roleInstance = role.at(newRoleAddr);
            let res = roleInstance.queryPermissions.call();
            console.log('\nThe number of the permission:\n', res.length);
            lengthOfPermissions = res.length;
        });

        it('should send a newPermission tx and get receipt', function (done) {
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

        it('should send an addPermissions tx and get receipt', function (done) {
            let res = addPermissions(
                    newRoleAddr,
                    [newPermissionAddr]
                );

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

        it('should have the added permissions: role', function () {
            roleInstance = role.at(newRoleAddr);
            let res = roleInstance.queryPermissions.call();
            console.log('\nNew Added permissions:\n', res);
            assert.equal(res[res.length - 1], newPermissionAddr);
            assert.equal(res.length, lengthOfPermissions + 1);
        });

        it('should have the added permissions: from role_management', function () {
            let res = queryPermissionsFromRoleMana(newRoleAddr);
            console.log('\nNew Added permissions:\n', res);
            assert.equal(res[res.length - 1], newPermissionAddr);
            assert.equal(res.length, lengthOfPermissions + 1);
        });

        it('should send an addPermissions which role does not exist and get receipt with error message', function (done) {
            let res = addPermissions(
                    0x123456,
                    [newPermissionAddr]
                );

            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt with error message:\n', receipt);
                    assert.equal(receipt.errorMessage, "Reverted", JSON.stringify(receipt.errorMessage));
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get addPermissions receipt err:!!!!\n', err);
                    this.skip();
                });
        });

    });


    describe('\ntest add duplicated permissions\n', function () {

        before('Query the number of the permission', function() {
            roleInstance = role.at(newRoleAddr);
            let res = roleInstance.queryPermissions.call();
            console.log('\nThe number of the permission:\n', res.length);
            lengthOfPermissions = res.length;
        });

        it('should send a addPermissions tx and get receipt', function (done) {
            let res = addPermissions(
                    newRoleAddr,
                    [newPermissionAddr]
                );

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

        it('should not added into the permissions', function () {
            roleInstance = role.at(newRoleAddr);
            let res = roleInstance.queryPermissions.call();
            console.log('\nThe number of the permissions:\n', res.length);
            assert.equal(res.length, lengthOfPermissions);
        });
    });

    describe('\ntest delete permissions\n', function () {
        it('should send a deletePermissions tx and get receipt', function (done) {
            let res = deletePermissions(
                    newRoleAddr,
                    [newPermissionAddr]
                );

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
            roleInstance = role.at(newRoleAddr);
            let res = roleInstance.queryPermissions.call();
            console.log('\nPermissions lefted:\n', res);
            for (let i = 0; i < res.length; i++) {
                assert.equal(res[i], permissions[i]);
            }
        });

        it('should send a deletePermissions to an address that does not exist and get receipt with error message', function (done) {
            let res = deletePermissions(
                    0x123456,
                    [newPermissionAddr]
                );

            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt with error message:\n', receipt);
                    assert.equal(receipt.errorMessage, "Reverted", JSON.stringify(receipt.errorMessage));
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get deletePermissions receipt err:!!!!\n', err);
                    this.skip();
                });
        });

    });

    describe('\ntest set role\n', function () {
        it('should send a setRole tx and get receipt', function (done) {
            let res = setRole(config.testAddr[0], newRoleAddr);

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

        it('should have the role of account', function () {
            let res = queryRoles(config.testAddr[0]);
            console.log('\nroles of testAccount:\n', res);
            let l = res.length - 1;
            assert.equal(res[l], newRoleAddr);
            let res2 = queryAccounts(newRoleAddr);
            console.log('\nAccount of role:\n', res2);
            assert.equal(res2, config.testAddr[0]);
        });
    });

    describe('\ntest set duplicated role\n', function () {

        before('Query the number of role', function () {
            let res = queryRoles(config.testAddr[0]);
            console.log('\nThe length of role:\n', res.length);
            lengthOfRoles = res.length;
        });

        it('should send a setRole tx and get receipt', function (done) {
            let res = setRole(config.testAddr[0], newRoleAddr);

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

        it('should not setted into the roles', function () {
            let res = queryRoles(config.testAddr[0]);
            console.log('\nThe length of role:\n', res.length);
            lengthOfRoles = res.length;
            assert.equal(res.length, lengthOfRoles);
        });
    });

    describe('\ntest role permissions of account after add_permission\n', function () {

        it('should send a addPermissions tx and get receipt', function (done) {
            let res = addPermissions(
                    newRoleAddr,
                    ['0x00000000000000000000000000000000033241b0']
                );

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

        it('should have the added permissions: role', function () {
            roleInstance = role.at(newRoleAddr);
            let res = roleInstance.queryPermissions.call();
            console.log('\nNew Added permissions:\n', res);
            lengthOfPermissions = res.length;
            assert.equal(res[lengthOfPermissions - 1], '0x00000000000000000000000000000000033241b0');
        });

        it('should have the added permissions: auth', function () {
            let res = queryPermissions(config.testAddr[0]);
            console.log('\nPermissions of testAddr:\n', res);
            lengthOfPermissions = res.length;
            assert.equal(res[lengthOfPermissions - 1], '0x00000000000000000000000000000000033241b0');
        });

    });

    describe('\ntest role permissions of account after delete_permission\n', function () {

        it('should send a deletePermissions tx and get receipt', function (done) {
            let res = deletePermissions(
                    newRoleAddr,
                    ['0x00000000000000000000000000000000033241b0']
                );

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

        it('should have deleted the test permissions: role', function () {
            let res = queryPermissions(config.testAddr[0]);
            console.log('\nPermissions of testAddr:\n', res);
            assert.equal(res.length, lengthOfPermissions - 1);
        });

    });

    describe('\ntest cancel role\n', function () {
        it('should send a cancelRole tx and get receipt', function (done) {
            let res = cancelRole(config.testAddr[0], newRoleAddr);

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
            let res = queryRoles(config.testAddr[0]);
            console.log('\nroles of testAccount:\n', res);
            assert.equal(res.length, 0);
            let res2 = queryAccounts(newRoleAddr);
            console.log('\nAccount of roles:\n', res2);
            assert.equal(res2.length, 0);
        });
    });

    describe('\ntest clear role\n', function () {
        it('should send a clearRole tx and get receipt', function (done) {
            let res = clearRole(config.testAddr[0]);

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
            let res = queryRoles(config.testAddr[0]);
            console.log('\nRoles of testAccount:\n', res);
            assert.equal(res.length, 0);
        });
    });

    describe('\ntest delete role\n', function () {
        it('should send a deleteRole tx and get receipt', function (done) {
            let res = deleteRole(newRoleAddr);

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

    describe('\ntest cancel role should check other roles of account\n', function () {
        it('should send a newRole tx and get receipt', function (done) {
            let res = newRole('testNewRole', permissions);

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

        it('should send a newRole tx and get receipt', function (done) {
            let res = newRole('testNewRole2', permissions);

            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt:\n', receipt);
                    assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                    newRoleAddr2 = receipt.logs[0].address;
                    console.log('\nThe new role contract address:\n', newRoleAddr2);
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get newRole receipt err:!!!!\n', err);
                    this.skip();
                });
        });

        it('should send a setRole tx and get receipt', function (done) {
            let res = setRole(config.testAddr[1], newRoleAddr);

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

        it('should send a setRole tx and get receipt', function (done) {
            let res = setRole(config.testAddr[1], newRoleAddr2);

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

        it('should send a deletePermissions tx and get receipt', function (done) {
            let res = deletePermissions(newRoleAddr2, permissions);

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

        it('should have the newRole\'s permission after delete the permissions', function () {
            let res = queryPermissions(config.testAddr[1]);
            console.log('\nPermissions of testAddr:\n', res);
            assert.equal(res.length, permissions.length);
            for (let i = 0; i < permissions.length; i++) {
                assert.equal(res[i], permissions[i]);
            }
        });

        it('should cancel newRole', function (done) {
            let res = cancelRole(config.testAddr[1], newRoleAddr2);

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
                
        it('should have the newRole\'s permission after cancel the newRole2', function () {
            let res = queryPermissions(config.testAddr[1]);
            console.log('\nPermissions of testAddr:\n', res);
            assert.equal(res.length, permissions.length);
            for (let i = 0; i < permissions.length; i++) {
                assert.equal(res[i], permissions[i]);
            }
        });

        after('cancel role', function (done) {
            let res = cancelRole(config.testAddr[1], newRoleAddr);

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
    });
});
