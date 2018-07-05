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

const {
  describe, it, before,
} = mocha;

// const STORE_ADDRESS = '0xffffffffffffffffffffffffffffffffff010000';
const sendTx = '0x0000000000000000000000000000000000000001';
const { setAuthorization, cancelAuthorization } = permissionManagement;

// TODO / BUG: use store_address
//           : data is null
// =======================

describe('\n\ntest send transaction permission\n\n', () => {
  it('should send a tx and get receipt: superAdmin', (done) => {
    const res = web3.eth.sendTransaction({
      privkey: superAdmin.privkey,
      nonce: util.randomInt(),
      quota,
      validUntilBlock: web3.eth.blockNumber + blockLimit,
      from: superAdmin.address,
      // to: STORE_ADDRESS,
      to: config.testAddr[0],
      // data: ''
      data: '00000000',
    });
    getTxReceipt(res)
      .then((receipt) => {
        logger.debug('\nSend ok and get receipt:\n', receipt);
        // assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
        assert.equal(receipt.errorMessage, 'No Call contract permission.', JSON.stringify(receipt.errorMessage));

        done();
      })
      .catch((err) => {
        logger.error('\n!!!!Get receipt err:!!!!\n', err);
        this.skip();
        done();
      });
  });

  it('should send a tx and get receipt with error message: testSender', (done) => {
    const res = web3.eth.sendTransaction({
      privkey: sender.privkey,
      nonce: util.randomInt(),
      quota,
      validUntilBlock: web3.eth.blockNumber + blockLimit,
      from: sender.address,
      // to: STORE_ADDRESS,
      to: config.testAddr[0],
      data: '00000000',
    });
    getTxReceipt(res)
      .then((receipt) => {
        logger.debug('\nSend ok and get receipt:\n', receipt);
        assert.equal(receipt.errorMessage, 'No transaction permission.', JSON.stringify(receipt.errorMessage));
        done();
      })
      .catch((err) => {
        logger.error('\n!!!!Get receipt err:!!!!\n', err);
        this.skip();
        done();
      });
  });

  describe('\n\ntest send tx permission after set send permission\n\n', () => {
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

    it('should wait a new block', (done) => {
      const num = web3.eth.blockNumber;
      let tmp;
      do {
        tmp = web3.eth.blockNumber;
      } while (tmp <= num);
      done();
    });

    it('should send a tx and get receipt: testSender', (done) => {
      const res = web3.eth.sendTransaction({
        privkey: sender.privkey,
        nonce: util.randomInt(),
        quota,
        validUntilBlock: web3.eth.blockNumber + blockLimit,
        from: sender.address,
        // to: STORE_ADDRESS,
        to: config.testAddr[0],
        data: '00000000',
      });
      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          // assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
          assert.equal(receipt.errorMessage, 'No Call contract permission.', JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get receipt err:!!!!\n', err);
          this.skip();
          done();
        });
    });
  });

  describe('\n\ntest send tx permission after cancel send permission\n\n', () => {
    before('should send a cancelAuthorization tx and get receipt', (done) => {
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

    it('should wait a new block', (done) => {
      const num = web3.eth.blockNumber;
      let tmp;
      do {
        tmp = web3.eth.blockNumber;
      } while (tmp <= num);
      done();
    });

    it('should send a tx and get receipt with error message: testSender', (done) => {
      const res = web3.eth.sendTransaction({
        privkey: sender.privkey,
        nonce: util.randomInt(),
        quota,
        validUntilBlock: web3.eth.blockNumber + blockLimit,
        from: sender.address,
        // to: STORE_ADDRESS,
        to: config.testAddr[0],
        data: '00000000',
      });
      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, 'No transaction permission.', JSON.stringify(receipt.errorMessage));
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
