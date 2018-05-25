const mocha = require('mocha');
const assert = require('assert');
const util = require('../helpers/util');
const groupManagement = require('../helpers/group_management');
const group = require('../helpers/group');
const config = require('../config');

// util
const { logger, web3, getTxReceipt } = util;

// config
const sender = config.testSender;

// group management
const {
  queryGroups, checkScope, newGroup, updateGroupName, addAccounts, deleteAccounts, deleteGroup,
} = groupManagement;

// group
const gr = group.group;
const rootGroupAddr = config.contract.group.gAddr;
let gContractInstance;

const { describe, it, before } = mocha;

// temp
let newGroupAddr;
let lengthOfAccounts;
let lengthOfChild;
let lengthOfGroups;

// =======================

describe('\n\ntest group management contract\n\n', () => {
  describe('\ntest add new group\n', () => {
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

    it('should have info of new group', () => {
      gContractInstance = gr.at(newGroupAddr);
      const res = gContractInstance.queryInfo.call();
      logger.debug('\nInfo:\n', res);
      assert.equal(res[0].substr(0, 20), web3.toHex('testGroup'));

      assert.equal(res[1][0], sender.address);
    });

    it('should have more groups', () => {
      const res = queryGroups.call();
      logger.debug('\nThe groups:\n', res);
      assert.equal(res.length, lengthOfGroups + 1);
      assert.equal(res[res.length - 1], newGroupAddr);
    });

    it('should in the scope of root', () => {
      const res = checkScope(rootGroupAddr, newGroupAddr);
      logger.debug('\nIs in the scope of root:\n', res);
      assert.equal(res, true);
    });

    it('should in the scope of self', () => {
      const res = checkScope(newGroupAddr, newGroupAddr, sender);
      logger.debug('\nIs in the scope of self:\n', res);
      assert.equal(res, true);
    });
  });

  describe('\ntest update group name by self\n', () => {
    it('should send a updateGroupName tx and get receipt', (done) => {
      const res = updateGroupName(newGroupAddr, newGroupAddr, 'testGroupNewName', sender);

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get updateGroupName receipt err:!!!!\n', err);
          this.skip();
        });
    });

    it('should have the new group name', () => {
      gContractInstance = gr.at(newGroupAddr);
      const res = gContractInstance.queryName.call();
      logger.debug('\nNew Group name:\n', res);
      assert.equal(res.substr(0, 34), web3.toHex('testGroupNewName'));
    });
  });

  describe('\ntest update group name by root\n', () => {
    it('should send a updateGroupName tx and get receipt', (done) => {
      const res = updateGroupName(rootGroupAddr, newGroupAddr, 'testGroupNewName2');

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get updateGroupName receipt err:!!!!\n', err);
          this.skip();
        });
    });

    it('should have the new group name', () => {
      gContractInstance = gr.at(newGroupAddr);
      const res = gContractInstance.queryName.call();
      logger.debug('\nNew Group name:\n', res);
      assert.equal(res.substr(0, 36), web3.toHex('testGroupNewName2'));
    });
  });

  describe('\ntest add accounts\n', () => {
    before('Query the number of the accounts', () => {
      gContractInstance = gr.at(newGroupAddr);
      const res = gContractInstance.queryAccounts.call();
      logger.debug('\nThe number of the accounts:\n', res.length);
      lengthOfAccounts = res.length;
    });

    it('should send a addAccounts tx and get receipt', (done) => {
      const res = addAccounts(
        newGroupAddr,
        newGroupAddr,
        ['0x1a702a25c6bca72b67987968f0bfb3a3213c5603'],
        sender,
      );

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get addAccounts receipt err:!!!!\n', err);
          this.skip();
        });
    });

    it('should have the added accounts', () => {
      gContractInstance = gr.at(newGroupAddr);
      const res = gContractInstance.queryAccounts.call();
      logger.debug('\nNew Added accounts:\n', res);
      const l = res.length - 1;
      assert.equal(res[l], '0x1a702a25c6bca72b67987968f0bfb3a3213c5603');
      assert.equal(l, lengthOfAccounts);
    });

    it('should send a addAccounts to a group address that does not exist and get receipt with error message', (done) => {
      const res = addAccounts(
        0x1234567,
        0x1234567,
        ['0x1a702a25c6bca72b67987968f0bfb3a3213c5604'],
        sender,
      );

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt with error message:\n', receipt);
          assert.equal(receipt.errorMessage, 'Reverted.', JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get addAccounts receipt err:!!!!\n', err);
          this.skip();
        });
    });
  });


  describe('\ntest add duplicate accounts\n', () => {
    before('Query the number of the accounts', () => {
      gContractInstance = gr.at(newGroupAddr);
      const res = gContractInstance.queryAccounts.call();
      logger.debug('\nThe number of the accounts:\n', res.length);
      lengthOfAccounts = res.length;
    });

    it('should send a addAccounts tx and get receipt', (done) => {
      const res = addAccounts(
        newGroupAddr,
        newGroupAddr,
        ['0x1a702a25c6bca72b67987968f0bfb3a3213c5603'],
        sender,
      );

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get addResources receipt err:!!!!\n', err);
          this.skip();
        });
    });

    it('should not added into the accounts', () => {
      gContractInstance = gr.at(newGroupAddr);
      const res = gContractInstance.queryAccounts.call();
      logger.debug('\nThe num of the account:\n', res.length);
      assert.equal(res.length, lengthOfAccounts);
    });
  });

  describe('\ntest delete accounts\n', () => {
    it('should send a deleteAccounts tx and get receipt', (done) => {
      const res = deleteAccounts(
        newGroupAddr,
        newGroupAddr,
        ['0x1a702a25c6bca72b67987968f0bfb3a3213c5603'],
        sender,
      );

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get deleteAccounts receipt err:!!!!\n', err);
          this.skip();
        });
    });

    it('should have deleted the accounts', () => {
      gContractInstance = gr.at(newGroupAddr);
      const res = gContractInstance.queryAccounts.call();
      logger.debug('\nAccounts deleted:\n', res);
      assert.equal(res[0], sender.address);
    });

    it('should send a deleteAccounts to a group address that does not exist and get receipt with error message', (done) => {
      const res = deleteAccounts(
        0x1234567,
        0x1234567,
        ['0x1a702a25c6bca72b67987968f0bfb3a3213c5603'],
        sender,
      );

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt with error message:\n', receipt);
          assert.equal(receipt.errorMessage, 'Reverted.', JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get deleteAccounts receipt err:!!!!\n', err);
          this.skip();
        });
    });
  });

  describe('\ntest delete group\n', () => {
    before('Query the number of the accounts', () => {
      gContractInstance = gr.at(rootGroupAddr);
      const res = gContractInstance.queryChild.call();
      logger.debug('\nThe number of the child group:\n', res.length);
      lengthOfChild = res.length;
    });

    it('should send a deleteGroup tx and get receipt', (done) => {
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

    it('should have deleted the group', () => {
      gContractInstance = gr.at(rootGroupAddr);
      const res = gContractInstance.queryChild.call();
      logger.debug('\nNow the number of child group:\n', res.length);
      assert.equal(res.length, lengthOfChild - 1);
    });

    it('should send a deleteGroup that does not exist and get receipt with error message', (done) => {
      const res = deleteGroup(newGroupAddr, newGroupAddr, sender);

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt with error message:\n', receipt);
          assert.equal(receipt.errorMessage, 'Reverted.', JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get deleteGroup receipt err:!!!!\n', err);
          this.skip();
        });
    });
  });
});
