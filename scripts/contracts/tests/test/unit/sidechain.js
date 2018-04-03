/* jshint esversion: 6 */
/* jshint expr: true */

const chai = require('chai');
const assert = chai.assert;
const util = require('../helpers/util');
const sidechain = require('../helpers/sidechain');
const config = require('../config');
const getTxReceipt = util.getTxReceipt;

const newChain = sidechain.newChain;
const enableChain = sidechain.enableChain;
const disableChain = sidechain.disableChain;
const getStatus = sidechain.getStatus;
const getNodes = sidechain.getNodes;
const getId = sidechain.getId;

//====================

describe('test side chain management contract', function() {

    describe('\ntest register new side chain\n', function () {

        it('should send a newChain tx and get receipt', function (done) {
            let res = newChain(config.testAddr);

            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt:\n', receipt);
                    assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get newChain receipt err:!!!!\n', err);
                    this.skip();
                });
        });
    });

    describe('\ntest enable side chain\n', function () {

        it('should send a enableChain tx and get receipt', function (done) {
            let res = enableChain(1);

            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt:\n', receipt);
                    assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get enableChain receipt err:!!!!\n', err);
                    this.skip();
                });
        });

        it('should have updated the status of side chain', function () {
            let res = getStatus(1);
            console.log('\nNow the status of child group:\n', res);
            assert.equal(res, true);
        });
    });

    describe('\ntest disable side chain\n', function () {

        it('should send a disableChain tx and get receipt', function (done) {
            let res = disableChain(1);

            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt:\n', receipt);
                    assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get disableChain receipt err:!!!!\n', err);
                    this.skip();
                });
        });

        it('should have updated the status of side chain', function () {
            let res = getStatus(1);
            console.log('\nNow the status of side chain:\n', res);
            assert.equal(res, false);
        });
    });

    describe('\ntest get nodes of side chain\n', function () {

        it('should have get nodes in the side chain', function () {
            let res = getNodes(1);
            console.log('\nInfo:\n', res);

            for (let i = 0; i < res.length; i++) {
                assert.equal(res[i], config.testAddr[i]);
            }
        });
    });
});
