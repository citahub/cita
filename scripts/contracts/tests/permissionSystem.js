/* jshint esversion: 6 */
/* jshint expr: true */

const Web3 = require('web3');
const config = require('./config');

var chai = require('chai');
var assert = chai.assert;
// var BigNumber = require('bignumber.js');

const web3 = new Web3(new Web3.providers.HttpProvider(config.localServer));
// Use remote server
// const web3 = new Web3(new Web3.providers.HttpProvider(config.remoteServer));

const { pmABI, pmAddr, sender } = config.contract.permission_manager;
const { psABI, psAddr, superAdmin } = config.contract.permission_system;

const permManager = web3.eth.contract(pmABI);
const pmContractInstance = permManager.at(pmAddr);

const permSys = web3.eth.contract(psABI);
const psContractInstance = permSys.at(psAddr);

const quota = 9999999;
const blockLimit = 100;

// =======================

// TODO Move to helper
function randomInt() {
    return Math.floor(Math.random() * 100).toString();
}

function getTxReceipt(res) {
    return new Promise((resolve, reject) => { 
        let count = 0;
        const filter = web3.eth.filter('latest', err => {

            if (err) reject(err);
            
            count++;

            if (count > 20) {
                filter.stopWatching(function() {});
                reject(err);
            }

            web3.eth.getTransactionReceipt(res.hash, function(err, receipt) {

                if (err) reject(err);

                if (receipt) {
                    filter.stopWatching(function() {});
                    resolve(receipt);
                }
            });
        });
    });
}

// =======================

