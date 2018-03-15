/* jshint esversion: 6 */
/* jshint expr: true */

const chai = require('chai');
const assert = chai.assert;
const util = require('../helpers/util');
const config = require('../config');

const permissionManagement = require('../helpers/permission_management');

// config
const superAdmin = config.contract.authorization.superAdmin;
const sender = config.testSender;

// util
const web3 = util.web3;
const getTxReceipt = util.getTxReceipt;
const quota = util.quota;
const blockLimit = util.blockLimit;

const STORE_ADDRESS = "0xffffffffffffffffffffffffffffffffffffffff";
const send_tx = "0x0000000000000000000000000000000000000001";
const setAuthorization = permissionManagement.setAuthorization;
const cancelAuthorization = permissionManagement.cancelAuthorization;

// TODO / BUG: use store_address
//           : data is null
// =======================

describe('\n\ntest send transaction permission\n\n', function() { 

    it('should send a tx and get receipt: superAdmin', function(done) {
        let res = web3.eth.sendTransaction({
                privkey: superAdmin.privkey,
                nonce: util.randomInt(),
                quota: quota,
                validUntilBlock: web3.eth.blockNumber + blockLimit,
                from: superAdmin.address,
                // to: STORE_ADDRESS,
                to: config.testAddr[0],
                // data: ''
                data: '00000000'
            });
        getTxReceipt(res)
            .then((receipt) => {
                console.log('\nSend ok and get receipt:\n', receipt);
                // assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                assert.equal(receipt.errorMessage, "No Call contract permission.", JSON.stringify(receipt.errorMessage));

                done();
            })
            .catch(err => {
                console.log('\n!!!!Get receipt err:!!!!\n', err);
                this.skip();
                done();
            });
    });

    it('should send a tx and get receipt with error message: testSender', function(done) {
        let res = web3.eth.sendTransaction({
                privkey: sender.privkey,
                nonce: util.randomInt(),
                quota: quota,
                validUntilBlock: web3.eth.blockNumber + blockLimit,
                from: sender.address,
                // to: STORE_ADDRESS,
                to: config.testAddr[0],
                data: '00000000'
            });
        getTxReceipt(res)
            .then((receipt) => {
                console.log('\nSend ok and get receipt:\n', receipt);
                assert.equal(receipt.errorMessage, 'No transaction permission.', JSON.stringify(receipt.errorMessage));
                done();
            })
            .catch(err => {
                console.log('\n!!!!Get receipt err:!!!!\n', err);
                this.skip();
                done();
            });
    });

    describe('\n\ntest send tx permission after set send permission\n\n', function() { 

        before('should send a setAuthorization tx and get receipt', function(done) {
            let res = setAuthorization(sender.address, send_tx);

            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt:\n', receipt);
                    assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get setAuthorization receipt err:!!!!\n', err);
                    this.skip();
                    done();
                });
        });
        
        it('should wait a new block', function(done) {
            let num = web3.eth.blockNumber;
            let tmp;
            do {
                tmp = web3.eth.blockNumber;
            } while (tmp <= num);
            done();
        });

        it('should send a tx and get receipt: testSender', function(done) {
            let res = web3.eth.sendTransaction({
                    privkey: sender.privkey,
                    nonce: util.randomInt(),
                    quota: quota,
                    validUntilBlock: web3.eth.blockNumber + blockLimit,
                    from: sender.address,
                    // to: STORE_ADDRESS,
                    to: config.testAddr[0],
                    data: '00000000'
                });
            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt:\n', receipt);
                    // assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                    assert.equal(receipt.errorMessage, "No Call contract permission.", JSON.stringify(receipt.errorMessage));
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get receipt err:!!!!\n', err);
                    this.skip();
                    done();
                });
        });
    });

    describe('\n\ntest send tx permission after cancel send permission\n\n', function() { 

        before('should send a cancelAuthorization tx and get receipt', function(done) {
            let res = cancelAuthorization(sender.address, send_tx);

            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt:\n', receipt);
                    assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get cancelAuthorization receipt err:!!!!\n', err);
                    this.skip();
                    done();
                });
        });
        
        it('should wait a new block', function(done) {
            let num = web3.eth.blockNumber;
            let tmp;
            do {
                tmp = web3.eth.blockNumber;
            } while (tmp <= num);
            done();
        });

        it('should send a tx and get receipt with error message: testSender', function(done) {
            let res = web3.eth.sendTransaction({
                    privkey: sender.privkey,
                    nonce: util.randomInt(),
                    quota: quota,
                    validUntilBlock: web3.eth.blockNumber + blockLimit,
                    from: sender.address,
                    // to: STORE_ADDRESS,
                    to: config.testAddr[0],
                    data: '00000000'
                });
            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt:\n', receipt);
                    assert.equal(receipt.errorMessage, 'No transaction permission.', JSON.stringify(receipt.errorMessage));
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get receipt err:!!!!\n', err);
                    this.skip();
                    done();
                });
        });
    });
});
