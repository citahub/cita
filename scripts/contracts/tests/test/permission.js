/* jshint esversion: 6 */
/* jshint expr: true */

const chai = require('chai');
const assert = chai.assert;
const util = require('./helpers/util');
const permission = require('./helpers/permission');

const web3 = util.web3;

const queryInfo = permission.queryInfo;
const inPermission = permission.inPermission;

// =======================

describe('test permission contract', function() { 

    it('should be the build-in newPermission', function() {
        let res = queryInfo();
        console.log('\nInfo:\n', res);
        assert.equal(res[0].substr(0, 28), web3.toHex('newPermission'));
        assert.equal(res[1], '0x00000000000000000000000000000000013241b2');
        assert.equal(res[2], '0xf036ed56');
    });

    it('test resource in permission', function() {
        let res = inPermission(
                '0x00000000000000000000000000000000013241b2',
                '0xf036ed56'
            );
        console.log('\nThe result:\n', res);
        assert.equal(res, true);
    });

    it('test resource not in permission: wrong address', function() {
        let res = inPermission(
                '0x00000000000000000000000000000000013241b3',
                '0xf036ed56'
            );
        console.log('\nThe result:\n', res);
        assert.equal(res, false);
    });

    it('test resource not in permission: wrong function', function() {
        let res = inPermission(
                '0x00000000000000000000000000000000013241b2',
                '0xf036ed57'
            );
        console.log('\nThe result:\n', res);
        assert.equal(res, false);
    });

    it('test resource not in permission: all wrong', function() {
        let res = inPermission(
                '0x00000000000000000000000000000000013241b3',
                '0xf036ed57'
            );
        console.log('\nThe result:\n', res);
        assert.equal(res, false);
    });
});
