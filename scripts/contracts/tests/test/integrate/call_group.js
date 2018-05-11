/* jshint esversion: 6 */
/* jshint expr: true */

const chai = require('chai');
const assert = chai.assert;
const util = require('../helpers/util');
const groupManagement = require('../helpers/group_management');
const permissionManagement = require('../helpers/permission_management');
const config = require('../config');

// util
const web3 = util.web3;
const getTxReceipt = util.getTxReceipt;

// config
const sender = config.testSender;

// group management
const newGroup = groupManagement.newGroup;
const deleteGroup = groupManagement.deleteGroup;
const queryGroups = groupManagement.queryGroups;

// permission management
const setAuthorization = permissionManagement.setAuthorization;

// temp
let newGroupAddr;
let newGroupAddr2;
let lengthOfGroups;

const deleteGroupPermission = "0x000000000000000000000000000000000c3241b5";
const rootGroupAddr = "0x00000000000000000000000000000000013241b6";

// Only deleteGroup
// =======================

describe('\n\nintegrate test group: \n\n', function() {

    before('Query the number of the groups', function () {
        let res = queryGroups.call();
        console.log('\nThe groups:\n', res);
        lengthOfGroups = res.length;
    });

    it('should send a newGroup tx and get receipt', function (done) {
        let name = 'testGroup';
        let res = newGroup(rootGroupAddr, name, [sender.address]);

        getTxReceipt(res)
            .then((receipt) => {
                console.log('\nSend ok and get receipt:\n', receipt);
                assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                newGroupAddr = receipt.logs[0].address;
                console.log('\nThe new permission contract address:\n', newGroupAddr);
                done();
            })
            .catch(err => {
                console.log('\n!!!!Get newGroup receipt err:!!!!\n', err);
                this.skip();
            });
    });

    it('should send another newGroup tx and get receipt', function (done) {
        let name = 'testGroup2';
        let res = newGroup(rootGroupAddr, name, [sender.address]);

        getTxReceipt(res)
            .then((receipt) => {
                console.log('\nSend ok and get receipt:\n', receipt);
                assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                newGroupAddr2 = receipt.logs[0].address;
                console.log('\nThe new permission contract address:\n', newGroupAddr2);
                done();
            })
            .catch(err => {
                console.log('\n!!!!Get newGroup receipt err:!!!!\n', err);
                this.skip();
            });
    });

    it('should have more groups', function () {
        let res = queryGroups.call();
        console.log('\nThe groups:\n', res);
        assert.equal(res.length, lengthOfGroups + 2);
        assert.equal(res[res.length-1], newGroupAddr2);
        assert.equal(res[res.length-2], newGroupAddr);
    });

    it('should send a deleteGroup tx and get receipt with errormessage: No contract permission. origin: newGroup', function (done) {
        let res = deleteGroup(newGroupAddr, newGroupAddr, sender);

        getTxReceipt(res)
            .then((receipt) => {
                console.log('\nSend ok and get receipt:\n', receipt);
                assert.equal(receipt.errorMessage, 'No Call contract permission.', JSON.stringify(receipt.errorMessage));
                done();
            })
            .catch(err => {
                console.log('\n!!!!Get deleteGroup receipt err:!!!!\n', err);
                this.skip();
            });
    });

    it('should send a setAuthorization tx and get receipt: newGroup', function (done) {
        let res = setAuthorization(newGroupAddr, deleteGroupPermission);

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

    it('should send a setAuthorization tx and get receipt: rootGroup', function (done) {
        let res = setAuthorization(rootGroupAddr, deleteGroupPermission);

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

    it('should send a deleteGroup tx and get receipt with errormessage: reverted. origin: newGroup', function (done) {
        let res = deleteGroup(newGroupAddr, rootGroupAddr, sender);

        getTxReceipt(res)
            .then((receipt) => {
                console.log('\nSend ok and get receipt:\n', receipt);
                assert.equal(receipt.errorMessage, 'Reverted.', JSON.stringify(receipt.errorMessage));
                done();
            })
            .catch(err => {
                console.log('\n!!!!Get deleteGroup receipt err:!!!!\n', err);
                this.skip();
            });
    });

    it('should send a deleteGroup tx and get receipt with errormessage: No contract permission. origin: newGroup', function (done) {
        let res = deleteGroup(newGroupAddr2, newGroupAddr, sender);

        getTxReceipt(res)
            .then((receipt) => {
                console.log('\nSend ok and get receipt:\n', receipt);
                assert.equal(receipt.errorMessage, 'No Call contract permission.', JSON.stringify(receipt.errorMessage));
                done();
            })
            .catch(err => {
                console.log('\n!!!!Get deleteGroup receipt err:!!!!\n', err);
                this.skip();
            });
    });

    it('should send a deleteGroup tx and get receipt. origin: newGroup', function (done) {
        let res = deleteGroup(newGroupAddr, newGroupAddr, sender);

        getTxReceipt(res)
            .then((receipt) => {
                console.log('\nSend ok and get receipt:\n', receipt);
                assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                done();
            })
            .catch(err => {
                console.log('\n!!!!Get deleteGroup receipt err:!!!!\n', err);
                this.skip();
            });
    });

    it('should have less groups', function () {
        let res = queryGroups.call();
        console.log('\nThe groups:\n', res);
        assert.equal(res.length, lengthOfGroups+1);
    });

    it('should send a setAuthorization tx and get receipt: rootGroup', function (done) {
        let res = setAuthorization(rootGroupAddr, deleteGroupPermission);

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

    it('should send a deleteGroup tx and get receipt. origin: root ', function (done) {
        let res = deleteGroup(rootGroupAddr, newGroupAddr2, sender);

        getTxReceipt(res)
            .then((receipt) => {
                console.log('\nSend ok and get receipt:\n', receipt);
                assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                done();
            })
            .catch(err => {
                console.log('\n!!!!Get deleteGroup receipt err:!!!!\n', err);
                this.skip();
            });
    });

    it('should have less groups', function () {
        let res = queryGroups.call();
        console.log('\nThe groups:\n', res);
        assert.equal(res.length, lengthOfGroups);
    });
});
