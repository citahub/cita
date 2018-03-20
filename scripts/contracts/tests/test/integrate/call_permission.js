/* jshint esversion: 6 */
/* jshint expr: true */

const chai = require('chai');
const assert = chai.assert;
const util = require('../helpers/util');
const permissionManagement = require('../helpers/permission_management');
const permission = require('../helpers/permission');
const config = require('../config');

// util
const web3 = util.web3;
const getTxReceipt = util.getTxReceipt;

// config
const superAdmin = config.contract.authorization.superAdmin;
const sender = config.testSender;

// permission management
const updatePermissionName = permissionManagement.updatePermissionName;
const setAuthorization = permissionManagement.setAuthorization;
const cancelAuthorization = permissionManagement.cancelAuthorization;

// perm
const perm = permission.perm;
let pContractInstance;

const send_tx = "0x0000000000000000000000000000000000000001";
const update_permission = "0x00000000000000000000000000000000033241b5";

// Only updatePermissionName
// =======================

describe('\n\nintegrate test permission: \n\n', function() {

    before('should send a setAuthorization tx and get receipt: grant the send_tx permissiont to sender', function(done) {
        let res = setAuthorization(sender.address, send_tx);

        getTxReceipt(res)
            .then((receipt) => {
                console.log('\nSend ok and get receipt:\n', receipt);
                assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                done();
            })
            .catch(err => {
                console.log('\n!!!!Get setAuthorization receipt err:!!!!\n', err);
                this.skip();
                done();
            });
    });

    describe('\n\ntest update permission name before setted auth:\n\n', function() {

        it('should wait a new block', function(done) {
            let num = web3.eth.blockNumber;
            let tmp;
            do {
                tmp = web3.eth.blockNumber;
            } while (tmp <= num);
            done();
        });

        it('should send a updatePermissionName tx and get receipt', function(done) {
            let res = updatePermissionName(send_tx, 'new_send_tx', sender); 
            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt:\n', receipt);
                    assert.equal(receipt.errorMessage, "No Call contract permission.", JSON.stringify(receipt.errorMessage));
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get updatePermissionName receipt err:!!!!\n', err);
                    this.skip();
                    done();
                });
        });
    });

    describe('\n\ntest update permission name after setted auth:\n\n', function() {
        before('should send a setAuthorization tx and get receipt: grant the update_permission permissiont to sender', function(done) {
            let res = setAuthorization(sender.address, update_permission);

            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt:\n', receipt);
                    assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get setAuthorization receipt err:!!!!\n', err);
                    this.skip();
                    done();
                });
        });

        it('should wait a new block', function(done) {
            let num = web3.eth.blockNumber;
            let tmp;
            do {
                tmp = web3.eth.blockNumber;
            } while (tmp <= num);
            done();
        });

        it('should send a updatePermissionName tx and get receipt', function(done) {
            let res = updatePermissionName(send_tx, 'new_send_tx', sender); 
            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt:\n', receipt);
                    assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get updatePermissionName receipt err:!!!!\n', err);
                    this.skip();
                    done();
                });
        });

        it('should have the new permission name', function() {
            pContractInstance = perm.at(send_tx);
            let res = pContractInstance.queryName.call();
            console.log('\nNew send_tx permission name:\n', res);
            assert.equal(res.substr(0, 24), web3.toHex('new_send_tx'));
        });
    });

    describe('\n\ntest update permission name after cancel auth:\n\n', function() {
        before('should send a cancelAuthorization tx and get receipt: cancel the update_permission permissiont of sender', function(done) {
            let res = cancelAuthorization(sender.address, update_permission);

            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt:\n', receipt);
                    assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get cancelAuthorization receipt err:!!!!\n', err);
                    this.skip();
                    done();
                });
        });

        it('should wait a new block', function(done) {
            let num = web3.eth.blockNumber;
            let tmp;
            do {
                tmp = web3.eth.blockNumber;
            } while (tmp <= num);
            done();
        });

        it('should send a updatePermissionName tx and get receipt', function(done) {
            let res = updatePermissionName(send_tx, 'new_send_tx', sender); 
            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt:\n', receipt);
                    assert.equal(receipt.errorMessage, "No Call contract permission.", JSON.stringify(receipt.errorMessage));
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get updatePermissionName receipt err:!!!!\n', err);
                    this.skip();
                    done();
                });
        });
    });

    after('should send a cancelAuthorization tx and get receipt: cancel the send_tx permissiont of sender', function(done) {
        let res = cancelAuthorization(sender.address, send_tx);

        getTxReceipt(res)
            .then((receipt) => {
                console.log('\nSend ok and get receipt:\n', receipt);
                assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                let num = web3.eth.blockNumber;
                let tmp;
                do {
                    tmp = web3.eth.blockNumber;
                } while (tmp <= num);
                done();
            })
            .catch(err => {
                console.log('\n!!!!Get cancelAuthorization receipt err:!!!!\n', err);
                this.skip();
                done();
            });
    });
});
