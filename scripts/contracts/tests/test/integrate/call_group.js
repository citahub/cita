const mocha = require('mocha');
const assert = require('assert');
const util = require('../helpers/util');
const groupManagement = require('../helpers/group_management');
const permissionManagement = require('../helpers/permission_management');
const config = require('../config');

// util
const { logger, getTxReceipt } = util;

// config
const sender = config.testSender;

// group management
const {
  newGroup, deleteGroup, queryGroups,
} = groupManagement;

// permission management
const { setAuthorization } = permissionManagement;

const { describe, it, before } = mocha;

// temp
let newGroupAddr;
let newGroupAddr2;
let lengthOfGroups;

const deleteGroupPermission = '0xffffffffffffffffffffffffffffffffff02101b';
const rootGroupAddr = '0xffffffffffffffffffffffffffffffffff020009';

// Only deleteGroup
// =======================

describe('\n\nintegrate test group: \n\n', () => {
  before('Query the number of the groups', () => {
    const res = queryGroups.call();
    logger.debug('\nThe groups:\n', res);
    lengthOfGroups = res.length;
  });

  it('should send a newGroup tx and get receipt', (done) => {
    const name = 'testGroup';
    const res = newGroup(rootGroupAddr, name, [sender.address]);

    getTxReceipt(res)
      .then((receipt) => {
        logger.debug('\nSend ok and get receipt:\n', receipt);
        assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
        newGroupAddr = receipt.logs[0].address;
        logger.debug('\nThe new permission contract address:\n', newGroupAddr);
        done();
      })
      .catch((err) => {
        logger.error('\n!!!!Get newGroup receipt err:!!!!\n', err);
        this.skip();
      });
  });

  it('should send another newGroup tx and get receipt', (done) => {
    const name = 'testGroup2';
    const res = newGroup(rootGroupAddr, name, [sender.address]);

    getTxReceipt(res)
      .then((receipt) => {
        logger.debug('\nSend ok and get receipt:\n', receipt);
        assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
        newGroupAddr2 = receipt.logs[0].address;
        logger.debug('\nThe new permission contract address:\n', newGroupAddr2);
        done();
      })
      .catch((err) => {
        logger.error('\n!!!!Get newGroup receipt err:!!!!\n', err);
        this.skip();
      });
  });

  it('should have more groups', () => {
    const res = queryGroups.call();
    logger.debug('\nThe groups:\n', res);
    assert.equal(res.length, lengthOfGroups + 2);
    assert.equal(res[res.length - 1], newGroupAddr2);
    assert.equal(res[res.length - 2], newGroupAddr);
  });

  it('should send a deleteGroup tx and get receipt with errormessage: No contract permission. origin: newGroup', (done) => {
    const res = deleteGroup(newGroupAddr, newGroupAddr, sender);

    getTxReceipt(res)
      .then((receipt) => {
        logger.debug('\nSend ok and get receipt:\n', receipt);
        assert.equal(receipt.errorMessage, 'No Call contract permission.', JSON.stringify(receipt.errorMessage));
        done();
      })
      .catch((err) => {
        logger.error('\n!!!!Get deleteGroup receipt err:!!!!\n', err);
        this.skip();
      });
  });

  it('should send a setAuthorization tx and get receipt: newGroup', (done) => {
    const res = setAuthorization(newGroupAddr, deleteGroupPermission);

    getTxReceipt(res)
      .then((receipt) => {
        logger.debug('\nSend ok and get receipt:\n', receipt);
        assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
        done();
      })
      .catch((err) => {
        logger.error('\n!!!!Get setAuthorization receipt err:!!!!\n', err);
        this.skip();
      });
  });

  it('should send a setAuthorization tx and get receipt: rootGroup', (done) => {
    const res = setAuthorization(rootGroupAddr, deleteGroupPermission);

    getTxReceipt(res)
      .then((receipt) => {
        logger.debug('\nSend ok and get receipt:\n', receipt);
        assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
        done();
      })
      .catch((err) => {
        logger.error('\n!!!!Get setAuthorization receipt err:!!!!\n', err);
        this.skip();
      });
  });

  it('should send a deleteGroup tx and get receipt with errormessage: reverted. origin: newGroup', (done) => {
    const res = deleteGroup(newGroupAddr, rootGroupAddr, sender);

    getTxReceipt(res)
      .then((receipt) => {
        logger.debug('\nSend ok and get receipt:\n', receipt);
        assert.equal(receipt.errorMessage, 'Reverted.', JSON.stringify(receipt.errorMessage));
        done();
      })
      .catch((err) => {
        logger.error('\n!!!!Get deleteGroup receipt err:!!!!\n', err);
        this.skip();
      });
  });

  it('should send a deleteGroup tx and get receipt with errormessage: No contract permission. origin: newGroup', (done) => {
    const res = deleteGroup(newGroupAddr2, newGroupAddr, sender);

    getTxReceipt(res)
      .then((receipt) => {
        logger.debug('\nSend ok and get receipt:\n', receipt);
        assert.equal(receipt.errorMessage, 'No Call contract permission.', JSON.stringify(receipt.errorMessage));
        done();
      })
      .catch((err) => {
        logger.error('\n!!!!Get deleteGroup receipt err:!!!!\n', err);
        this.skip();
      });
  });

  it('should send a deleteGroup tx and get receipt. origin: newGroup', (done) => {
    const res = deleteGroup(newGroupAddr, newGroupAddr, sender);

    getTxReceipt(res)
      .then((receipt) => {
        logger.debug('\nSend ok and get receipt:\n', receipt);
        assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
        done();
      })
      .catch((err) => {
        logger.error('\n!!!!Get deleteGroup receipt err:!!!!\n', err);
        this.skip();
      });
  });

  it('should have less groups', () => {
    const res = queryGroups.call();
    logger.debug('\nThe groups:\n', res);
    assert.equal(res.length, lengthOfGroups + 1);
  });

  it('should send a setAuthorization tx and get receipt: rootGroup', (done) => {
    const res = setAuthorization(rootGroupAddr, deleteGroupPermission);

    getTxReceipt(res)
      .then((receipt) => {
        logger.debug('\nSend ok and get receipt:\n', receipt);
        assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
        done();
      })
      .catch((err) => {
        logger.error('\n!!!!Get setAuthorization receipt err:!!!!\n', err);
        this.skip();
      });
  });

  it('should send a deleteGroup tx and get receipt. origin: root ', (done) => {
    const res = deleteGroup(rootGroupAddr, newGroupAddr2, sender);

    getTxReceipt(res)
      .then((receipt) => {
        logger.debug('\nSend ok and get receipt:\n', receipt);
        assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
        done();
      })
      .catch((err) => {
        logger.error('\n!!!!Get deleteGroup receipt err:!!!!\n', err);
        this.skip();
      });
  });

  it('should have less groups', () => {
    const res = queryGroups.call();
    logger.debug('\nThe groups:\n', res);
    assert.equal(res.length, lengthOfGroups);
  });
});
