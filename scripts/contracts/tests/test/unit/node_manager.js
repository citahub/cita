const util = require('../helpers/util');
const nodeManager = require('../helpers/node_manager');
const config = require('../config');
const chai = require('chai');

const { expect } = chai;
const {
  logger, getTxReceipt,
} = util;
const {
  getStatus, listNode, deleteNode, approveNode,
} = nodeManager;
const { testAddr } = config;

// temo
let hash;

// test data
const addr = testAddr[0];
const addr1 = testAddr[1];

describe('\n\ntest node manager\n\n', () => {
  describe('\ntest approve node\n', () => {
    before('should be close status', async () => {
      const res = await getStatus(addr);
      logger.debug('\nthe status of the node:\n', res);
      expect(res).to.equal('0');
    });

    it('should send a tx: approveNode', async () => {
      const res = await approveNode(addr1);
      logger.debug('\nSend tx ok:\n', JSON.stringify(res));
      expect(res.status).to.equal('OK');
      ({ hash } = res);
    });

    it('should get receipt: approveNode', async () => {
      const res = await getTxReceipt(hash);
      logger.debug('\nget receipt:\n', res);
      expect(res.errorMessage).to.be.null;
    });

    it('should be start status', async () => {
      const res = await getStatus(addr1);
      logger.debug('\nthe status of the node:\n', res);
      expect(res).to.equal('1');
    });

    it('should have the new consensus node', async () => {
      const res = await listNode();
      logger.debug('\nthe consensus nodes:\n', res);
      expect(addr1).to.be.oneOf(res);
    });
  });

  describe('\ntest delete consensus node\n', () => {
    before('should be start status and wait a new block', async () => {
      const res = await getStatus(addr1);
      logger.debug('\nthe status of the node:\n', res);
      expect(res).to.equal('1');
      setTimeout(() => {}, 10000);
    });

    it('should send a tx: deleteNode', async () => {
      const res = await deleteNode(addr1);
      logger.debug('\nSend tx ok:\n', JSON.stringify(res));
      expect(res.status).to.equal('OK');
      ({ hash } = res);
    });

    it('should get receipt: deleteNode', async () => {
      const res = await getTxReceipt(hash);
      logger.debug('\nget receipt:\n', res);
      expect(res.errorMessage).to.be.null;
    });

    it('should be close status', async () => {
      const res = await getStatus(addr1);
      logger.debug('\nthe status of the node:\n', res);
      expect(res).to.equal('0');
    });
  });
});
