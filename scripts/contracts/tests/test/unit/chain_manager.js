const util = require('../helpers/util');
const chainManager = require('../helpers/chain_manager');
const config = require('../config');
const chai = require('chai');

const { expect } = chai;
const { logger, getTxReceipt } = util;
const {
  newSideChain, enableSideChain, disableSideChain, getChainId, getParentChainId,
} = chainManager;

// TODO Add query interface and event of chain_manager.sol
// TODO Check the new sideChain

// temp
let chainId;
let hash;

describe('test side chain management contract', () => {
  describe('\ntest register new side chain\n', () => {
    // TODO fixit use getMetaData
    before('Query the parent chain id ', async () => {
      const res = await getParentChainId();
      logger.debug('\nThe parent chain id:\n', res);
      expect(res).to.equal('0');
    });

    before('Query the chain id ', async () => {
      const res = await getChainId();
      logger.debug('\nThe chain id:\n', res);
      expect(res).to.equal('1');
    });

    it('should send a tx: newSideChain', async () => {
      const res = await newSideChain(2, config.testAddr);
      logger.debug('\nSend tx ok:\n', JSON.stringify(res));
      expect(res.status).to.equal('OK');
      ({ hash } = res);
    });

    it('should get receipt: newSideChain ', async () => {
      const res = await getTxReceipt(hash);
      logger.debug('\nget receipt:\n', res);
      expect(res.errorMessage).to.be.null;
    });
  });

  describe('\ntest enable side chain\n', () => {
    before('Query the side chain id ', async () => {
      chainId = await getChainId();
      logger.debug('\nThe side chain id:\n', chainId);
    });

    it('should send a tx: enableSideChain', async () => {
      // let res = enableSideChain(chainId);
      const res = await enableSideChain(2);
      logger.debug('\nSend tx ok:\n', res);
      expect(res.status).to.equal('OK');
      ({ hash } = res);
    });

    it('should get receipt: enableSideChain', async () => {
      const res = await getTxReceipt(hash);
      logger.debug('\nGet receipt:\n', res);
      expect(res.errorMessage).to.be.null;
    });
  });

  describe('\ntest disable side chain\n', () => {
    it('should send a tx: disableSideChain', async () => {
      // let res = disableSideChain(chainId);
      const res = await disableSideChain(2);
      logger.debug('\nSend ok:\n', res);
      expect(res.status).to.equal('OK');
      ({ hash } = res);
    });

    it('should get receipt: disableSideChain ', async () => {
      const res = await getTxReceipt(hash);
      logger.debug('\nGet receipt:\n', res);
      expect(res.errorMessage).to.be.null;
    });
  });
});
