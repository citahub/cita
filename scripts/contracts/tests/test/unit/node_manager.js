/* jshint esversion: 6 */
/* jshint expr: true */

const chai = require('chai');
const assert = chai.assert;
const util = require('../helpers/util');
const nodeManager = require('../helpers/node_manager');
const config = require('../config');

// util
const web3 = util.web3;
const getTxReceipt = util.getTxReceipt;

// node_manager
const addAdmin = nodeManager.addAdmin;
const newNode = nodeManager.newNode;
const approveNode = nodeManager.approveNode;
const deleteNode = nodeManager.deleteNode;
const listNode = nodeManager.listNode;
const getStatus = nodeManager.getStatus;
const isAdmin = nodeManager.isAdmin;

const admin = config.contract.node_manager.admin;

// =======================

describe('\n\ntest node manager\n\n', function () {

    describe('\ntest add admin\n', function () {
        it('should send a addAdmin tx and get receipt', function (done) {
            let res = addAdmin(config.testAddr[0], admin);

            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt:\n', receipt);
                    assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get addAdmin receipt err:!!!!\n', err);
                    this.skip();
                });
        });

        it('should have new admin', function() {
            let res = isAdmin(config.testAddr[0]);
            console.log('\nthe account is an admin:\n', res);
            assert.equal(res, true);
        });
    });

    describe('\ntest new node\n', function () {
        before('should be close status', function() {
            let res = getStatus(config.testAddr[1]);
            console.log('\nthe status of the node:\n', res);
            assert.equal(res, 0);
        });

        it('should send a newNode tx and get receipt', function (done) {
            let res = newNode(config.testAddr[1], admin);

            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt:\n', receipt);
                    assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get newNode receipt err:!!!!\n', err);
                    this.skip();
                });
        });

        it('should be ready status', function() {
            let res = getStatus(config.testAddr[1]);
            console.log('\nthe status of the node:\n', res);
            assert.equal(res, 1);
        });
    });

    describe('\ntest approve node\n', function () {
        before('should be ready status', function() {
            let res = getStatus(config.testAddr[1]);
            console.log('\nthe status of the node:\n', res);
            assert.equal(res, 1);
        });

        it('should send a approveNode tx and get receipt', function (done) {
            let res = approveNode(config.testAddr[1], admin);

            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt:\n', receipt);
                    assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get approveNode receipt err:!!!!\n', err);
                    this.skip();
                });
        });

        it('should be start status', function() {
            let res = getStatus(config.testAddr[1]);
            console.log('\nthe status of the node:\n', res);
            assert.equal(res, 2);
        });

        it('should have the new consensus node', function() {
            let res = listNode();
            console.log('\nthe consensus nodes:\n', res);
            assert.equal(res[res.length-1], config.testAddr[1]);
        });
    });

    describe('\ntest delete consensus node\n', function () {
        before('should be ready status and wait a new block', function(done) {
            let res = getStatus(config.testAddr[1]);
            console.log('\nthe status of the node:\n', res);
            assert.equal(res, 2);
            let num = web3.eth.blockNumber;
            let tmp;
            do {
                tmp = web3.eth.blockNumber;
            } while (tmp <= num);
            done();
        });

        it('should send a deleteNode tx and get receipt', function (done) {
            let res = deleteNode(config.testAddr[1], admin);

            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt:\n', receipt);
                    assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get deleteNode receipt err:!!!!\n', err);
                    this.skip();
                });
        });

        it('should be close status', function() {
            let res = getStatus(config.testAddr[1]);
            console.log('\nthe status of the node:\n', res);
            assert.equal(res, 0);
        });
    });
});
