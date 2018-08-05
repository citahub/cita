const util = require('../helpers/util');
const roleManagement = require('../helpers/role_management');
const roleAuth = require('../helpers/role_auth');
const permissionManagement = require('../helpers/permission_management');
const authorization = require('../helpers/authorization');
const config = require('../config');
const chai = require('chai');

const { expect } = chai;

// util
const {
  web3, getTxReceipt, logger, genContract,
} = util;

const roleAbi = config.contract.role.abi;
const {
  deleteRole, clearRole,
  cancelRole, setRole, deletePermissions, addPermissions, updateRoleName, newRole,
} = roleManagement;

// authorization
const { queryPermissions } = authorization;

const { queryRoles, queryAccounts } = roleAuth;

// permission management
const { newPermission } = permissionManagement;

// temp
let newRoleAddr;
let newRoleAddr2;
let lengthOfPermissions;
let lengthOfRoles;
let newPermissionAddr;
let hash;
let roleInstance;

// test data
const name = web3.utils.utf8ToHex('testNewRole');
const newName = web3.utils.utf8ToHex('testNewRoleName');
const name2 = web3.utils.utf8ToHex('testNewRole2');
const permName = web3.utils.utf8ToHex('testPermission');
const { testAddr, testFunc, permissions } = config;
const addr = testAddr[0];
const addr2 = testAddr[1];

