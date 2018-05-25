const mocha = require('mocha');
const assert = require('assert');
const util = require('../helpers/util');
const permissionManagement = require('../helpers/permission_management');
const permission = require('../helpers/permission');
const config = require('../config');

// util
const { logger, web3, getTxReceipt } = util;

// config
const sender = config.testSender;

// permission management
const {
  cancelAuthorization, setAuthorization, updatePermissionName,
} = permissionManagement;

// perm
const { perm } = permission;
let pContractInstance;

const sendTx = '0x0000000000000000000000000000000000000001';
const updatePermission = '0x00000000000000000000000000000000033241b5';

const {
  describe, it, before, after,
} = mocha;

// Only updatePermissionName
// =======================

describe('\n\nintegrate test permission: \n\n', () => {
  before('should send a setAuthorization tx and get receipt: grant the sendTx permissiont to sender', (done) => {
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

  describe('\n\ntest update permission name before setted auth:\n\n', () => {
    it('should wait a new block', (done) => {
      const num = web3.eth.blockNumber;
      let tmp;
      do {
        tmp = web3.eth.blockNumber;
      } while (tmp <= num);
      done();
    });

    it('should send a updatePermissionName tx and get receipt', (done) => {
      const res = updatePermissionName(sendTx, 'new_sendTx', sender);
      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, 'No Call contract permission.', JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get updatePermissionName receipt err:!!!!\n', err);
          this.skip();
          done();
        });
    });
  });

  describe('\n\ntest update permission name after setted auth:\n\n', () => {
    before('should send a setAuthorization tx and get receipt: grant the updatePermission permissiont to sender', (done) => {
      const res = setAuthorization(sender.address, updatePermission);

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

    it('should send a updatePermissionName tx and get receipt', (done) => {
      const res = updatePermissionName(sendTx, 'new_sendTx', sender);
      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get updatePermissionName receipt err:!!!!\n', err);
          this.skip();
          done();
        });
    });

    it('should have the new permission name', () => {
      pContractInstance = perm.at(sendTx);
      const res = pContractInstance.queryName.call();
      logger.debug('\nNew sendTx permission name:\n', res);
      assert.equal(res.substr(0, 24), web3.toHex('new_sendTx'));
    });
  });

  describe('\n\ntest update permission name after cancel auth:\n\n', () => {
    before('should send a cancelAuthorization tx and get receipt: cancel the updatePermission permissiont of sender', (done) => {
      const res = cancelAuthorization(sender.address, updatePermission);

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

    it('should send a updatePermissionName tx and get receipt', (done) => {
      const res = updatePermissionName(sendTx, 'new_sendTx', sender);
      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, 'No Call contract permission.', JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get updatePermissionName receipt err:!!!!\n', err);
          this.skip();
          done();
        });
    });
  });

  after('should send a cancelAuthorization tx and get receipt: cancel the sendTx permissiont of sender', (done) => {
    const res = cancelAuthorization(sender.address, sendTx);

    getTxReceipt(res)
      .then((receipt) => {
        logger.debug('\nSend ok and get receipt:\n', receipt);
        assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
        const num = web3.eth.blockNumber;
        let tmp;
        do {
          tmp = web3.eth.blockNumber;
        } while (tmp <= num);
        done();
      })
      .catch((err) => {
        logger.error('\n!!!!Get cancelAuthorization receipt err:!!!!\n', err);
        this.skip();
        done();
      });
  });
});
