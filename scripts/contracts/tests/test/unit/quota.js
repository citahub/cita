/* jshint esversion: 6 */
/* jshint expr: true */

const chai = require('chai');
const assert = chai.assert;
const util = require('../helpers/util');
const quota = require('../helpers/quota');

const web3 = util.web3;

const isAdmin = quota.isAdmin;
const getAccounts = quota.getAccounts;
const getQuotas = quota.getQuotas;
const getBQL = quota.getBQL;
const getDefaultAQL = quota.getDefaultAQL;
const getAQL = quota.getAQL;

// =======================

describe('test quota manager constructor', function() { 

    it('should have build-in admin', function() {
        let res = isAdmin(quota.admin.address);
        console.log('\nthe account is the admin:\n', res);
        assert.equal(res, true);
    });

    it('should have build-in special account', function() {
        let res = getAccounts();
        console.log('\nthe special accounts:\n', res);
        assert.equal(res[0], quota.admin.address);
    });

    it('should have build-in quotas of special accounts', function() {
        let res = quota.getQuotas();
        console.log('\nthe quotas of the special accounts:\n', res);
        assert.equal(res[0], 1073741824);
    });

    it('should have build-in block quota limit', function() {
        let res = quota.getBQL();
        console.log('\nthe block quota limit:\n', res);
        assert.equal(res, 1073741824);
    });

    it('should have build-in default quota limit of account', function() {
        let res = quota.getDefaultAQL();
        console.log('\nthe default quota limit of account:\n', res);
        assert.equal(res, 268435456);
    });

    it('should have build-in quota of admin', function() {
        let res = quota.getAQL(quota.admin.address);
        console.log('\nthe quota of admin:\n', res);
        assert.equal(res, 1073741824);
    });
});
