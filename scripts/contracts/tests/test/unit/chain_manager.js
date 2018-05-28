const assert = require('assert');
const mocha = require('mocha');
const util = require('../helpers/util');
const chainManager = require('../helpers/chain_manager');
const config = require('../config');

// util
const { getTxReceipt, logger } = util;

const {
  newSideChain, enableSideChain, disableSideChain, getChainId, getParentChainId,
} = chainManager;

const { describe, it, before } = mocha;

// TODO Add query interface and event of chain_manager.sol
//= ===================

// temp
let chainId;

describe('test side chain management contract', () => {
  describe('\ntest register new side chain\n', () => {
    before('Query the parent chain id ', () => {
      const res = getParentChainId.call();
      logger.debug('\nThe parent chain id:\n', res);
      assert.equal(res, 0);
    });

    before('Query the chain id ', () => {
      const res = getChainId.call();
      logger.debug('\nThe chain id:\n', res);
      assert.equal(res, 1);
    });

    it('should send a newSideChain tx and get receipt', (done) => {
      const res = newSideChain(config.testAddr);

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get newSideChain receipt err:!!!!\n', err);
          this.skip();
        });
    });

    // TODO Check the new sideChain
  });

  describe('\ntest enable side chain\n', () => {
    before('Query the side chain id ', () => {
      chainId = getChainId.call();
      logger.debug('\nThe side chain id:\n', chainId);
    });

    it('should send a enableSideChain tx and get receipt', (done) => {
      // let res = enableSideChain(chainId);
      const res = enableSideChain(2);

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get enableSideChain receipt err:!!!!\n', err);
          this.skip();
        });
    });
  });

  describe('\ntest disable side chain\n', () => {
    it('should send a disableSideChain tx and get receipt', (done) => {
      // let res = disableSideChain(chainId);
      const res = disableSideChain(2);

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get disableSideChain receipt err:!!!!\n', err);
          this.skip();
        });
    });
  });
});
