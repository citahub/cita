/* jshint esversion: 6 */
/* jshint expr: true */

const chai = require('chai');
const assert = chai.assert;
const util = require('../helpers/util');
const chain_manager = require('../helpers/chain_manager');
const config = require('../config');

// util
const getTxReceipt = util.getTxReceipt;
const web3 = util.web3;

const newSideChain = chain_manager.newSideChain;
const enableSideChain = chain_manager.enableSideChain;
const disableSideChain = chain_manager.disableSideChain;
const getChainId = chain_manager.getChainId;
const getParentChainId = chain_manager.getParentChainId;
const getAuthorities = chain_manager.getAuthorities;

// TODO Add query interface and event of chain_manager.sol
//====================

//temp
let chainId;

describe('test side chain management contract', function() {
    describe('\ntest register new side chain\n', function () {

        before('Query the parent chain id ', function () {
            res = getParentChainId.call();
            console.log('\nThe parent chain id:\n', res);
            assert.equal(res, 0);
        });

        before('Query the chain id ', function () {
            res = getChainId.call();
            console.log('\nThe chain id:\n', res);
            assert.equal(res, 1);
        });

        it('should send a newSideChain tx and get receipt', function (done) {
            let res = newSideChain(config.testAddr);

            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt:\n', receipt);
                    assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get newSideChain receipt err:!!!!\n', err);
                    this.skip();
                });
        });

        // TODO Check the new sideChain
    });

    describe('\ntest enable side chain\n', function () {

        before('Query the side chain id ', function () {
            chainId = getChainId.call();
            console.log('\nThe side chain id:\n', chainId);
        });

        it('should send a enableSideChain tx and get receipt', function (done) {
            // let res = enableSideChain(chainId);
            let res = enableSideChain(2);

            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt:\n', receipt);
                    assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get enableSideChain receipt err:!!!!\n', err);
                    this.skip();
                });
        });
    });

    describe('\ntest disable side chain\n', function () {

        it('should send a disableSideChain tx and get receipt', function (done) {
            // let res = disableSideChain(chainId);
            let res = disableSideChain(2);

            getTxReceipt(res)
                .then((receipt) => {
                    console.log('\nSend ok and get receipt:\n', receipt);
                    assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
                    done();
                })
                .catch(err => {
                    console.log('\n!!!!Get disableSideChain receipt err:!!!!\n', err);
                    this.skip();
                });
        });
    });
});