describe('\n\ntest role management contract\n\n', () => {
  describe('\ntest new role\n', () => {
    it('should send a tx: newRole', async () => {
      const res = await newRole(name, permissions);
      logger.debug('\nSend tx ok:\n', JSON.stringify(res));
      expect(res.status).to.equal('OK');
      ({ hash } = res);
    });

    it('should get receipt', async () => {
      const res = await getTxReceipt(hash);
      logger.debug('\nget receipt:\n', res);
      expect(res.errorMessage).to.be.null;
      newRoleAddr = res.logs[0].address;
      logger.debug('\nThe new role contract address:\n', newRoleAddr);
    });

    it('should have info of new role', async () => {
      roleInstance = genContract(roleAbi, newRoleAddr);
      const res = await roleInstance.methods.queryRole().call();
      logger.debug('\nInfo:\n', res);
      expect(res[0]).to.have.string(name);
      expect(res[1]).to.deep.equal(permissions);
    });
  });

  describe('\ntest update role name\n', () => {
    it('should send a tx: updateRoleName', async () => {
      const res = await updateRoleName(newRoleAddr, newName);
      logger.debug('\nSend tx ok:\n', JSON.stringify(res));
      expect(res.status).to.equal('OK');
      ({ hash } = res);
    });

    it('should get receipt', async () => {
      const res = await getTxReceipt(hash);
      logger.debug('\nget receipt:\n', res);
      expect(res.errorMessage).to.be.null;
    });

    it('should have the new role name', async () => {
      const res = await roleInstance.methods.queryName().call();
      logger.debug('\nNew role name:\n', res);
      expect(res).to.have.string(newName);
    });
  });

  describe('\ntest add permissions\n', () => {
    before('Query the number of the permission', async () => {
      const res = await roleInstance.methods.queryPermissions().call();
      logger.debug('\nThe number of the permission:\n', res.length);
      lengthOfPermissions = res.length;
    });

    it('should send a tx: newPermission', async () => {
      const res = await newPermission(permName, testAddr, testFunc);
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

    it('should send a tx: addPermissions', async () => {
      const res = await addPermissions(newRoleAddr, [newPermissionAddr]);
      logger.debug('\nSend tx ok:\n', JSON.stringify(res));
      expect(res.status).to.equal('OK');
      ({ hash } = res);
    });

    it('should get receipt', async () => {
      const res = await getTxReceipt(hash);
      logger.debug('\nget receipt:\n', res);
      logger.debug('\nThe topic of logs:\n', JSON.stringify(res.logs[0].topics));
      expect(res.errorMessage).to.be.null;
    });

    it('should have the added permissions: role', async () => {
      const res = await roleInstance.methods.queryPermissions().call();
      logger.debug('\nNew Added permissions:\n', res);
      expect(newPermissionAddr).to.be.oneOf(res);
      expect(res).to.have.lengthOf.above(lengthOfPermissions);
    });
  });

  describe('\ntest add duplicated permissions\n', () => {
    before('Query the number of the permission', async () => {
      const res = await roleInstance.methods.queryPermissions().call();
      logger.debug('\nThe number of the permission:\n', res.length);
      lengthOfPermissions = res.length;
    });

    it('should send a tx: addPermissions', async () => {
      const res = await addPermissions(newRoleAddr, [newPermissionAddr]);
      logger.debug('\nSend tx ok:\n', JSON.stringify(res));
      expect(res.status).to.equal('OK');
      ({ hash } = res);
    });

    it('should get receipt', async () => {
      const res = await getTxReceipt(hash);
      logger.debug('\nget receipt:\n', res);
      logger.debug('\nThe topic of logs:\n', JSON.stringify(res.logs[0].topics));
      expect(res.errorMessage).to.be.null;
    });

    it('should not added into the permissions', async () => {
      const res = await roleInstance.methods.queryPermissions().call();
      logger.debug('\nThe number of the permissions:\n', res.length);
      expect(res).to.have.lengthOf(lengthOfPermissions);
    });
  });

  describe('\ntest delete permissions\n', () => {
    it('should send a tx: deletePermissions', async () => {
      const res = await deletePermissions(newRoleAddr, [newPermissionAddr]);
      logger.debug('\nSend tx ok:\n', JSON.stringify(res));
      expect(res.status).to.equal('OK');
      ({ hash } = res);
    });

    it('should get receipt', async () => {
      const res = await getTxReceipt(hash);
      logger.debug('\nget receipt:\n', res);
      expect(res.errorMessage).to.be.null;
    });

    it('should have deleted the permissions', async () => {
      const res = await roleInstance.methods.queryPermissions().call();
      logger.debug('\nPermissions lefted:\n', res);
      expect(res).to.deep.equal(permissions);
    });
  });

  describe('\ntest set role\n', () => {
    it('should send a tx: setRole', async () => {
      const res = await setRole(addr, newRoleAddr);
      logger.debug('\nSend tx ok:\n', JSON.stringify(res));
      expect(res.status).to.equal('OK');
      ({ hash } = res);
    });

    it('should get receipt', async () => {
      const res = await getTxReceipt(hash);
      logger.debug('\nget receipt:\n', res);
      expect(res.errorMessage).to.be.null;
    });

    it('should have the role of account', async () => {
      const res = await queryRoles(addr);
      logger.debug('\nroles of testAccount:\n', res);
      expect(newRoleAddr).to.be.oneOf(res);
      const res2 = await queryAccounts(newRoleAddr);
      logger.debug('\nAccount of role:\n', res2);
      expect(addr).to.be.oneOf(res2);
      // assert.equal(res2, addr);
    });
  });

  describe('\ntest set duplicated role\n', () => {
    before('Query the number of role', async () => {
      const res = await queryRoles(addr);
      logger.debug('\nThe length of role:\n', res.length);
      lengthOfRoles = res.length;
    });

    it('should send a tx: setRole', async () => {
      const res = await setRole(addr, newRoleAddr);
      logger.debug('\nSend tx ok:\n', JSON.stringify(res));
      expect(res.status).to.equal('OK');
      ({ hash } = res);
    });

    it('should get receipt', async () => {
      const res = await getTxReceipt(hash);
      logger.debug('\nget receipt:\n', res);
      expect(res.errorMessage).to.be.null;
    });

    it('should not setted into the roles', async () => {
      const res = await queryRoles(addr);
      logger.debug('\nThe length of role:\n', res.length);
      expect(res).to.have.lengthOf(lengthOfRoles);
    });
  });

  describe('\ntest role permissions of account after add_permission\n', () => {
    it('should send a tx: addPermissions', async () => {
      const res = await addPermissions(newRoleAddr, [addr2]);
      logger.debug('\nSend tx ok:\n', JSON.stringify(res));
      expect(res.status).to.equal('OK');
      ({ hash } = res);
    });

    it('should get receipt', async () => {
      const res = await getTxReceipt(hash);
      logger.debug('\nget receipt:\n', res);
      logger.debug('\nThe topic of logs:\n', JSON.stringify(res.logs[0].topics));
      expect(res.errorMessage).to.be.null;
    });

    it('should have the added permissions: role', async () => {
      const res = await roleInstance.methods.queryPermissions().call();
      logger.debug('\nNew Added permissions:\n', res);
      expect(addr2).to.be.oneOf(res);
    });

    it('should have the added permissions: auth', async () => {
      const res = await queryPermissions(addr);
      logger.debug('\nPermissions of testAddr:\n', res);
      expect(addr2).to.be.oneOf(res);
    });
  });

  describe('\ntest role permissions of account after delete_permission\n', () => {
    it('should send a tx: deletePermissions', async () => {
      const res = await deletePermissions(newRoleAddr, [addr2]);
      logger.debug('\nSend tx ok:\n', JSON.stringify(res));
      expect(res.status).to.equal('OK');
      ({ hash } = res);
    });

    it('should get receipt', async () => {
      const res = await getTxReceipt(hash);
      logger.debug('\nget receipt:\n', res);
      logger.debug('\nThe topic of logs:\n', JSON.stringify(res.logs[0].topics));
      expect(res.errorMessage).to.be.null;
    });

    it('should have deleted the test permissions: role', async () => {
      const res = await queryPermissions(addr);
      logger.debug('\nPermissions of testAddr:\n', res);
      expect(res).to.have.lengthOf.below(lengthOfPermissions);
    });
  });

  describe('\ntest cancel role\n', () => {
    it('should send a tx: cancelRole', async () => {
      const res = await cancelRole(addr, newRoleAddr);
      logger.debug('\nSend tx ok:\n', JSON.stringify(res));
      expect(res.status).to.equal('OK');
      ({ hash } = res);
    });

    it('should get receipt', async () => {
      const res = await getTxReceipt(hash);
      logger.debug('\nget receipt:\n', res);
      expect(res.errorMessage).to.be.null;
    });

    it('should not have the role of account', async () => {
      const res = await queryRoles(addr);
      logger.debug('\nroles of testAccount:\n', res);
      expect(newRoleAddr).to.not.be.oneOf(res);
      const res2 = await queryAccounts(newRoleAddr);
      logger.debug('\nAccount of roles:\n', res2);
      expect(addr).to.not.be.oneOf(res2);
    });
  });

  describe('\ntest clear role\n', () => {
    it('should send a tx: clearRole', async () => {
      const res = await clearRole(addr);
      logger.debug('\nSend tx ok:\n', JSON.stringify(res));
      expect(res.status).to.equal('OK');
      ({ hash } = res);
    });

    it('should get receipt', async () => {
      const res = await getTxReceipt(hash);
      logger.debug('\nget receipt:\n', res);
      expect(res.errorMessage).to.be.null;
    });

    it('should have no roles of testAccount', async () => {
      const res = await queryRoles(addr);
      logger.debug('\nRoles of testAccount:\n', res);
      expect(res).to.have.length(0);
    });
  });

  describe('\ntest delete role\n', () => {
    it('should send a tx: deleteRole', async () => {
      const res = await deleteRole(newRoleAddr);
      logger.debug('\nSend tx ok:\n', JSON.stringify(res));
      expect(res.status).to.equal('OK');
      ({ hash } = res);
    });
  });

  describe('\ntest cancel role should check other roles of account\n', () => {
    it('should send a tx: newRole', async () => {
      const res = await newRole(name, permissions);
      logger.debug('\nSend tx ok:\n', JSON.stringify(res));
      expect(res.status).to.equal('OK');
      ({ hash } = res);
    });

    it('should get receipt', async () => {
      const res = await getTxReceipt(hash);
      logger.debug('\nget receipt:\n', res);
      expect(res.errorMessage).to.be.null;
      newRoleAddr = res.logs[0].address;
      logger.debug('\nThe new role contract address:\n', newRoleAddr);
    });

    it('should send a tx: newRole', async () => {
      const res = await newRole(name2, permissions);
      logger.debug('\nSend tx ok:\n', JSON.stringify(res));
      expect(res.status).to.equal('OK');
      ({ hash } = res);
    });

    it('should get receipt', async () => {
      const res = await getTxReceipt(hash);
      logger.debug('\nget receipt:\n', res);
      expect(res.errorMessage).to.be.null;
      newRoleAddr2 = res.logs[0].address;
      logger.debug('\nThe new role contract address:\n', newRoleAddr);
    });

    it('should send a tx; setRole', async () => {
      const res = await setRole(addr2, newRoleAddr);
      logger.debug('\nSend tx ok:\n', JSON.stringify(res));
      expect(res.status).to.equal('OK');
      ({ hash } = res);
    });

    it('should get receipt', async () => {
      const res = await getTxReceipt(hash);
      logger.debug('\nget receipt:\n', res);
      expect(res.errorMessage).to.be.null;
    });

    it('should send a tx; setRole', async () => {
      const res = await setRole(addr2, newRoleAddr2);
      logger.debug('\nSend tx ok:\n', JSON.stringify(res));
      expect(res.status).to.equal('OK');
      ({ hash } = res);
    });

    it('should get receipt', async () => {
      const res = await getTxReceipt(hash);
      logger.debug('\nget receipt:\n', res);
      expect(res.errorMessage).to.be.null;
    });

    it('should send a tx: deletePermissions', async () => {
      const res = await deletePermissions(newRoleAddr2, permissions);
      logger.debug('\nSend tx ok:\n', JSON.stringify(res));
      expect(res.status).to.equal('OK');
      ({ hash } = res);
    });

    it('should get receipt', async () => {
      const res = await getTxReceipt(hash);
      logger.debug('\nget receipt:\n', res);
      expect(res.errorMessage).to.be.null;
    });

    it('should have the newRole\'s permission after delete the permissions', async () => {
      const res = await queryPermissions(addr2);
      logger.debug('\nPermissions of testAddr:\n', res);
      expect(res).to.deep.equal(permissions);
    });

    it('should cancel newRole', async () => {
      const res = await cancelRole(addr2, newRoleAddr2);
      logger.debug('\nSend tx ok:\n', JSON.stringify(res));
      expect(res.status).to.equal('OK');
      ({ hash } = res);
    });

    it('should get receipt', async () => {
      const res = await getTxReceipt(hash);
      logger.debug('\nget receipt:\n', res);
      expect(res.errorMessage).to.be.null;
    });

    it('should have the newRole\'s permission after cancel the newRole2', async () => {
      const res = await queryPermissions(addr2);
      logger.debug('\nPermissions of testAddr:\n', res);
      expect(res).to.deep.equal(permissions);
    });

    after('cancel role', async () => {
      const res = await cancelRole(addr2, newRoleAddr);
      logger.debug('\nSend tx ok:\n', JSON.stringify(res));
      expect(res.status).to.equal('OK');
      const receipt = await getTxReceipt(res.hash);
      logger.debug('\nget receipt:\n', receipt);
      expect(receipt.errorMessage).to.be.null;
    });
  });
});
