const chai = require('chai');
const util = require('../helpers/util');
const quota = require('../helpers/quota');
const config = require('../config');

const { expect } = chai;
const { getTxReceipt, logger } = util;
const { superAdmin } = config;
const {
  getDefaultAQL, getAQL, getBQL, getQuotas, getAccounts, setAQL,
  setDefaultAQL, setBQL,
} = quota;

// temp
let hash;
let lenOfAccounts;
let lenOfQuotas;

// test data
const value = '536870912';
const value2 = '536870911';
const admin = superAdmin;
const addr = config.testAddr[0];

describe('test quota manager', () => {
  describe('\ntest set block quota limit\n', () => {
    it('should send a tx: setBQL ', async () => {
      const res = await setBQL(value, admin);
      logger.debug('\nSend tx ok:\n', JSON.stringify(res));
      expect(res.status).to.equal('OK');
      ({ hash } = res);
    });

    it('should get receipt: setBQL', async () => {
      const res = await getTxReceipt(hash);
      logger.debug('\nget receipt:\n', res);
      expect(res.errorMessage).to.be.null;
    });

    it('should have new block quota limit', async () => {
      const res = await getBQL();
      logger.debug('\nthe block quota limit:\n', res);
      expect(res).to.equal(value);
    });
  });

  describe('\ntest set default account quota limit\n', () => {
    it('should send a tx: setDefaultAQL ', async () => {
      const res = await setDefaultAQL(value, admin);
      logger.debug('\nSend tx ok:\n', JSON.stringify(res));
      expect(res.status).to.equal('OK');
      ({ hash } = res);
    });

    it('should get receipt: setDefaultAQL', async () => {
      const res = await getTxReceipt(hash);
      logger.debug('\nget receipt:\n', res);
      expect(res.errorMessage).to.be.null;
    });

    it('should have new default account quota limit', async () => {
      const res = await getDefaultAQL();
      logger.debug('\nthe default account quota limit:\n', res);
      expect(res).to.equal(value);
    });
  });

  describe('\ntest set account\'s quota limit\n', () => {
    it('should send a tx: setAQL ', async () => {
      const res = await setAQL(addr, value2, admin);
      logger.debug('\nSend tx ok:\n', JSON.stringify(res));
      expect(res.status).to.equal('OK');
      ({ hash } = res);
    });

    it('should get receipt: setAQL', async () => {
      const res = await getTxReceipt(hash);
      logger.debug('\nget receipt:\n', res);
      expect(res.errorMessage).to.be.null;
    });

    it('should have new account quota limit', async () => {
      const res = await getAQL(addr);
      logger.debug('\nthe default account quota limit:\n', res);
      expect(res).to.equal(value2);
    });

    it('should have new special account', async () => {
      const res = await getAccounts();
      logger.debug('\nthe special accounts:\n', res);
      expect(addr).to.be.oneOf(res);
    });

    it('should have new quotas of special accounts', async () => {
      const res = await getQuotas();
      logger.debug('\nthe quotas of the special accounts:\n', res);
      expect(value2).to.be.oneOf(res);
    });
  });

  describe('\ntest set duplicate account\'s quota limit\n', () => {
    before('set duplicate AQL', async () => {
      let res = await getAccounts();
      logger.debug('\nthe special accounts:\n', res);
      lenOfAccounts = res.length;
      res = await getQuotas();
      logger.debug('\nthe special accounts\' quotas:\n', res);
      lenOfQuotas = res.length;
    });

    it('should send a tx: setAQL ', async () => {
      const res = await setAQL(addr, value, admin);
      logger.debug('\nSend tx ok:\n', JSON.stringify(res));
      expect(res.status).to.equal('OK');
      ({ hash } = res);
    });

    it('should get receipt: setAQL', async () => {
      const res = await getTxReceipt(hash);
      logger.debug('\nget receipt:\n', res);
      expect(res.errorMessage).to.be.null;
    });

    it('should have new account quota limit', async () => {
      const res = await getAQL(addr);
      logger.debug('\nthe default account quota limit:\n', res);
      expect(res).to.equal(value);
    });

    it('should have equal length of special account', async () => {
      const res = await getAccounts();
      logger.debug('\nthe special accounts:\n', res);
      expect(res).to.have.lengthOf(lenOfAccounts);
    });

    it('should have equla lengtho of quotas', async () => {
      const res = await getQuotas();
      logger.debug('\nthe quotas of the special accounts:\n', res);
      expect(res).to.have.lengthOf(lenOfQuotas);
    });
  });
});
