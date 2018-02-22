/* jshint esversion: 6 */
/* jshint expr: true */

const Web3 = require('web3');
const config = require('./config');

var chai = require('chai');
var assert = chai.assert;

const web3 = new Web3(new Web3.providers.HttpProvider(config.localServer));
// Use remote server
// const web3 = new Web3(new Web3.providers.HttpProvider(config.remoteServer));

const { pmABI, pmAddr, sender } = config.contract.permission_manager;
const { pABI, pAddr} = config.contract.permission;

const permManager = web3.eth.contract(pmABI);
const pmContractInstance = permManager.at(pmAddr);

const perm= web3.eth.contract(pABI);
const pContractInstance = perm.at(pAddr);

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

describe('test permission contract', function() { 

    it('should be the build-in newPermission', function() {
        var res = pContractInstance.queryInfo.call();
        console.log('\nInfo:\n', res);
        assert.equal(res[0].substr(0, 26), web3.toHex('newPermisson'));
        assert.equal(res[1], '0x00000000000000000000000000000000013241b2');
        assert.equal(res[2], '0xf036ed56');
    });

    it('test resource in permission', function() {
        var res = pContractInstance.inPermission.call(
                '0x00000000000000000000000000000000013241b2',
                '0xf036ed56'
            );
        console.log('\nThe result:\n', res);
        assert.equal(res, true);
    });

    it('test resource not in permission: wrong address', function() {
        var res = pContractInstance.inPermission.call(
                '0x00000000000000000000000000000000013241b3',
                '0xf036ed56'
            );
        console.log('\nThe result:\n', res);
        assert.equal(res, false);
    });

    it('test resource not in permission: wrong function', function() {
        var res = pContractInstance.inPermission.call(
                '0x00000000000000000000000000000000013241b2',
                '0xf036ed57'
            );
        console.log('\nThe result:\n', res);
        assert.equal(res, false);
    });

    it('test resource not in permission: all wrong', function() {
        var res = pContractInstance.inPermission.call(
                '0x00000000000000000000000000000000013241b3',
                '0xf036ed57'
            );
        console.log('\nThe result:\n', res);
        assert.equal(res, false);
    });
});
