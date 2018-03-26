/* jshint esversion: 6 */
/* jshint expr: true */

const chai = require('chai');
const assert = chai.assert;
const util = require('../helpers/util');
const group = require('../helpers/group');

const web3 = util.web3;

const queryInfo = group.queryInfo;
const queryAccounts = group.queryAccounts;
const queryParent = group.queryParent;

// =======================

describe('test group contract', function() {

    it('should be the build-in rootGroup', function() {
        let res = queryInfo();
        console.log('\nInfo:\n', res);
        assert.equal(res[0].substr(0, 20), web3.toHex('rootGroup'));
        assert.equal(res[1][0], '0x1a702a25c6bca72b67987968f0bfb3a3213c5688');
        assert.equal(res[1][1], '0x0dbd369a741319fa5107733e2c9db9929093e3c7');
        assert.equal(res[1][2], '0x9dcd6b234e2772c5451fd4ccf7582f4283140697');
    });

    it('should be the build-in accounts', function()  {
        let res = queryAccounts();
        console.log('\nAccounts:\n', res);
        assert.equal(res[0], '0x1a702a25c6bca72b67987968f0bfb3a3213c5688');
        assert.equal(res[1], '0x0dbd369a741319fa5107733e2c9db9929093e3c7');
        assert.equal(res[2], '0x9dcd6b234e2772c5451fd4ccf7582f4283140697');
    });

    it('should be the build-in parent group', function() {
        let res = queryParent();
        console.log('\nParent group:\n', res);
        assert.equal(res, '0x0000000000000000000000000000000000000000');
    });
});
