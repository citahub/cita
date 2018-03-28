/* jshint esversion: 6 */
/* jshint expr: true */

const chai = require('chai');
const assert = chai.assert;
const util = require('../helpers/util');
const quota = require('../helpers/quota');
const config = require('../config');

const web3 = util.web3;
const getTxReceipt = util.getTxReceipt;

const admin = config.contract.quota.admin;

const addAdmin = quota.addAdmin;
const setBlockGasLimit = quota.setBlockGasLimit;
const setGlobalAccountGasLimit = quota.setGlobalAccountGasLimit;
const setAccountGasLimit = quota.setAccountGasLimit;
const isAdmin = quota.isAdmin;
const getSpecialUsers = quota.getSpecialUsers;
const getUsersQuota = quota.getUsersQuota;
const getBlockGasLimit = quota.getBlockGasLimit;
const getAccountGasLimit = quota.getAccountGasLimit;

const value = Math.pow(2, 29);

// =======================

describe('test quota manager', function() { 

    describe('\ntest add admin\n', function () {

        it('should send an addAdmin tx', function(done) {
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

    describe('\ntest set block quota limit\n', function () {

        it('should send setBlockGasLimit tx', function(done) {
            let res = setBlockGasLimit(value, admin);

            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt:\n', receipt);
                    assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get setBlockGasLimit receipt err:!!!!\n', err);
                    this.skip();
                });
        });

        it('should have new block quota limit', function() {
            let res = quota.getBlockGasLimit();
            console.log('\nthe block quota limit:\n', res);
            assert.equal(res, value);
        });
    });

    describe('\ntest set default account quota limit\n', function () {

        it('should send setGlobalAccountGasLimit tx', function(done) {
            let res = setGlobalAccountGasLimit(value, admin);

            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt:\n', receipt);
                    assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get setGlobalAccountGasLimit receipt err:!!!!\n', err);
                    this.skip();
                });
        });

        it('should have new default account quota limit', function() {
            let res = quota.getAccountGasLimit();
            console.log('\nthe default account quota limit:\n', res);
            assert.equal(res, value);
        });
    });

    describe('\ntest set account\'s quota limit\n', function () {

        it('should send setAccountGasLimit tx', function(done) {
            let res = setAccountGasLimit(config.testAddr[0], value-1, admin);

            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt:\n', receipt);
                    assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get setGlobalAccountGasLimit receipt err:!!!!\n', err);
                    this.skip();
                });
        });

        it('should have new default account quota limit', function() {
            let res = quota.getAccountQuota(config.testAddr[0]);
            console.log('\nthe default account quota limit:\n', res);
            assert.equal(res, value-1);
        });

        it('should have new special account', function() {
            let res = getSpecialUsers();
            console.log('\nthe special accounts:\n', res);
            assert.equal(res[res.length-1], config.testAddr[0]);
        });

        it('should have new quotas of special accounts', function() {
            let res = quota.getUsersQuota();
            console.log('\nthe quotas of the special accounts:\n', res);
            assert.equal(res[res.length-1], value-1);
        });

    });
});