describe('test permission system', function() { 
    before('Grant the send_tx permission to superAdmin', function(done) {
        var num = web3.eth.blockNumber;
        // Grant the send_tx permission to superAdmin
        var res = pmContractInstance.grantPermission.sendTransaction(
            superAdmin.address,
            1,
            {
                privkey: sender.privkey,
                nonce: randomInt(),
                quota,
                validUntilBlock: num + blockLimit,
                from: sender.address
            });

        getTxReceipt(res)
            .then((receipt) => {
                console.log('\nSend ok and get receipt:\n', receipt);
                assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                console.log('\n*****Start to test*****\n');
                done();
            })
            .catch(err => {
                console.log('\n!!!!Get grantPermission receipt err:\n', err);
                this.skip();
            });
    });

    after('Grant the send_tx permission to superAdmin', function(done) {
        var num = web3.eth.blockNumber;
        // Revoke the send_tx permission of superAdmin
        var res = pmContractInstance.revokePermission.sendTransaction(
            superAdmin.address,
            1,
            {
                privkey: sender.privkey,
                nonce: randomInt(),
                quota,
                validUntilBlock: num + blockLimit,
                from: sender.address
            });

        getTxReceipt(res)
            .then((receipt) => {
                console.log('\nSend ok and get receipt:\n', receipt);
                assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                console.log('\n*****End of the test*****\n');
                done();
            })
            .catch(err => {
                console.log('\n!!!!Get revokePermission receipt err:\n', err);
                this.skip();
            });
    });

    describe('test superAdmin', function() {
        it('should be the superAdmin', function() {
            var res = psContractInstance.querySuperAdmin.call();
            console.log('\nsuperAdmin:\n', res);
            console.log(web3.toAscii(res[0]));
            assert.equal(res, superAdmin.address);
        });

        it('should have the send_tx permission', function() {
            var res = pmContractInstance.queryPermission.call(superAdmin.address);
            console.log("queryPermission res: " + res);
            // assert.deepEqual(res, new BigNumber(0x1));
            assert.equal(res, 1);
        });
    });

    describe('test group', function() {
        it('should null', function() {
            var res = psContractInstance.queryAllGroups.call();
            assert.equal(res.length, 0);
        });

        it('should be not in the test_group', function() {
            var res = psContractInstance.queryGroups.call(config.testAddr[0]);
            assert.equal(res.length, 0);
        });

        describe('test add group', function() {
            before('should send a newGroup tx and get receipt', function(done) {
                var num = web3.eth.blockNumber;
                var res = psContractInstance.newGroup.sendTransaction(
                    'zz',
                    'test_group',
                    config.testAddr,
                    true,
                    0,
                    '',
                    'This is a test group',
                    {
                        privkey: superAdmin.privkey,
                        nonce: randomInt(),
                        quota,
                        validUntilBlock: num + blockLimit,
                        from: superAdmin.address
                    });

                getTxReceipt(res)
                    .then((receipt) => {
                        console.log('\nSend ok and get receipt:\n', receipt);
                        assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                        done();
                    })
                    .catch(err => {
                        console.log('\n!!!!Get newGroup receipt err:!!!!\n', err);
                        this.skip();
                    });
            });

            it('should be in the new group', function() {
                var res = psContractInstance.queryGroups.call(config.testAddr[0]);
                console.log('\ngroups:\n', res);
                assert.equal(res.length, 1);
                assert.equal(res[0].substr(0, 22), web3.toHex('test_group'));
            });

            it('should be more groups', function() {
                var res = psContractInstance.queryAllGroups.call();
                console.log('\ngroups:\n', res);
                assert.equal(res.length, 1);
                assert.equal(res[0].substr(0, 22), web3.toHex('test_group'));
            });

            it('should have users', function() {
                var res = psContractInstance.queryUsers.call(web3.toHex('test_group'));
                console.log('\nusers of test_group:\n', res);
                assert.equal(res.length, 3);
            });

            it('should have subSwitch', function() {
                var res = psContractInstance.querySubSwitch.call(web3.toHex('test_group'));
                console.log('\nsubSwitch of test_group:\n', res);
                assert.equal(res, true);
            });

            it('should have profile', function() {
                var res = psContractInstance.queryProfile.call(web3.toHex('test_group'));
                console.log('\nprofile of test_group:\n', res);
                assert.equal(res, 'This is a test group');
            });
        });

        describe('test modify group', function() {
            before('should send a modifySubSwitch tx and get receipt', function(done) {
                var num = web3.eth.blockNumber;
                var res = psContractInstance.modifySubSwitch.sendTransaction(
                    'test_group',
                    'test_group',
                    '',
                    false,
                    {
                        privkey: superAdmin.privkey,
                        nonce: randomInt(),
                        quota,
                        validUntilBlock: num + blockLimit,
                        from: superAdmin.address
                    });

                getTxReceipt(res)
                    .then((receipt) => {
                        console.log('\nSend ok and get receipt:\n', receipt);
                        assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                        done();
                    })
                    .catch(err => {
                        console.log('\n!!!!Get modifySubSwitch receipt err:!!!!\n', err);
                        this.skip();
                    });
            });

            it('should be a new subSwitch', function() {
                var res = psContractInstance.querySubSwitch.call(web3.toHex('test_group'));
                console.log('\ngroup subSwitch:\n', res);
                assert.equal(res, false);
            });
            
            it('should send a modifyGroupName tx and get receipt', function(done) {
                var num = web3.eth.blockNumber;
                var res = psContractInstance.modifyGroupName.sendTransaction(
                    'test_group',
                    'test_group_new',
                    '',
                    '',
                    {
                        privkey: superAdmin.privkey,
                        nonce: randomInt(),
                        quota,
                        validUntilBlock: num + blockLimit,
                        from: superAdmin.address
                    });

                getTxReceipt(res)
                    .then((receipt) => {
                        console.log('\nSend ok and get receipt:\n', receipt);
                        assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                        done();
                    })
                    .catch(err => {
                        console.log('\n!!!!Get modifyGroupName receipt err:!!!!\n', err);
                    });
            });

            it('should be a new group name', function() {
                var res = psContractInstance.queryGroups.call(config.testAddr[0]);
                console.log('\ngroup name:\n', res);
                assert.equal(res[0].substr(0, 30), web3.toHex('test_group_new'));
            });
        });

        describe('test delete group', function() {
            before('should send a deleteGroup tx and get receipt', function(done) {
                var num = web3.eth.blockNumber;
                var res = psContractInstance.deleteGroup.sendTransaction(
                    'zz',
                    'test_group_new',
                    '',
                    {
                        privkey: superAdmin.privkey,
                        nonce: randomInt(),
                        quota,
                        validUntilBlock: num + blockLimit,
                        from: superAdmin.address
                    });

                getTxReceipt(res)
                    .then((receipt) => {
                        console.log('\nSend ok and get receipt:\n', receipt);
                        assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                        done();
                    })
                    .catch(err => {
                        console.log('\n!!!!Get deleteGroup receipt err:!!!!\n', err);
                        this.skip();
                    });

            });

            it('should be less groups', function() {
                var res = psContractInstance.queryAllGroups.call();
                console.log('\ngroups:\n', res);
                assert.equal(res.length, 0);
            });

        });
    });

    describe('test role', function() {
        it('should null', function() {
            var res = psContractInstance.queryAllRoles.call();
            assert.equal(res.length, 0);
        });

        it('should have no permissions', function() {
            var res = psContractInstance.queryPermissions.call(web3.toHex('test_role'));
            assert.equal(res.length, 0);
        });

        describe('test add role', function() {
            before('should send a newRole tx and get receipt', function(done) {
                var num = web3.eth.blockNumber;
                var res = psContractInstance.newRole.sendTransaction(
                    'test',
                    'test_role',
                    0,
                    [ 'DeleteRole', 'UpdateRole' ],
                    0,
                    {
                        privkey: superAdmin.privkey,
                        nonce: randomInt(),
                        quota,
                        validUntilBlock: num + blockLimit,
                        from: superAdmin.address
                    });

                getTxReceipt(res)
                    .then((receipt) => {
                        console.log('\nSend ok and get receipt:\n', receipt);
                        assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                        done();
                    })
                    .catch(err => {
                        console.log('\n!!!!Get newRole receipt err:!!!!\n', err);
                        this.skip();
                    });
            });

            // TODO Check the permission
            it('should have the permissions', function() {
                var res = psContractInstance.queryPermissions.call(web3.toHex('test_role'));
                console.log("\npermissions:\n", res);
                assert.equal(res.length, 2);
            });

            it('should be more roles', function() {
                var res = psContractInstance.queryAllRoles.call();
                console.log('\nroles :\n', res);
                assert.equal(res.length, 1);
            });
        });

        describe('test modify role', function() {
            before('should send a modifyRoleName tx and get receipt', function(done) {
                var num = web3.eth.blockNumber;
                var res = psContractInstance.modifyRoleName.sendTransaction(
                    'test_role',
                    'test_role_new',
                    '',
                    '',
                    {
                        privkey: superAdmin.privkey,
                        nonce: randomInt(),
                        quota,
                        validUntilBlock: num + blockLimit,
                        from: superAdmin.address
                    });

                getTxReceipt(res)
                    .then((receipt) => {
                        console.log('\nSend ok and get receipt:\n', receipt);
                        assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                        done();
                    })
                    .catch(err => {
                        console.log('\n!!!!Get modifyRoleName receipt err:!!!!\n', err);
                        this.skip();
                    });
            });

            it('should be a new role name', function() {
                var res = psContractInstance.queryAllRoles.call();
                console.log('\nAll role name:\n', res);
                assert.equal(res.length, 1);
                assert.equal(res[0].substr(0, 28), web3.toHex('test_role_new'));
            });
       
            it('should have permissions in new role', function() {
                var res = psContractInstance.queryPermissions.call(web3.toHex('test_role_new'));
                console.log("\npermissions:\n", res);
                assert.equal(res.length, 2);
            });

            it('should have no permissions in old role', function() {
                var res = psContractInstance.queryPermissions.call(web3.toHex('test_role'));
                console.log("\npermissions:\n", res);
                assert.equal(res.length, 0);
            });
        });

        describe('test delete role', function() {
            before('should send a deleteRole tx and get receipt', function(done) {
                var num = web3.eth.blockNumber;
                var res = psContractInstance.deleteRole.sendTransaction(
                    'test_role_new',
                    'test',
                    '',
                    {
                        privkey: superAdmin.privkey,
                        nonce: randomInt(),
                        quota,
                        validUntilBlock: num + blockLimit,
                        from: superAdmin.address
                    });

                getTxReceipt(res)
                    .then((receipt) => {
                        console.log('\nSend ok and get receipt:\n', receipt);
                        assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                        done();
                    })
                    .catch(err => {
                        console.log('\n!!!!Get deleteRole receipt err:!!!!\n', err);
                        this.skip();
                    });
            });

            it('should be less roles', function() {
                var res = psContractInstance.queryAllRoles.call();
                console.log('\nroles:\n', res);
                assert.equal(res.length, 0);
            });

        });
    });

    describe('test authorization', function() {
        it('should have no role of the test_group', function() {
            var res = psContractInstance.queryRoles.call(web3.toHex('test_group'));
            console.log('\nrole of the group:\n', res);
            assert.equal(res.length, 0);
        });

        describe('test set authorazation', function() {
            before('should send a setAuthorization tx and get receipt', function(done) {
                var num = web3.eth.blockNumber;
                var res = psContractInstance.setAuthorization.sendTransaction(
                    'test_group',
                    'test_role',
                    '',
                    {
                        privkey: superAdmin.privkey,
                        nonce: randomInt(),
                        quota,
                        validUntilBlock: num + blockLimit,
                        from: superAdmin.address
                    });

                getTxReceipt(res)
                    .then((receipt) => {
                        console.log('\nSend ok and get receipt:\n', receipt);
                        assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                        done();
                    })
                    .catch(err => {
                        console.log('\n!!!!Get setAuthorization receipt err:!!!!\n', err);
                        this.skip();
                    });
            });

            it('should have test_role of the test_group', function() {
                var res = psContractInstance.queryRoles.call(web3.toHex('test_group'));
                console.log('\nrole of the test group:\n', res);
                assert.equal(res[0].substr(0, 20), web3.toHex('test_role'));
            });
        });

        describe('test cancel authorazation', function() {
            before('should send a cancelAuthorization tx and get receipt', function(done) {
                var num = web3.eth.blockNumber;
                var res = psContractInstance.cancelAuthorization.sendTransaction(
                    'test_group',
                    'test_role',
                    '',
                    {
                        privkey: superAdmin.privkey,
                        nonce: randomInt(),
                        quota,
                        validUntilBlock: num + blockLimit,
                        from: superAdmin.address
                    });

                getTxReceipt(res)
                    .then((receipt) => {
                        console.log('\nSend ok and get receipt:\n', receipt);
                        assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                        done();
                    })
                    .catch(err => {
                        console.log('\n!!!!Get cancelAuthorization receipt err:!!!!\n', err);
                        this.skip();
                    });
            });

            it('should have no test_role of the test_group', function() {
                var res = psContractInstance.queryRoles.call(web3.toHex('test_group'));
                console.log('\nrole of the test group:\n', res);
                assert.equal(res.length, 0);
            });
        });
    });
});
