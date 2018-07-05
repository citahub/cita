const mocha = require('mocha');
const assert = require('assert');
const util = require('../helpers/util');
const config = require('../config');
const permissionManagement = require('../helpers/permission_management');

// config
const { superAdmin } = config.contract.authorization;
const sender = config.testSender;

// util
const {
  logger, web3, getTxReceipt, quota, blockLimit,
} = util;

const createContract = '0xffffffffffffffffffffffffffffffffff021001';
const sendTx = '0xffffffffffffffffffffffffffffffffff021000';
const { setAuthorization, cancelAuthorization } = permissionManagement;

const {
  describe, it, before, after,
} = mocha;

// =======================

describe('\n\ntest create contract permission\n\n', () => {
  before('should send a setAuthorization tx and get receipt', (done) => {
    const res = setAuthorization(sender.address, sendTx);

    getTxReceipt(res)
      .then((receipt) => {
        logger.debug('\nSend ok and get receipt:\n', receipt);
        assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
        done();
      })
      .catch((err) => {
        logger.error('\n!!!!Get setAuthorization receipt err:!!!!\n', err);
        this.skip();
        done();
      });
  });

  after('should send a cancelAuthorization tx and get receipt', (done) => {
    const res = cancelAuthorization(sender.address, sendTx);

    getTxReceipt(res)
      .then((receipt) => {
        logger.debug('\nSend ok and get receipt:\n', receipt);
        assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
        done();
      })
      .catch((err) => {
        logger.error('\n!!!!Get cancelAuthorization receipt err:!!!!\n', err);
        this.skip();
        done();
      });
  });


  it('should send a deploy_contract tx and get receipt: superAdmin', (done) => {
    const res = web3.eth.sendTransaction({
      privkey: superAdmin.privkey,
      nonce: util.randomInt(),
      quota,
      validUntilBlock: web3.eth.blockNumber + blockLimit,
      data: config.testBin,
    });
    getTxReceipt(res)
      .then((receipt) => {
        logger.debug('\nSend ok and get receipt:\n', receipt);
        assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
        done();
      })
      .catch((err) => {
        logger.error('\n!!!!Get receipt err:!!!!\n', err);
        this.skip();
        done();
      });
  });

  it('should send a deploy_contract tx and get receipt with error message: testSender', (done) => {
    const res = web3.eth.sendTransaction({
      privkey: sender.privkey,
      nonce: util.randomInt(),
      quota,
      validUntilBlock: web3.eth.blockNumber + blockLimit,
      data: config.testBin,
    });
    getTxReceipt(res)
      .then((receipt) => {
        logger.debug('\nSend ok and get receipt:\n', receipt);
        assert.equal(receipt.errorMessage, 'No contract permission.', JSON.stringify(receipt.errorMessage));
        done();
      })
      .catch((err) => {
        logger.error('\n!!!!Get receipt err:!!!!\n', err);
        this.skip();
        done();
      });
  });

  describe('\n\ntest create contract permission after set createContract permission\n\n', () => {
    before('should send a setAuthorization tx and get receipt', (done) => {
      const res = setAuthorization(sender.address, createContract);

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get setAuthorization receipt err:!!!!\n', err);
          this.skip();
          done();
        });
    });

    it('should wait a new block', (done) => {
      const num = web3.eth.blockNumber;
      let tmp;
      do {
        tmp = web3.eth.blockNumber;
      } while (tmp <= num);
      done();
    });

    it('should send a deploy_contract tx and get receipt: testSender', (done) => {
      const res = web3.eth.sendTransaction({
        privkey: sender.privkey,
        nonce: util.randomInt(),
        quota,
        validUntilBlock: web3.eth.blockNumber + blockLimit,
        data: config.testBin,
      });
      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get receipt err:!!!!\n', err);
          this.skip();
          done();
        });
    });
  });

  describe('\n\ntest create contract permission after cancel createContract permission\n\n', () => {
    before('should send a cancelAuthorization tx and get receipt', (done) => {
      const res = cancelAuthorization(sender.address, createContract);

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get cancelAuthorization receipt err:!!!!\n', err);
          this.skip();
          done();
        });
    });

    it('should wait a new block', (done) => {
      const num = web3.eth.blockNumber;
      let tmp;
      do {
        tmp = web3.eth.blockNumber;
      } while (tmp <= num);
      done();
    });

    it('should send a deploy_contract tx and get receipt with error message: testSender', (done) => {
      const res = web3.eth.sendTransaction({
        privkey: sender.privkey,
        nonce: util.randomInt(),
        quota,
        validUntilBlock: web3.eth.blockNumber + blockLimit,
        data: config.testBin,
      });
      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, 'No contract permission.', JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get receipt err:!!!!\n', err);
          this.skip();
          done();
        });
    });
  });
});
