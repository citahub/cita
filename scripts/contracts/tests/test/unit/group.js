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
        assert.equal(res[1][0], '0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523');
        assert.equal(res[1][1], '0xd3f1a71d1d8f073f4e725f57bbe14d67da22f888');
        assert.equal(res[1][2], '0x9dcd6b234e2772c5451fd4ccf7582f4283140697');
    });

    it('should be the build-in accounts', function()  {
        let res = queryAccounts();
        console.log('\nAccounts:\n', res);
        assert.equal(res[0], '0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523');
        assert.equal(res[1], '0xd3f1a71d1d8f073f4e725f57bbe14d67da22f888');
        assert.equal(res[2], '0x9dcd6b234e2772c5451fd4ccf7582f4283140697');
    });

    it('should be the build-in parent group', function() {
        let res = queryParent();
        console.log('\nParent group:\n', res);
        assert.equal(res, '0x0000000000000000000000000000000000000000');
    });
});
