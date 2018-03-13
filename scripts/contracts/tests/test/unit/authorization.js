/* jshint esversion: 6 */
/* jshint expr: true */

const chai = require('chai');
const assert = chai.assert;
const authorization = require('../helpers/authorization');
const config = require('../config');

const { superAdmin, permissions, resources } = config.contract.authorization;

const queryPermissions = authorization.queryPermissions;
const queryAccounts = authorization.queryAccounts;
const checkPermission = authorization.checkPermission;
const queryAllAccounts = authorization.queryAllAccounts;

// =======================

describe('test authorization contract', function() { 

    it('should be the build-in authorization: superAdmin has the permission', function() {
        let res = queryPermissions(superAdmin.address);
        console.log('\nPermissions of superAdmin:\n', res);

        for (let i=0; i<5; i++) 
            assert.equal(res[i], permissions[i]);
    });

    it('should be the build-in authorization: account of the permission', function() {
        for (let i=0; i<permissions.length; i++) {
            let res = queryAccounts(permissions[i]);
            console.log('\nAccount of permissions:\n', res);
            assert.equal(res, superAdmin.address);
        }
    });

    it("should check the account has the resource", function() {
        for (let i=0; i<resources.length; i++) {
            let res = checkPermission(
                superAdmin.address,
                "0x00000000000000000000000000000000013241b2",
                resources[i]
            );
            console.log('\nResult of check:\n', res);
            assert.equal(res, true);
        }
    });

    it("should check the account does not have the resource", function() {
        let res = checkPermission(
                superAdmin.address,
                "0x00000000000000000000000000000000013241b2",
                "0xf036ed57"
            );
        console.log('\nResult of check:\n', res);
        assert.equal(res, false);
    });

    it("should check the account does not have the resource", function() {
        let res = checkPermission(
                superAdmin.address,
                "0x00000000000000000000000000000000013241b3",
                "0xf036ed56"
            );
        console.log('\nResult of check:\n', res);
        assert.equal(res, false);
    });

    it("should have all the accounts", function() {
        let res = queryAllAccounts();
        console.log('\nAll accounts:\n', res);
        assert.equal(res[0], superAdmin.address);
    });
});
