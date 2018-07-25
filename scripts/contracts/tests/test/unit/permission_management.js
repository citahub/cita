const util = require('../helpers/util');
const permissionManagement = require('../helpers/permission_management');
const authorization = require('../helpers/authorization');
const config = require('../config');
const chai = require('chai');

const { expect } = chai;

// util
const {
  logger, web3, getTxReceipt, genContract,
} = util;

const { abi } = config.contract.permission;

// permission management
const {
  setAuthorizations, deletePermission, cancelAuthorization, setAuthorization, clearAuthorization,
  deleteResources, addResources, updatePermissionName, newPermission,
} = permissionManagement;

// authorization
const { queryAllAccounts, queryAccounts, queryPermissions } = authorization;

// temp
let newPermissionAddr;
let newPermissionAddrA;
let newPermissionAddrB;
let lengthOfAccounts;
let lengthOfResources;
let hash;
let contract;

// test data
// const name = 'testPermission';
const name = web3.utils.utf8ToHex('testPermission');
const newName = web3.utils.utf8ToHex('testPermissionNewName');
const nameA = web3.utils.utf8ToHex('testPermissionA');
const nameB = web3.utils.utf8ToHex('testPermissionB');
const { testAddr, testFunc, permissions } = config;
const cont = '0x1a702A25C6bCA72B67987968f0BfB3a3213c5603';
const func = '0xf036ed59';
const notExistAddr = testAddr[1];

