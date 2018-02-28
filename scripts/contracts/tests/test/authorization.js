/* jshint esversion: 6 */
/* jshint expr: true */

const Web3 = require('web3');
const config = require('./config');

var chai = require('chai');
var assert = chai.assert;

const web3 = new Web3(new Web3.providers.HttpProvider(config.localServer));
// Use remote server
// const web3 = new Web3(new Web3.providers.HttpProvider(config.remoteServer));

const { aABI, aAddr, superAdmin, permissions, resources } = config.contract.authorization;

const auth = web3.eth.contract(aABI);
const aContractInstance = auth.at(aAddr);

const quota = 9999999;
const blockLimit = 100;

describe('test authorization contract', function() { 

    it('should be the build-in authorization: superAdmin has the permission', function() {
        var res = aContractInstance.queryPermissions.call(superAdmin);
        console.log('\nPermissions of superAdmin:\n', res);

        for (var i=0; i<5; i++) 
            assert.equal(res[i], permissions[i]);
    });

    it('should be the build-in authorization: account of the permission', function() {
        for (var i=0; i<permissions.length; i++) {
            var res = aContractInstance.queryAccounts.call(permissions[i]);
            console.log('\nAccount of permissions:\n', res);
            assert.equal(res, superAdmin);
        }
    });

    it("should check the account has the resource", function() {
        for (var i=0; i<resources.length; i++) {
            var res = aContractInstance.checkPermission.call(
                superAdmin,
                "0x00000000000000000000000000000000013241b2",
                resources[i]
            );
            console.log('\nResult of check:\n', res);
            assert.equal(res, true);
        }
    });

    it("should check the account does not have the resource", function() {
        var res = aContractInstance.checkPermission.call(
                superAdmin,
                "0x00000000000000000000000000000000013241b2",
                "0xf036ed57"
            );
            console.log('\nResult of check:\n', res);
            assert.equal(res, false);
    });

    it("should check the account does not have the resource", function() {
        var res = aContractInstance.checkPermission.call(
                superAdmin,
                "0x00000000000000000000000000000000013241b3",
                "0xf036ed56"
            );
            console.log('\nResult of check:\n', res);
            assert.equal(res, false);
    });
});
