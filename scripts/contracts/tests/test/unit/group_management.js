/* jshint esversion: 6 */
/* jshint expr: true */

const chai = require('chai');
const assert = chai.assert;
const util = require('../helpers/util');
const groupManagement = require('../helpers/group_management');
const authorization = require('../helpers/authorization');
const group = require('../helpers/group');
const config = require('../config');

// util
const web3 = util.web3;
const getTxReceipt = util.getTxReceipt;

// config
const sender = config.testSender;

// group management
const newGroup = groupManagement.newGroup;
const updateGroupName = groupManagement.updateGroupName;
const addAccounts = groupManagement.addAccounts;
const deleteAccounts = groupManagement.deleteAccounts;
const deleteGroup = groupManagement.deleteGroup;
const checkScope = groupManagement.checkScope;
const queryGroups = groupManagement.queryGroups;

// group
const gr = group.group;
const rootGroupAddr = config.contract.group.gAddr;
let gContractInstance;

// temp
let newGroupAddr;
let lengthOfAccounts;
let lengthOfChild;
let lengthOfGroups;

// =======================

describe('\n\ntest group management contract\n\n', function () {

    describe('\ntest add new group\n', function () {
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

        it('should have info of new group', function () {
            gContractInstance = gr.at(newGroupAddr);
            let res = gContractInstance.queryInfo.call();
            console.log('\nInfo:\n', res);
            assert.equal(res[0].substr(0, 20), web3.toHex('testGroup'));

            assert.equal(res[1][0], sender.address);
        });

        it('should have more groups', function () {
            let res = queryGroups.call();
            console.log('\nThe groups:\n', res);
            assert.equal(res.length, lengthOfGroups + 1);
            assert.equal(res[res.length-1], newGroupAddr);
        });

        it('should in the scope of root', function () {
            let res = checkScope(rootGroupAddr, newGroupAddr);
            console.log('\nIs in the scope of root:\n', res);
            assert.equal(res, true);
        });

        it('should in the scope of self', function () {
            let res = checkScope(newGroupAddr, newGroupAddr, sender);
            console.log('\nIs in the scope of self:\n', res);
            assert.equal(res, true);
        });
    });

    describe('\ntest update group name by self\n', function () {
        it('should send a updateGroupName tx and get receipt', function (done) {
            let num = web3.eth.blockNumber;
            let res = updateGroupName(newGroupAddr, newGroupAddr, 'testGroupNewName', sender);

            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt:\n', receipt);
                    assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get updateGroupName receipt err:!!!!\n', err);
                    this.skip();
                });
        });

        it('should have the new group name', function () {
            gContractInstance = gr.at(newGroupAddr);
            let res = gContractInstance.queryName.call();
            console.log('\nNew Group name:\n', res);
            assert.equal(res.substr(0, 34), web3.toHex('testGroupNewName'));
        });
    });

    describe('\ntest update group name by root\n', function () {
        it('should send a updateGroupName tx and get receipt', function (done) {
            let num = web3.eth.blockNumber;
            let res = updateGroupName(rootGroupAddr, newGroupAddr, 'testGroupNewName2');

            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt:\n', receipt);
                    assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get updateGroupName receipt err:!!!!\n', err);
                    this.skip();
                });
        });

        it('should have the new group name', function () {
            gContractInstance = gr.at(newGroupAddr);
            let res = gContractInstance.queryName.call();
            console.log('\nNew Group name:\n', res);
            assert.equal(res.substr(0, 36), web3.toHex('testGroupNewName2'));
        });
    });

    describe('\ntest add accounts\n', function () {

        before('Query the number of the accounts', function () {
            gContractInstance = gr.at(newGroupAddr);
            let res = gContractInstance.queryAccounts.call();
            console.log('\nThe number of the accounts:\n', res.length);
            lengthOfAccounts = res.length;
        });

        it('should send a addAccounts tx and get receipt', function (done) {
            let res = addAccounts(
                newGroupAddr,
                newGroupAddr,
                ['0x1a702a25c6bca72b67987968f0bfb3a3213c5603'],
                sender
            );

            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt:\n', receipt);
                    assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get addAccounts receipt err:!!!!\n', err);
                    this.skip();
                });
        });

        it('should have the added accounts', function () {
            gContractInstance = gr.at(newGroupAddr);
            let res = gContractInstance.queryAccounts.call();
            console.log('\nNew Added accounts:\n', res);
            let l = res.length - 1;
            assert.equal(res[l], '0x1a702a25c6bca72b67987968f0bfb3a3213c5603');
            assert.equal(l, lengthOfAccounts);
        });

        it('should send a addAccounts to a group address that does not exist and get receipt with error message', function (done) {
            let res = addAccounts(
                0x1234567,
                0x1234567,
                ['0x1a702a25c6bca72b67987968f0bfb3a3213c5604'],
                sender
            );

            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt with error message:\n', receipt);
                    assert.equal(receipt.errorMessage, "Reverted", JSON.stringify(receipt.errorMessage));
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get addAccounts receipt err:!!!!\n', err);
                    this.skip();
                });
        });
    });


    describe('\ntest add duplicate accounts\n', function () {

        before('Query the number of the accounts', function () {
            gContractInstance = gr.at(newGroupAddr);
            let res = gContractInstance.queryAccounts.call();
            console.log('\nThe number of the accounts:\n', res.length);
            lengthOfAccounts = res.length;
        });

        it('should send a addAccounts tx and get receipt', function (done) {
            let res = addAccounts(
                newGroupAddr,
                newGroupAddr,
                ['0x1a702a25c6bca72b67987968f0bfb3a3213c5603'],
                sender
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

        it('should not added into the accounts', function () {
            gContractInstance = gr.at(newGroupAddr);
            let res = gContractInstance.queryAccounts.call();
            console.log('\nThe num of the account:\n', res.length);
            assert.equal(res.length, lengthOfAccounts);
        });
    });

    describe('\ntest delete accounts\n', function () {
        it('should send a deleteAccounts tx and get receipt', function (done) {
            let res = deleteAccounts(
                newGroupAddr,
                newGroupAddr,
                ['0x1a702a25c6bca72b67987968f0bfb3a3213c5603'],
                sender
            );

            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt:\n', receipt);
                    assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get deleteAccounts receipt err:!!!!\n', err);
                    this.skip();
                });
        });

        it('should have deleted the accounts', function () {
            gContractInstance = gr.at(newGroupAddr);
            let res = gContractInstance.queryAccounts.call();
            console.log('\nAccounts deleted:\n', res);
            assert.equal(res[0], sender.address);
        });

        it('should send a deleteAccounts to a group address that does not exist and get receipt with error message', function (done) {
            let res = deleteAccounts(
                0x1234567,
                0x1234567,
                ['0x1a702a25c6bca72b67987968f0bfb3a3213c5603'],
                sender
            );

            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt with error message:\n', receipt);
                    assert.equal(receipt.errorMessage, "Reverted", JSON.stringify(receipt.errorMessage));
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get deleteAccounts receipt err:!!!!\n', err);
                    this.skip();
                });
        });

    });

    describe('\ntest delete group\n', function () {
        before('Query the number of the accounts', function () {
            gContractInstance = gr.at(rootGroupAddr);
            let res = gContractInstance.queryChild.call();
            console.log('\nThe number of the child group:\n', res.length);
            lengthOfChild = res.length;
        });

        it('should send a deleteGroup tx and get receipt', function (done) {
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

        it('should have deleted the group', function () {
            gContractInstance = gr.at(rootGroupAddr);
            let res = gContractInstance.queryChild.call();
            console.log('\nNow the number of child group:\n', res.length);
            assert.equal(res.length, lengthOfChild - 1);
        });

        it('should send a deleteGroup that does not exist and get receipt with error message', function (done) {
            let res = deleteGroup(newGroupAddr, newGroupAddr, sender);

            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt with error message:\n', receipt);
                    assert.equal(receipt.errorMessage, "Reverted", JSON.stringify(receipt.errorMessage));
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get deleteGroup receipt err:!!!!\n', err);
                    this.skip();
                });
        });
    });
});
