const mocha = require('mocha');
const assert = require('assert');
const util = require('../helpers/util');
const nodeManager = require('../helpers/node_manager');
const config = require('../config');

// util
const { logger, web3, getTxReceipt } = util;

// node_manager
const {
  isAdmin, getStatus, listNode, deleteNode, approveNode, addAdmin, newNode,
} = nodeManager;

const { admin } = config.contract.node_manager;

const { describe, it, before } = mocha;

// =======================

describe('\n\ntest node manager\n\n', () => {
  describe('\ntest add admin\n', () => {
    it('should send a addAdmin tx and get receipt', (done) => {
      const res = addAdmin(config.testAddr[0], admin);

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get addAdmin receipt err:!!!!\n', err);
          this.skip();
        });
    });

    it('should have new admin', () => {
      const res = isAdmin(config.testAddr[0]);
      logger.debug('\nthe account is an admin:\n', res);
      assert.equal(res, true);
    });
  });

  describe('\ntest new node\n', () => {
    before('should be close status', () => {
      const res = getStatus(config.testAddr[1]);
      logger.debug('\nthe status of the node:\n', res);
      assert.equal(res, 0);
    });

    it('should send a newNode tx and get receipt', (done) => {
      const res = newNode(config.testAddr[1], admin);

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get newNode receipt err:!!!!\n', err);
          this.skip();
        });
    });

    it('should be ready status', () => {
      const res = getStatus(config.testAddr[1]);
      logger.debug('\nthe status of the node:\n', res);
      assert.equal(res, 1);
    });
  });

  describe('\ntest approve node\n', () => {
    before('should be ready status', () => {
      const res = getStatus(config.testAddr[1]);
      logger.debug('\nthe status of the node:\n', res);
      assert.equal(res, 1);
    });

    it('should send a approveNode tx and get receipt', (done) => {
      const res = approveNode(config.testAddr[1], admin);

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get approveNode receipt err:!!!!\n', err);
          this.skip();
        });
    });

    it('should be start status', () => {
      const res = getStatus(config.testAddr[1]);
      logger.debug('\nthe status of the node:\n', res);
      assert.equal(res, 2);
    });

    it('should have the new consensus node', () => {
      const res = listNode();
      logger.debug('\nthe consensus nodes:\n', res);
      assert.equal(res[res.length - 1], config.testAddr[1]);
    });
  });

  describe('\ntest delete consensus node\n', () => {
    before('should be ready status and wait a new block', (done) => {
      const res = getStatus(config.testAddr[1]);
      logger.debug('\nthe status of the node:\n', res);
      assert.equal(res, 2);
      const num = web3.eth.blockNumber;
      let tmp;
      do {
        tmp = web3.eth.blockNumber;
      } while (tmp <= num);
      done();
    });

    it('should send a deleteNode tx and get receipt', (done) => {
      const res = deleteNode(config.testAddr[1], admin);

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get deleteNode receipt err:!!!!\n', err);
          this.skip();
        });
    });

    it('should be close status', () => {
      const res = getStatus(config.testAddr[1]);
      logger.debug('\nthe status of the node:\n', res);
      assert.equal(res, 0);
    });
  });
});