describe('\n\ntest permission management contract\n\n', () => {
  describe('\ntest add permission\n', () => {
    it('should send a tx: newPermission', async () => {
      const res = await newPermission(name, testAddr, testFunc);
      logger.debug('\nSend tx ok:\n', JSON.stringify(res));
      expect(res.status).to.equal('OK');
      ({ hash } = res);
    });

    it('should get receipt', async () => {
      const res = await getTxReceipt(hash);
      logger.debug('\nget receipt:\n', res);
      expect(res.errorMessage).to.be.null;
      newPermissionAddr = res.logs[0].address;
      logger.debug('\nThe new permission contract address:\n', newPermissionAddr);
    });

    it('should have info of new permission', async () => {
      contract = genContract(abi, newPermissionAddr);
      const res = await contract.methods.queryInfo().call();
      logger.debug('\nInfo:\n', res);
      expect(res[0]).to.have.string(name);
      expect(res[1]).to.deep.equal(testAddr);
      expect(res[2]).to.deep.equal(testFunc);
    });
  });

  describe('\ntest update permission name\n', () => {
    it('should send a tx: updatePermissionName', async () => {
      const res = await updatePermissionName(newPermissionAddr, newName);
      logger.debug('\nSend tx ok:\n', JSON.stringify(res));
      expect(res.status).to.equal('OK');
      ({ hash } = res);
    });

    it('should get receipt', async () => {
      const res = await getTxReceipt(hash);
      logger.debug('\nget receipt:\n', res);
      expect(res.errorMessage).to.be.null;
    });

    it('should have the new permission name', async () => {
      const res = await contract.methods.queryName().call();
      logger.debug('\nNew permission name:\n', res);
      expect(res).to.have.string(newName);
    });
  });

  describe('\ntest add resources\n', () => {
    it('should send a tx: addResources', async () => {
      const res = await addResources(newPermissionAddr, [cont], [func]);
      logger.debug('\nSend tx ok:\n', JSON.stringify(res));
      expect(res.status).to.equal('OK');
      ({ hash } = res);
    });

    it('should get receipt', async () => {
      const res = await getTxReceipt(hash);
      logger.debug('\nget receipt:\n', res);
      expect(res.errorMessage).to.be.null;
    });

    it('should have the added resources', async () => {
      const res = await contract.methods.queryResource().call();
      logger.debug('\nNew Added resources:\n', res);
      expect(res[0].length).to.equal(res[1].length);
      expect(cont).to.be.oneOf(res[0]);
      expect(func).to.be.oneOf(res[1]);
    });
  });

  describe('\ntest add duplicate resources\n', () => {
    before('Query the number of the resource', async () => {
      const res = await contract.methods.queryResource().call();
      logger.debug('\nThe number of the resource:\n', res[0].length);
      lengthOfResources = res[0].length;
    });

    it('should send a tx: addResources', async () => {
      const res = await addResources(newPermissionAddr, [cont], [func]);
      logger.debug('\nSend tx ok:\n', JSON.stringify(res));
      expect(res.status).to.equal('OK');
      ({ hash } = res);
    });

    it('should get receipt', async () => {
      const res = await getTxReceipt(hash);
      logger.debug('\nget receipt:\n', res);
      expect(res.errorMessage).to.be.null;
    });

    it('should not added into the resources', async () => {
      const res = await contract.methods.queryResource().call();
      logger.debug('\nThe num of the resource:\n', res[0].length);
      expect(res[0]).to.have.lengthOf(lengthOfResources);
    });
  });

  describe('\ntest delete resources\n', () => {
    it('should send a tx: deleteResources', async () => {
      const res = await deleteResources(newPermissionAddr, [cont], [func]);
      logger.debug('\nSend tx ok:\n', JSON.stringify(res));
      expect(res.status).to.equal('OK');
      ({ hash } = res);
    });

    it('should get receipt', async () => {
      const res = await getTxReceipt(hash);
      logger.debug('\nget receipt:\n', res);
      expect(res.errorMessage).to.be.null;
    });

    it('should have deleted the resources', async () => {
      const res = await contract.methods.queryResource().call();
      logger.debug('\nResources lefted:\n', res);
      expect(res[0]).to.deep.equal(testAddr);
      expect(res[1]).to.deep.equal(testFunc);
    });

    it('should send a tx of deleteResources to an exist addr', async () => {
      const res = await deleteResources(notExistAddr, [cont], [func]);
      logger.debug('\nSend tx ok:\n', JSON.stringify(res));
      expect(res.status).to.equal('OK');
      ({ hash } = res);
    });

    it('should get receipt', async () => {
      const res = await getTxReceipt(hash);
      logger.debug('\nget receipt:\n', res);
      expect(res.errorMessage).to.equal('Reverted.');
    });
  });

  describe('\ntest clear authorization\n', () => {
    it('should send a tx: clearAuthorization', async () => {
      const res = await clearAuthorization(testAddr[0]);
      logger.debug('\nSend tx ok:\n', JSON.stringify(res));
      expect(res.status).to.equal('OK');
      ({ hash } = res);
    });

    it('should get receipt', async () => {
      const res = await getTxReceipt(hash);
      logger.debug('\nget receipt:\n', res);
      expect(res.errorMessage).to.be.null;
    });

    it('should have no permissions of testAccount', async () => {
      const res = await queryPermissions(testAddr[0]);
      logger.debug('\nPermissions of testAccount:\n', res);
      expect(res).to.be.empty;
    });
  });

  describe('\ntest set authorization\n', () => {
    it('should send a tx: setAuthorization', async () => {
      const res = await setAuthorization(testAddr[0], testAddr[1]);
      logger.debug('\nSend tx ok:\n', JSON.stringify(res));
      expect(res.status).to.equal('OK');
      ({ hash } = res);
    });

    it('should get receipt', async () => {
      const res = await getTxReceipt(hash);
      logger.debug('\nget receipt:\n', res);
      expect(res.errorMessage).to.be.null;
    });

    it('should have the permission of account', async () => {
      const res = await queryPermissions(testAddr[0]);
      logger.debug('\nPermissions of testAccount:\n', res);
      expect(testAddr[1]).to.be.oneOf(res);
      const res2 = await queryAccounts(testAddr[1]);
      logger.debug('\nAccount of permissions:\n', res2);
      expect(testAddr[0]).to.be.oneOf(res2);
    });

    it('should have all accounts', async () => {
      const res = await queryAllAccounts();
      logger.debug('\nAll accounts:\n', res);
      expect(testAddr[0]).to.be.oneOf(res);
    });
  });

  describe('\ntest set duplicated authorization\n', () => {
    before('Query the number of the account', async () => {
      const res = await queryAllAccounts();
      lengthOfAccounts = res.length;
    });

    it('should send a tx: setAuthorization', async () => {
      const res = await setAuthorization(testAddr[0], testAddr[1]);
      logger.debug('\nSend tx ok:\n', JSON.stringify(res));
      expect(res.status).to.equal('OK');
      ({ hash } = res);
    });

    it('should get receipt', async () => {
      const res = await getTxReceipt(hash);
      logger.debug('\nget receipt:\n', res);
      expect(res.errorMessage).to.be.null;
    });

    it('should not be setted', async () => {
      const res = await queryAllAccounts();
      logger.debug('\nAll accounts:\n', res);
      expect(res).to.have.lengthOf(lengthOfAccounts);
    });
  });

  describe('\ntest cancel authorization\n', () => {
    it('should send a tx: cancelAuthorization', async () => {
      const res = await cancelAuthorization(testAddr[0], testAddr[1]);
      logger.debug('\nSend tx ok:\n', JSON.stringify(res));
      expect(res.status).to.equal('OK');
      ({ hash } = res);
    });

    it('should get receipt', async () => {
      const res = await getTxReceipt(hash);
      logger.debug('\nget receipt:\n', res);
      expect(res.errorMessage).to.be.null;
    });

    it('should not have the permission of account', async () => {
      const res = await queryPermissions(testAddr[0]);
      logger.debug('\nPermissions of testAccount:\n', res);
      expect(res).to.be.empty;
      const res2 = await queryAccounts(testAddr[1]);
      logger.debug('\nAccount of permissions:\n', res2);
      expect(res2).to.be.empty;
    });
  });

  describe('\ntest delete built-in permission\n', () => {
    it('should send a deletePermission tx and get receipt with error message', async () => {
      const results = permissions.map(p => deletePermission(p));
      const res = await Promise.all(results);
      const receipts = res.map(h => getTxReceipt(h.hash));
      const errors = await Promise.all(receipts);
      logger.debug('\nAll the receipts:\n', errors);
      errors.map(re => expect(re.errorMessage).to.equal('Reverted.'));
    });
  });

  describe('\ntest delete permission\n', () => {
    it('should send a tx: deletePermission ', async () => {
      const res = await deletePermission(newPermissionAddr);
      logger.debug('\nSend tx ok:\n', JSON.stringify(res));
      expect(res.status).to.equal('OK');
      ({ hash } = res);
    });

    it('should get receipt', async () => {
      const res = await getTxReceipt(hash);
      logger.debug('\nget receipt:\n', res);
      expect(res.errorMessage).to.be.null;
    });
  });

  describe('\ntest delete permission: query the auth\n', () => {
    it('should send a newPermission tx and get receipt', async () => {
      const res = await newPermission(nameA, testAddr, testFunc);
      logger.debug('\nSend tx ok:\n', JSON.stringify(res));
      expect(res.status).to.equal('OK');
      ({ hash } = res);
    });

    it('should get receipt', async () => {
      const res = await getTxReceipt(hash);
      logger.debug('\nget receipt:\n', res);
      expect(res.errorMessage).to.be.null;
      newPermissionAddrA = res.logs[0].address;
      logger.debug('\nThe new permission contract address:\n', newPermissionAddr);
    });

    it('should send a newPermission tx and get receipt', async () => {
      const res = await newPermission(nameB, testAddr, testFunc);
      logger.debug('\nSend tx ok:\n', JSON.stringify(res));
      expect(res.status).to.equal('OK');
      ({ hash } = res);
    });

    it('should get receipt', async () => {
      const res = await getTxReceipt(hash);
      logger.debug('\nget receipt:\n', res);
      expect(res.errorMessage).to.be.null;
      newPermissionAddrB = res.logs[0].address;
      logger.debug('\nThe new permission contract address:\n', newPermissionAddr);
    });

    it('should send a tx: setAuthorizations', async () => {
      const res = await setAuthorizations(testAddr[0], [newPermissionAddrA, newPermissionAddrB]);
      logger.debug('\nSend tx ok:\n', JSON.stringify(res));
      expect(res.status).to.equal('OK');
      ({ hash } = res);
    });

    it('should get receipt', async () => {
      const res = await getTxReceipt(hash);
      logger.debug('\nget receipt:\n', res);
      expect(res.errorMessage).to.be.null;
    });

    it('should have the permission of account', async () => {
      const res = await queryPermissions(testAddr[0]);
      logger.debug('\nPermissions of testAccount:\n', res);
      expect(newPermissionAddrA).to.be.oneOf(res);
      expect(newPermissionAddrB).to.be.oneOf(res);
      const res1 = await queryAccounts(newPermissionAddrA);
      logger.debug('\nAccount of permissionA:\n', res1);
      expect(testAddr[0]).to.be.oneOf(res1);
      const res2 = await queryAccounts(newPermissionAddrB);
      logger.debug('\nAccount of permissionB:\n', res2);
      expect(testAddr[0]).to.be.oneOf(res2);
    });

    it('should send a tx: deletePermission ', async () => {
      const res = await deletePermission(newPermissionAddrA);
      logger.debug('\nSend tx ok:\n', JSON.stringify(res));
      expect(res.status).to.equal('OK');
      ({ hash } = res);
    });

    it('should get receipt', async () => {
      const res = await getTxReceipt(hash);
      logger.debug('\nget receipt:\n', res);
      expect(res.errorMessage).to.be.null;
    });

    it("should cancel the account's permission", async () => {
      const res = await queryPermissions(testAddr[0]);
      logger.debug('\nPermissions of testAccount:\n', res);
      expect(res).to.have.lengthOf(1);
      expect(newPermissionAddrB).to.be.oneOf(res);
    });
  });
});
