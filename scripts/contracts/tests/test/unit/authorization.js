/* jshint esversion: 6 */
/* jshint expr: true */

const chai = require('chai');
const assert = chai.assert;
const authorization = require('../helpers/authorization');
const config = require('../config');

const superAdmin = config.contract.authorization.superAdmin;
const permissions = config.permissions;
const resources = config.resources;

const queryPermissions = authorization.queryPermissions;
const queryAccounts = authorization.queryAccounts;
const checkPermission = authorization.checkPermission;
const queryAllAccounts = authorization.queryAllAccounts;

const rootGroup = "0x00000000000000000000000000000000013241b6";
const len = permissions.length;

// =======================

describe('test authorization contract', function() { 

    it('should be the build-in authorization: superAdmin has the permission', function() {
        let res = queryPermissions(superAdmin.address);
        console.log('\nPermissions of superAdmin:\n', res);

        for (let i=0; i<len; i++) 
            assert.equal(res[i], permissions[i]);
    });

    it('should be the build-in authorization: rootGroup has the permission', function() {
        let res = queryPermissions(rootGroup);
        console.log('\nPermissions of rootGroup:\n', res);

        for (let i=0; i<2; i++) 
            assert.equal(res[i], permissions[i]);
    });

    it('should be the build-in authorization: account of the permission', function() {
        for (let i=2; i<len; i++) {
            let res = queryAccounts(permissions[i]);
            console.log('\nAccount of permissions:\n', res);
            assert.equal(res, superAdmin.address);
        }
        for (let i=0; i<2; i++) {
            let res = queryAccounts(permissions[i]);
            console.log('\nAccount of permissions:\n', res);
            assert.equal(res[0], superAdmin.address);
            assert.equal(res[1], rootGroup);
        }
    });

    it("should check the superAdmin has the resource", function() {
        for (let i=0; i<resources.length; i++) {
            for(let j=1; j<resources[i].length; j++) {
                let res = checkPermission(
                    superAdmin.address,
                    resources[i][0],
                    resources[i][j]
                );
                console.log('\nResult of check:(%i,%j)\n', i,j,res);
                assert.equal(res, true);
            }
        }
    });

    it("should check the rootGroup has the resource", function() {
        for (let i=0; i<2; i++) {
            for(let j=1; j<resources[i].length; j++) {
                let res = checkPermission(
                    superAdmin.address,
                    resources[i][0],
                    resources[i][j]
                );
                console.log('\nResult of check:(%i,%j)\n', i,j,res);
                assert.equal(res, true);
            }
        }
    });

    it("should check the superAdmin does not have the resource: wrong func", function() {
        let res = checkPermission(
                superAdmin.address,
                "0x00000000000000000000000000000000013241b2",
                "0xf036ed57"
            );
        console.log('\nResult of check:\n', res);
        assert.equal(res, false);
    });

    it("should check the superAdmin does not have the resource: wrong cont", function() {
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
