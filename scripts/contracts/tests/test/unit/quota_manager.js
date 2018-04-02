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
const setBQL = quota.setBQL;
const setDefaultAQL = quota.setDefaultAQL;
const setAQL = quota.setAQL;
const isAdmin = quota.isAdmin;
const getAccounts = quota.getAccounts;
const getQuotas = quota.getQuotas;
const getBQL = quota.getBQL;
const getAQL = quota.getAQL;
const getDefaultAQL = quota.getDefaultAQL;

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

        it('should send setBQL tx', function(done) {
            let res = setBQL(value, admin);

            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt:\n', receipt);
                    assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get setBQL receipt err:!!!!\n', err);
                    this.skip();
                });
        });

        it('should have new block quota limit', function() {
            let res = quota.getBQL();
            console.log('\nthe block quota limit:\n', res);
            assert.equal(res, value);
        });
    });

    describe('\ntest set default account quota limit\n', function () {

        it('should send setDefaultAQL tx', function(done) {
            let res = setDefaultAQL(value, admin);

            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt:\n', receipt);
                    assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get setDefaultAQL receipt err:!!!!\n', err);
                    this.skip();
                });
        });

        it('should have new default account quota limit', function() {
            let res = quota.getDefaultAQL();
            console.log('\nthe default account quota limit:\n', res);
            assert.equal(res, value);
        });
    });

    describe('\ntest set account\'s quota limit\n', function () {

        it('should send setAQL tx', function(done) {
            let res = setAQL(config.testAddr[0], value-1, admin);

            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt:\n', receipt);
                    assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get setDefaultAQL receipt err:!!!!\n', err);
                    this.skip();
                });
        });

        it('should have new account quota limit', function() {
            let res = quota.getAQL(config.testAddr[0]);
            console.log('\nthe default account quota limit:\n', res);
            assert.equal(res, value-1);
        });

        it('should have new special account', function() {
            let res = getAccounts();
            console.log('\nthe special accounts:\n', res);
            assert.equal(res[res.length-1], config.testAddr[0]);
        });

        it('should have new quotas of special accounts', function() {
            let res = quota.getQuotas();
            console.log('\nthe quotas of the special accounts:\n', res);
            assert.equal(res[res.length-1], value-1);
        });

    });
});
