const assert = require('assert');
const mocha = require('mocha');
const util = require('../helpers/util');
const roleManagement = require('../helpers/role_management');
const permissionManagement = require('../helpers/permission_management');
const authorization = require('../helpers/authorization');
const config = require('../config');

// util
const { web3, getTxReceipt, logger } = util;

// config
const { permissions } = config;
const { rABI } = config.contract.role;

// role
const role = web3.eth.contract(rABI);

// role management
const {
  queryPermissionsFromRoleMana, queryAccounts, queryRoles, deleteRole, clearRole,
  cancelRole, setRole, deletePermissions, addPermissions, updateRoleName, newRole,
} = roleManagement;

// authorization
const { queryPermissions } = authorization;
let roleInstance;

// permission management
const { newPermission } = permissionManagement;

const {
  describe, it, before, after,
} = mocha;

// temp
let newRoleAddr;
let newRoleAddr2;
let lengthOfPermissions;
let lengthOfRoles;
let newPermissionAddr;

// =======================

describe('\n\ntest role management contract\n\n', () => {
  describe('\ntest new role\n', () => {
    it('should send a newRole tx and get receipt', (done) => {
      const res = newRole('testNewRole', permissions);

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
          newRoleAddr = receipt.logs[0].address;
          logger.debug('\nThe new role contract address:\n', newRoleAddr);
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get newRole receipt err:!!!!\n', err);
          this.skip();
        });
    });

    it('should have info of new role', () => {
      roleInstance = role.at(newRoleAddr);
      const res = roleInstance.queryRole.call();
      logger.debug('\nInfo:\n', res);
      assert.equal(res[0].substr(0, 24), web3.toHex('testNewRole'));

      for (let i = 0; i < res[1].length; i += 1) {
        assert.equal(res[1][i], permissions[i]);
      }
    });
  });

  describe('\ntest update role name\n', () => {
    it('should send a updateRoleName tx and get receipt', (done) => {
      const res = updateRoleName(newRoleAddr, 'testNewRoleName');

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get updateRoleName receipt err:!!!!\n', err);
          this.skip();
        });
    });

    it('should have the new role name', () => {
      roleInstance = role.at(newRoleAddr);
      const res = roleInstance.queryName.call();
      logger.debug('\nNew role name:\n', res);
      assert.equal(res.substr(0, 32), web3.toHex('testNewRoleName'));
    });
  });

  describe('\ntest add permissions\n', () => {
    before('Query the number of the permission', () => {
      roleInstance = role.at(newRoleAddr);
      const res = roleInstance.queryPermissions.call();
      logger.debug('\nThe number of the permission:\n', res.length);
      lengthOfPermissions = res.length;
    });

    it('should send a newPermission tx and get receipt', (done) => {
      const name = 'testPermission';
      const res = newPermission(name, config.testAddr, config.testFunc);

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
          newPermissionAddr = receipt.logs[0].address;
          logger.debug('\nThe new permission contract address:\n', newPermissionAddr);
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get newPermission receipt err:!!!!\n', err);
          this.skip();
        });
    });

    it('should send an addPermissions tx and get receipt', (done) => {
      const res = addPermissions(
        newRoleAddr,
        [newPermissionAddr],
      );

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get addPermissions receipt err:!!!!\n', err);
          this.skip();
        });
    });

    it('should have the added permissions: role', () => {
      roleInstance = role.at(newRoleAddr);
      const res = roleInstance.queryPermissions.call();
      logger.debug('\nNew Added permissions:\n', res);
      assert.equal(res[res.length - 1], newPermissionAddr);
      assert.equal(res.length, lengthOfPermissions + 1);
    });

    it('should have the added permissions: from role_management', () => {
      const res = queryPermissionsFromRoleMana(newRoleAddr);
      logger.debug('\nNew Added permissions:\n', res);
      assert.equal(res[res.length - 1], newPermissionAddr);
      assert.equal(res.length, lengthOfPermissions + 1);
    });

    it('should send an addPermissions which role does not exist and get receipt with error message', (done) => {
      const res = addPermissions(
        0x123456,
        [newPermissionAddr],
      );

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt with error message:\n', receipt);
          assert.equal(receipt.errorMessage, 'Reverted.', JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get addPermissions receipt err:!!!!\n', err);
          this.skip();
        });
    });
  });


  describe('\ntest add duplicated permissions\n', () => {
    before('Query the number of the permission', () => {
      roleInstance = role.at(newRoleAddr);
      const res = roleInstance.queryPermissions.call();
      logger.debug('\nThe number of the permission:\n', res.length);
      lengthOfPermissions = res.length;
    });

    it('should send a addPermissions tx and get receipt', (done) => {
      const res = addPermissions(
        newRoleAddr,
        [newPermissionAddr],
      );

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get addPermissions receipt err:!!!!\n', err);
          this.skip();
        });
    });

    it('should not added into the permissions', () => {
      roleInstance = role.at(newRoleAddr);
      const res = roleInstance.queryPermissions.call();
      logger.debug('\nThe number of the permissions:\n', res.length);
      assert.equal(res.length, lengthOfPermissions);
    });
  });

  describe('\ntest delete permissions\n', () => {
    it('should send a deletePermissions tx and get receipt', (done) => {
      const res = deletePermissions(
        newRoleAddr,
        [newPermissionAddr],
      );

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get deletePermissions receipt err:!!!!\n', err);
          this.skip();
        });
    });

    it('should have deleted the permissions', () => {
      roleInstance = role.at(newRoleAddr);
      const res = roleInstance.queryPermissions.call();
      logger.debug('\nPermissions lefted:\n', res);
      for (let i = 0; i < res.length; i += 1) {
        assert.equal(res[i], permissions[i]);
      }
    });

    it('should send a deletePermissions to an address that does not exist and get receipt with error message', (done) => {
      const res = deletePermissions(
        0x123456,
        [newPermissionAddr],
      );

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt with error message:\n', receipt);
          assert.equal(receipt.errorMessage, 'Reverted.', JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get deletePermissions receipt err:!!!!\n', err);
          this.skip();
        });
    });
  });

  describe('\ntest set role\n', () => {
    it('should send a setRole tx and get receipt', (done) => {
      const res = setRole(config.testAddr[0], newRoleAddr);

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get setRole receipt err:!!!!\n', err);
          this.skip();
        });
    });

    it('should have the role of account', () => {
      const res = queryRoles(config.testAddr[0]);
      logger.debug('\nroles of testAccount:\n', res);
      const l = res.length - 1;
      assert.equal(res[l], newRoleAddr);
      const res2 = queryAccounts(newRoleAddr);
      logger.debug('\nAccount of role:\n', res2);
      assert.equal(res2, config.testAddr[0]);
    });
  });

  describe('\ntest set duplicated role\n', () => {
    before('Query the number of role', () => {
      const res = queryRoles(config.testAddr[0]);
      logger.debug('\nThe length of role:\n', res.length);
      lengthOfRoles = res.length;
    });

    it('should send a setRole tx and get receipt', (done) => {
      const res = setRole(config.testAddr[0], newRoleAddr);

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get setRole receipt err:!!!!\n', err);
          this.skip();
        });
    });

    it('should not setted into the roles', () => {
      const res = queryRoles(config.testAddr[0]);
      logger.debug('\nThe length of role:\n', res.length);
      lengthOfRoles = res.length;
      assert.equal(res.length, lengthOfRoles);
    });
  });

  describe('\ntest role permissions of account after add_permission\n', () => {
    it('should send a addPermissions tx and get receipt', (done) => {
      const res = addPermissions(
        newRoleAddr,
        ['0x00000000000000000000000000000000033241b0'],
      );

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get addPermissions receipt err:!!!!\n', err);
          this.skip();
        });
    });

    it('should have the added permissions: role', () => {
      roleInstance = role.at(newRoleAddr);
      const res = roleInstance.queryPermissions.call();
      logger.debug('\nNew Added permissions:\n', res);
      lengthOfPermissions = res.length;
      assert.equal(res[lengthOfPermissions - 1], '0x00000000000000000000000000000000033241b0');
    });

    it('should have the added permissions: auth', () => {
      const res = queryPermissions(config.testAddr[0]);
      logger.debug('\nPermissions of testAddr:\n', res);
      lengthOfPermissions = res.length;
      assert.equal(res[lengthOfPermissions - 1], '0x00000000000000000000000000000000033241b0');
    });
  });

  describe('\ntest role permissions of account after delete_permission\n', () => {
    it('should send a deletePermissions tx and get receipt', (done) => {
      const res = deletePermissions(
        newRoleAddr,
        ['0x00000000000000000000000000000000033241b0'],
      );

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get deletePermissions receipt err:!!!!\n', err);
          this.skip();
        });
    });

    it('should have deleted the test permissions: role', () => {
      const res = queryPermissions(config.testAddr[0]);
      logger.debug('\nPermissions of testAddr:\n', res);
      assert.equal(res.length, lengthOfPermissions - 1);
    });
  });

  describe('\ntest cancel role\n', () => {
    it('should send a cancelRole tx and get receipt', (done) => {
      const res = cancelRole(config.testAddr[0], newRoleAddr);

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get cancelRole receipt err:!!!!\n', err);
          this.skip();
        });
    });

    it('should not have the role of account', () => {
      const res = queryRoles(config.testAddr[0]);
      logger.debug('\nroles of testAccount:\n', res);
      assert.equal(res.length, 0);
      const res2 = queryAccounts(newRoleAddr);
      logger.debug('\nAccount of roles:\n', res2);
      assert.equal(res2.length, 0);
    });
  });

  describe('\ntest clear role\n', () => {
    it('should send a clearRole tx and get receipt', (done) => {
      const res = clearRole(config.testAddr[0]);

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get clearRole receipt err:!!!!\n', err);
          this.skip();
        });
    });

    it('should have no roles of testAccount', () => {
      const res = queryRoles(config.testAddr[0]);
      logger.debug('\nRoles of testAccount:\n', res);
      assert.equal(res.length, 0);
    });
  });

  describe('\ntest delete role\n', () => {
    it('should send a deleteRole tx and get receipt', (done) => {
      const res = deleteRole(newRoleAddr);

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get deleteRole receipt err:!!!!\n', err);
          this.skip();
        });
    });
  });

  describe('\ntest cancel role should check other roles of account\n', () => {
    it('should send a newRole tx and get receipt', (done) => {
      const res = newRole('testNewRole', permissions);

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
          newRoleAddr = receipt.logs[0].address;
          logger.debug('\nThe new role contract address:\n', newRoleAddr);
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get newRole receipt err:!!!!\n', err);
          this.skip();
        });
    });

    it('should send a newRole tx and get receipt', (done) => {
      const res = newRole('testNewRole2', permissions);

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
          newRoleAddr2 = receipt.logs[0].address;
          logger.debug('\nThe new role contract address:\n', newRoleAddr2);
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get newRole receipt err:!!!!\n', err);
          this.skip();
        });
    });

    it('should send a setRole tx and get receipt', (done) => {
      const res = setRole(config.testAddr[1], newRoleAddr);

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get setRole receipt err:!!!!\n', err);
          this.skip();
        });
    });

    it('should send a setRole tx and get receipt', (done) => {
      const res = setRole(config.testAddr[1], newRoleAddr2);

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get setRole receipt err:!!!!\n', err);
          this.skip();
        });
    });

    it('should send a deletePermissions tx and get receipt', (done) => {
      const res = deletePermissions(newRoleAddr2, permissions);

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get deletePermissions receipt err:!!!!\n', err);
          this.skip();
        });
    });

    it('should have the newRole\'s permission after delete the permissions', () => {
      const res = queryPermissions(config.testAddr[1]);
      logger.debug('\nPermissions of testAddr:\n', res);
      assert.equal(res.length, permissions.length);
      for (let i = 0; i < permissions.length; i += 1) {
        assert.equal(res[i], permissions[i]);
      }
    });

    it('should cancel newRole', (done) => {
      const res = cancelRole(config.testAddr[1], newRoleAddr2);

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get cancelRole receipt err:!!!!\n', err);
          this.skip();
        });
    });

    it('should have the newRole\'s permission after cancel the newRole2', () => {
      const res = queryPermissions(config.testAddr[1]);
      logger.debug('\nPermissions of testAddr:\n', res);
      assert.equal(res.length, permissions.length);
      for (let i = 0; i < permissions.length; i += 1) {
        assert.equal(res[i], permissions[i]);
      }
    });

    after('cancel role', (done) => {
      const res = cancelRole(config.testAddr[1], newRoleAddr);

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get cancelRole receipt err:!!!!\n', err);
          this.skip();
        });
    });
  });
});
