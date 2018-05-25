const assert = require('assert');
const mocha = require('mocha');
const util = require('../helpers/util');
const roleManagement = require('../helpers/role_management');
const config = require('../config');

// util
const { web3, getTxReceipt, logger } = util;
const { rABI } = config.contract.role;

// config
const sender = config.testSender;

// role management
const { cancelAuthorization, setAuthorization, newRole } = roleManagement;

// role
const role = web3.eth.contract(rABI);

const sendTx = '0x0000000000000000000000000000000000000001';
const permissions = [sendTx];
const newRolePermission = '0x00000000000000000000000000000000063241b5';

// temp
let newRoleAddr;

const {
  describe, it, before, after,
} = mocha;

// Only newRole
// =======================

describe('\n\nintegrate test role: \n\n', () => {
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

  describe('\n\ntest new role before setted auth:\n\n', () => {
    it('should wait a new block', (done) => {
      const num = web3.eth.blockNumber;
      let tmp;
      do {
        tmp = web3.eth.blockNumber;
      } while (tmp <= num);
      done();
    });

    it('should send a newRole tx and get receipt', (done) => {
      const res = newRole('new_sendTx', permissions, sender);
      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, 'No Call contract permission.', JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get newRole receipt err:!!!!\n', err);
          this.skip();
          done();
        });
    });
  });

  describe('\n\ntest new role after setted auth:\n\n', () => {
    before('should send a setAuthorization tx and get receipt: grant the newRole permissiont to sender', (done) => {
      const res = setAuthorization(sender.address, newRolePermission);

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

    it('should send a newRole tx and get receipt', (done) => {
      const res = newRole('newRole', permissions, sender);
      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
          newRoleAddr = receipt.logs[0].address;
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get newRole receipt err:!!!!\n', err);
          this.skip();
          done();
        });
    });

    it('should have info of new role', () => {
      const roleInstance = role.at(newRoleAddr);
      const res = roleInstance.queryRole.call();
      logger.debug('\nInfo:\n', res);
      assert.equal(res[0].substr(0, 18), web3.toHex('newRole'));

      for (let i = 0; i < res[1].length; i += 1) {
        assert.equal(res[1][i], permissions[i]);
      }
    });
  });

  describe('\n\ntest newRole after cancel auth:\n\n', () => {
    before('should send a cancelAuthorization tx and get receipt: cancel the newRole permissiont of sender', (done) => {
      const res = cancelAuthorization(sender.address, newRolePermission);

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

    it('should send a newRole tx and get receipt', (done) => {
      const res = newRole('newRole_2', permissions, sender);
      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, 'No Call contract permission.', JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get newRole receipt err:!!!!\n', err);
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
