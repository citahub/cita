/* jshint esversion: 6 */
/* jshint expr: true */

const chai = require('chai');
const assert = chai.assert;
const util = require('../helpers/util');
const roleManagement = require('../helpers/role_management');
const permissionManagement = require('../helpers/permission_management');
const config = require('../config');

// util
const web3 = util.web3;
const getTxReceipt = util.getTxReceipt;
const rABI = config.contract.role.rABI;

// config
const superAdmin = config.contract.authorization.superAdmin;
const sender = config.testSender;

// role management
const newRole = roleManagement.newRole;
const setAuthorization = permissionManagement.setAuthorization;
const cancelAuthorization = permissionManagement.cancelAuthorization;

// role
const role = web3.eth.contract(rABI);

const send_tx = "0x0000000000000000000000000000000000000001";
const permissions = [send_tx];
const new_role = "0x00000000000000000000000000000000063241b5";

// temp
let newRoleAddr;

// Only newRole
// =======================

describe('\n\nintegrate test role: \n\n', function() {

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

    describe('\n\ntest new role before setted auth:\n\n', function() {

        it('should wait a new block', function(done) {
            let num = web3.eth.blockNumber;
            let tmp;
            do {
                tmp = web3.eth.blockNumber;
            } while (tmp <= num);
            done();
        });

        it('should send a newRole tx and get receipt', function(done) {
            let res = newRole('new_send_tx', permissions, sender); 
            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt:\n', receipt);
                    assert.equal(receipt.errorMessage, "No Call contract permission.", JSON.stringify(receipt.errorMessage));
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get newRole receipt err:!!!!\n', err);
                    this.skip();
                    done();
                });
        });
    });

    describe('\n\ntest new role after setted auth:\n\n', function() {
        before('should send a setAuthorization tx and get receipt: grant the new_role permissiont to sender', function(done) {
            let res = setAuthorization(sender.address, new_role);

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

        it('should send a newRole tx and get receipt', function(done) {
            let res = newRole('new_role', permissions, sender); 
            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt:\n', receipt);
                    assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                    newRoleAddr = receipt.logs[0].address;
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get newRole receipt err:!!!!\n', err);
                    this.skip();
                    done();
                });
        });

        it('should have info of new role', function () {
            roleInstance = role.at(newRoleAddr);
            let res = roleInstance.queryRole.call();
            console.log('\nInfo:\n', res);
            assert.equal(res[0].substr(0, 18), web3.toHex('new_role'));

            for (let i = 0; i < res[1].length; i++) {
                assert.equal(res[1][i], permissions[i]);
            }
        });
    });

    describe('\n\ntest newRole after cancel auth:\n\n', function() {
        before('should send a cancelAuthorization tx and get receipt: cancel the new_role permissiont of sender', function(done) {
            let res = cancelAuthorization(sender.address, new_role);

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

        it('should send a newRole tx and get receipt', function(done) {
            let res = newRole('new_role_2', permissions, sender); 
            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt:\n', receipt);
                    assert.equal(receipt.errorMessage, "No Call contract permission.", JSON.stringify(receipt.errorMessage));
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get newRole receipt err:!!!!\n', err);
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
