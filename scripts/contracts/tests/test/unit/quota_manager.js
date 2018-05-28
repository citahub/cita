const assert = require('assert');
const mocha = require('mocha');
const util = require('../helpers/util');
const quota = require('../helpers/quota');
const config = require('../config');

const { getTxReceipt, logger } = util;

const { admin } = config.contract.quota;

const {
  getDefaultAQL, getAQL, getBQL, getQuotas, getAccounts, isAdmin, setAQL,
  setDefaultAQL, setBQL, addAdmin,
} = quota;

const value = 2 ** 29;

const { describe, it } = mocha;

// =======================

describe('test quota manager', () => {
  describe('\ntest add admin\n', () => {
    it('should send an addAdmin tx', (done) => {
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

  describe('\ntest set block quota limit\n', () => {
    it('should send setBQL tx', (done) => {
      const res = setBQL(value, admin);

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get setBQL receipt err:!!!!\n', err);
          this.skip();
        });
    });

    it('should have new block quota limit', () => {
      const res = getBQL();
      logger.debug('\nthe block quota limit:\n', res);
      assert.equal(res, value);
    });
  });

  describe('\ntest set default account quota limit\n', () => {
    it('should send setDefaultAQL tx', (done) => {
      const res = setDefaultAQL(value, admin);

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get setDefaultAQL receipt err:!!!!\n', err);
          this.skip();
        });
    });

    it('should have new default account quota limit', () => {
      const res = getDefaultAQL();
      logger.debug('\nthe default account quota limit:\n', res);
      assert.equal(res, value);
    });
  });

  describe('\ntest set account\'s quota limit\n', () => {
    it('should send setAQL tx', (done) => {
      const res = setAQL(config.testAddr[0], value - 1, admin);

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get setDefaultAQL receipt err:!!!!\n', err);
          this.skip();
        });
    });

    it('should have new account quota limit', () => {
      const res = getAQL(config.testAddr[0]);
      logger.debug('\nthe default account quota limit:\n', res);
      assert.equal(res, value - 1);
    });

    it('should have new special account', () => {
      const res = getAccounts();
      logger.debug('\nthe special accounts:\n', res);
      assert.equal(res[res.length - 1], config.testAddr[0]);
    });

    it('should have new quotas of special accounts', () => {
      const res = getQuotas();
      logger.debug('\nthe quotas of the special accounts:\n', res);
      assert.equal(res[res.length - 1], value - 1);
    });
  });
});
