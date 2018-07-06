const chai = require('chai');
const util = require('../helpers/util');
const groupManagement = require('../helpers/group_management');
const config = require('../config');

const { expect } = chai;
const { abi } = config.contract.group;


// util
const {
  logger, web3, getTxReceipt, genContract,
} = util;

// group management
const {
  queryGroups, checkScope, newGroup, updateGroupName, addAccounts, deleteAccounts, deleteGroup,
} = groupManagement;

// temp
let newGroupAddr;
let lengthOfAccounts;
let lengthOfChild;
let lengthOfGroups;
let hash;
let contract;
let res;

// test data TODO as a file
const name = web3.utils.utf8ToHex('testGroup');
const newName = web3.utils.utf8ToHex('testGroupNewName');
const newName2 = web3.utils.utf8ToHex('testGroupNewName2');
const { rootGroup, testSender, testAddr } = config;
const addr = testSender.address;

describe('\n\ntest group management contract\n\n', () => {
  describe('\ntest add new group\n', () => {
    before('Query the number of the groups', async () => {
      res = await queryGroups();
      logger.debug('\nThe groups:\n', res);
      lengthOfGroups = res.length;
    });

    it('should send a tx: newGroup', async () => {
      res = await newGroup(rootGroup, name, [addr]);
      logger.debug('\nSend tx ok:\n', JSON.stringify(res));
      expect(res.status).to.equal('OK');
      ({ hash } = res);
    });

    it('should get receipt: newGroup', async () => {
      res = await getTxReceipt(hash);
      logger.debug('\nget receipt:\n', res);
      expect(res.errorMessage).to.equal(null);
      newGroupAddr = res.logs[0].address;
      logger.debug('\nnew group addr: \n', newGroupAddr);
    });

    it('should have info of new group', async () => {
      contract = await genContract(abi, newGroupAddr);
      res = await contract.methods.queryInfo().call();
      logger.debug('\nInfo:\n', res);
      expect(res[0]).to.have.string(name);
      expect(addr).to.be.oneOf(res[1]);
    });

    it('should have more groups', async () => {
      res = await queryGroups();
      logger.debug('\nThe groups:\n', res);
      expect(res).to.have.lengthOf.above(lengthOfGroups);
      expect(newGroupAddr).to.be.oneOf(res);
    });

    it('should in the scope of root', async () => {
      res = await checkScope(rootGroup, newGroupAddr);
      logger.debug('\nIs in the scope of root:\n', res);
      expect(res).to.be.true;
    });

    it('should in the scope of self', async () => {
      res = await checkScope(newGroupAddr, newGroupAddr, testSender);
      logger.debug('\nIs in the scope of self:\n', res);
      expect(res).to.be.true;
    });
  });

  describe('\ntest update group name by self\n', () => {
    it('should send a tx: updateGroupName', async () => {
      res = await updateGroupName(newGroupAddr, newGroupAddr, newName, testSender);
      logger.debug('\nSend tx ok:\n', JSON.stringify(res));
      expect(res.status).to.equal('OK');
      ({ hash } = res);
    });

    it('should get receipt: updateGroupName', async () => {
      res = await getTxReceipt(hash);
      logger.debug('\nget receipt:\n', res);
      expect(res.errorMessage).to.equal(null);
    });

    it('should have the new group name', async () => {
      res = await contract.methods.queryName().call();
      logger.debug('\nNew Group name:\n', res);
      expect(res).to.have.string(newName);
    });
  });

  describe('\ntest update group name by root\n', () => {
    it('should send a tx: updateGroupName', async () => {
      res = await updateGroupName(rootGroup, newGroupAddr, newName2);
      logger.debug('\nSend tx ok:\n', JSON.stringify(res));
      expect(res.status).to.equal('OK');
      ({ hash } = res);
    });

    it('should get receipt: updateGroupName', async () => {
      res = await getTxReceipt(hash);
      logger.debug('\nget receipt:\n', res);
      expect(res.errorMessage).to.equal(null);
    });

    it('should have the new group name', async () => {
      res = await contract.methods.queryName().call();
      logger.debug('\nNew Group name:\n', res);
      expect(res).to.have.string(newName2);
    });
  });

  describe('\ntest add accounts\n', () => {
    before('Query the number of the accounts', async () => {
      res = await contract.methods.queryAccounts().call();
      logger.debug('\nThe number of the accounts:\n', res.length);
      lengthOfAccounts = res.length;
    });

    it('should send a tx: addAccounts', async () => {
      res = await addAccounts(
        newGroupAddr,
        newGroupAddr,
        testAddr,
        testSender,
      );
      logger.debug('\nSend tx ok:\n', JSON.stringify(res));
      expect(res.status).to.equal('OK');
      ({ hash } = res);
    });

    it('should get receipt', async () => {
      res = await getTxReceipt(hash);
      logger.debug('\nget receipt:\n', res);
      expect(res.errorMessage).to.equal(null);
    });

    it('should have the added accounts', async () => {
      res = await contract.methods.queryAccounts().call();
      logger.debug('\nNew Added accounts:\n', res);
      testAddr.map(a => expect(a).to.be.oneOf(res));
    });
  });

  describe('\ntest add duplicate accounts\n', () => {
    before('Query the number of the accounts', async () => {
      res = await contract.methods.queryAccounts().call();
      logger.debug('\nThe number of the accounts:\n', res.length);
      lengthOfAccounts = res.length;
    });

    it('should send a addAccounts tx and get receipt', async () => {
      res = await addAccounts(
        newGroupAddr,
        newGroupAddr,
        testAddr,
        testSender,
      );
      logger.debug('\nSend tx ok:\n', JSON.stringify(res));
      expect(res.status).to.equal('OK');
      ({ hash } = res);
    });

    it('should get receipt', async () => {
      res = await getTxReceipt(hash);
      logger.debug('\nget receipt:\n', res);
      expect(res.errorMessage).to.equal(null);
    });

    it('should not added into the accounts', async () => {
      res = await contract.methods.queryAccounts().call();
      logger.debug('\nThe num of the account:\n', res.length);
      expect(res).to.have.lengthOf(lengthOfAccounts);
    });
  });

  describe('\ntest delete accounts\n', () => {
    it('should send a tx: deleteAccounts ', async () => {
      res = await deleteAccounts(
        newGroupAddr,
        newGroupAddr,
        testAddr,
        testSender,
      );
      logger.debug('\nSend tx ok:\n', JSON.stringify(res));
      expect(res.status).to.equal('OK');
      ({ hash } = res);
    });

    it('should get receipt', async () => {
      res = await getTxReceipt(hash);
      logger.debug('\nget receipt:\n', res);
      expect(res.errorMessage).to.equal(null);
    });

    it('should have deleted the accounts', async () => {
      res = await contract.methods.queryAccounts().call();
      logger.debug('\nAccounts deleted:\n', res);
      testAddr.map(a => expect(a).to.not.be.oneOf(res));
    });
  });

  describe('\ntest delete group\n', () => {
    before('Query the number of the accounts', async () => {
      contract = await genContract(abi, rootGroup);
      res = await contract.methods.queryChild().call();
      logger.debug('\nThe number of the child group:\n', res.length);
      lengthOfChild = res.length;
    });

    it('should send a deleteGroup tx', async () => {
      res = await deleteGroup(newGroupAddr, newGroupAddr, testSender);
      logger.debug('\nSend tx ok:\n', JSON.stringify(res));
      expect(res.status).to.equal('OK');
      ({ hash } = res);
    });

    it('should get receipt', async () => {
      res = await getTxReceipt(hash);
      logger.debug('\nget receipt:\n', res);
      expect(res.errorMessage).to.equal(null);
    });

    it('should have deleted the group', async () => {
      res = await contract.methods.queryChild().call();
      logger.debug('\nNow the number of child group:\n', res.length);
      expect(res).to.have.lengthOf.below(lengthOfChild);
    });
  });
});
