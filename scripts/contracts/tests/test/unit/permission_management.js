const mocha = require('mocha');
const assert = require('assert');
const util = require('../helpers/util');
const permissionManagement = require('../helpers/permission_management');
const authorization = require('../helpers/authorization');
const permission = require('../helpers/permission');
const config = require('../config');

// util
const { logger, web3, getTxReceipt } = util;

const { describe, it, before } = mocha;

// permission management
const {
  setAuthorizations, deletePermission, cancelAuthorization, setAuthorization, clearAuthorization,
  deleteResources, addResources, updatePermissionName, newPermission,
} = permissionManagement;

// authorization
const { queryAllAccounts, queryAccounts, queryPermissions } = authorization;

// perm
const { perm } = permission;
let pContractInstance;

// temp
let newPermissionAddr;
let newPermissionAddrA;
let newPermissionAddrB;
let lengthOfAccounts;
let lengthOfResources;

// =======================

describe('\n\ntest permission management contract\n\n', () => {
  describe('\ntest add permission\n', () => {
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

    it('should have info of new permission', () => {
      pContractInstance = perm.at(newPermissionAddr);
      const res = pContractInstance.queryInfo.call();
      logger.debug('\nInfo:\n', res);
      assert.equal(res[0].substr(0, 30), web3.toHex('testPermission'));

      for (let i = 0; i < res[1].length; i += 1) {
        assert.equal(res[1][i], config.testAddr[i]);
        assert.equal(res[2][i], config.testFunc[i]);
      }
    });
  });

  describe('\ntest update permission name\n', () => {
    it('should send a updatePermissionName tx and get receipt', (done) => {
      const res = updatePermissionName(newPermissionAddr, 'testPermissionNewName');

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get updatePermissionName receipt err:!!!!\n', err);
          this.skip();
        });
    });

    it('should have the new permission name', () => {
      pContractInstance = perm.at(newPermissionAddr);
      const res = pContractInstance.queryName.call();
      logger.debug('\nNew permission name:\n', res);
      assert.equal(res.substr(0, 44), web3.toHex('testPermissionNewName'));
    });
  });

  describe('\ntest add resources\n', () => {
    before('Query the number of the resource', () => {
      pContractInstance = perm.at(newPermissionAddr);
      const res = pContractInstance.queryResource.call();
      logger.debug('\nThe number of the resource:\n', res[0].length);
      lengthOfResources = res[0].length;
    });

    it('should send a addResources tx and get receipt', (done) => {
      const res = addResources(newPermissionAddr, ['0x1a702a25c6bca72b67987968f0bfb3a3213c5603'], ['0xf036ed59']);

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

    it('should have the added resources', () => {
      pContractInstance = perm.at(newPermissionAddr);
      const res = pContractInstance.queryResource.call();
      logger.debug('\nNew Added resources:\n', res);
      const l = res[0].length - 1;
      assert.equal(res[0].length, res[1].length);
      assert.equal(res[0][l], '0x1a702a25c6bca72b67987968f0bfb3a3213c5603');
      assert.equal(res[1][l], '0xf036ed59');
      assert.equal(l, lengthOfResources);
    });

    it('should send a addResources to an address that does not exist and get receipt with error message', (done) => {
      const res = addResources(0x1234567, ['0x1a702a25c6bca72b67987968f0bfb3a3213c5603'], ['0xf036ed59']);

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt with error message:\n', receipt);
          assert.equal(receipt.errorMessage, 'Reverted.', JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get addResources receipt err:!!!!\n', err);
          this.skip();
        });
    });
  });

  describe('\ntest add duplicate resources\n', () => {
    before('Query the number of the resource', () => {
      pContractInstance = perm.at(newPermissionAddr);
      const res = pContractInstance.queryResource.call();
      logger.debug('\nThe number of the resource:\n', res[0].length);
      lengthOfResources = res[0].length;
    });

    it('should send a addResources tx and get receipt', (done) => {
      const res = addResources(newPermissionAddr, ['0x1a702a25c6bca72b67987968f0bfb3a3213c5603'], ['0xf036ed59']);

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

    it('should not added into the resources', () => {
      pContractInstance = perm.at(newPermissionAddr);
      const res = pContractInstance.queryResource.call();
      logger.debug('\nThe num of the resource:\n', res[0].length);
      assert.equal(res[0].length, lengthOfResources);
    });
  });

  describe('\ntest delete resources\n', () => {
    it('should send a deleteResources tx and get receipt', (done) => {
      const res = deleteResources(newPermissionAddr, ['0x1a702a25c6bca72b67987968f0bfb3a3213c5603'], ['0xf036ed59']);

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get deleteResources receipt err:!!!!\n', err);
          this.skip();
        });
    });

    it('should have deleted the resources', () => {
      pContractInstance = perm.at(newPermissionAddr);
      const res = pContractInstance.queryResource.call();
      logger.debug('\nResources lefted:\n', res);
      for (let i = 0; i < res[1].length; i += 1) {
        assert.equal(res[0][i], config.testAddr[i]);
        assert.equal(res[1][i], config.testFunc[i]);
      }
    });

    it('should send a deleteResources to an address that does not exist and get receipt with error message', (done) => {
      const res = deleteResources(0x1234567, ['0x1a702a25c6bca72b67987968f0bfb3a3213c5603'], ['0xf036ed59']);

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt with error message:\n', receipt);
          assert.equal(receipt.errorMessage, 'Reverted.', JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get deleteResources receipt err:!!!!\n', err);
          this.skip();
        });
    });
  });

  describe('\ntest clear authorization\n', () => {
    it('should send a clearAuthorization tx and get receipt', (done) => {
      const res = clearAuthorization(config.testAddr[0]);

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get clearAuthorization receipt err:!!!!\n', err);
          this.skip();
        });
    });

    it('should have no permissions of testAccount', () => {
      const res = queryPermissions(config.testAddr[0]);
      logger.debug('\nPermissions of testAccount:\n', res);
      assert.equal(res.length, 0);
    });
  });

  describe('\ntest set authorization\n', () => {
    before('Query the number of the account', () => {
      const res = queryAllAccounts();
      lengthOfAccounts = res.length;
    });

    it('should send a setAuthorization tx and get receipt', (done) => {
      const res = setAuthorization(config.testAddr[0], config.testAddr[1]);

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

    it('should have the permission of account', () => {
      const res = queryPermissions(config.testAddr[0]);
      logger.debug('\nPermissions of testAccount:\n', res);
      const l = res.length - 1;
      assert.equal(res[l], config.testAddr[1]);
      const res2 = queryAccounts(config.testAddr[1]);
      logger.debug('\nAccount of permissions:\n', res2);
      assert.equal(res2, config.testAddr[0]);
    });

    it('should have all accounts', () => {
      const res = queryAllAccounts();
      logger.debug('\nAll accounts:\n', res);
      assert.equal(res[res.length - 1], config.testAddr[0]);
      assert.equal(res.length, lengthOfAccounts + 1);
    });
  });

  describe('\ntest set duplicated authorization\n', () => {
    before('Query the number of the account', () => {
      const res = queryAllAccounts();
      lengthOfAccounts = res.length;
    });

    it('should send a setAuthorization tx and get receipt', (done) => {
      const res = setAuthorization(config.testAddr[0], config.testAddr[1]);

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

    it('should not be setted', () => {
      const res = queryAllAccounts();
      logger.debug('\nAll accounts:\n', res);
      assert.equal(res.length, lengthOfAccounts);
    });
  });

  describe('\ntest cancel authorization\n', () => {
    it('should send a cancelAuthorization tx and get receipt', (done) => {
      const res = cancelAuthorization(config.testAddr[0], config.testAddr[1]);

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get canelAuthorization receipt err:!!!!\n', err);
          this.skip();
        });
    });

    it('should not have the permission of account', () => {
      const res = queryPermissions(config.testAddr[0]);
      logger.debug('\nPermissions of testAccount:\n', res);
      assert.equal(res.length, 0);
      const res2 = queryAccounts(config.testAddr[1]);
      logger.debug('\nAccount of permissions:\n', res2);
      assert.equal(res2.length, 0);
    });
  });

  describe('\ntest delete built-in permission\n', () => {
    it('should send a deletePermission tx and get receipt with error message', () => {
      const builtInPermissions = [
        '0xffffffffffffffffffffffffffffffffff021010',
        '0xffffffffffffffffffffffffffffffffff021011',
        '0x00000000000000000000000000000000033241B5',
        '0xffffffffffffffffffffffffffffffffff021013',
        '0xffffffffffffffffffffffffffffffffff021014',
        '0xffffffffffffffffffffffffffffffffff021015',
        '0xffffffffffffffffffffffffffffffffff021016',
        '0x00000000000000000000000000000000083241B5',
        '0x00000000000000000000000000000000093241B5',
        '0x000000000000000000000000000000000A3241b5',
        '0xffffffffffffffffffffffffffffffffff021000',
        '0xffffffffffffffffffffffffffffffffff021001',
      ];

      builtInPermissions.map(p => getTxReceipt(deletePermission(p)).then((receipt) => {
        logger.debug(`
                Send ok and get receipt:
                ${receipt}`);
        assert.equal(receipt.errorMessage, 'Reverted.', JSON.stringify(receipt.errorMessage));
      }).catch((err) => {
        logger.error(`
                !!!!Get deletePermission receipt err:!!!
                ${err}`);
        this.skip();
      }));
    });
  });

  describe('\ntest delete permission\n', () => {
    it('should send a deletePermission tx and get receipt', (done) => {
      const res = deletePermission(newPermissionAddr);

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
          assert.equal(receipt.logs[0].data.substr(26), newPermissionAddr.substr(2));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get deletePermission receipt err:!!!!\n', err);
          this.skip();
        });
    });
  });

  describe('\ntest delete permission: query the auth\n', () => {
    it('should send a newPermission tx and get receipt', (done) => {
      const res = newPermission('testPermissionA', config.testAddr, config.testFunc);

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
          newPermissionAddrA = receipt.logs[0].address;
          logger.debug('\nThe new permission contract address:\n', newPermissionAddr);
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get newPermission receipt err:!!!!\n', err);
          this.skip();
        });
    });

    it('should send a newPermission tx and get receipt', (done) => {
      const res = newPermission('testPermissionB', config.testAddr, config.testFunc);

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
          newPermissionAddrB = receipt.logs[0].address;
          logger.debug('\nThe new permission contract address:\n', newPermissionAddr);
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get newPermission receipt err:!!!!\n', err);
          this.skip();
        });
    });

    it('should send a setAuthorization tx and get receipt', (done) => {
      const res = setAuthorizations(config.testAddr[0], [newPermissionAddrA, newPermissionAddrB]);

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

    it('should have the permission of account', () => {
      const res = queryPermissions(config.testAddr[0]);
      logger.debug('\nPermissions of testAccount:\n', res);
      const l = res.length - 1;
      assert.equal(res[l], newPermissionAddrB);
      assert.equal(res[l - 1], newPermissionAddrA);
      const res1 = queryAccounts(newPermissionAddrA);
      logger.debug('\nAccount of permissionA:\n', res1);
      const res2 = queryAccounts(newPermissionAddrB);
      logger.debug('\nAccount of permissionB:\n', res2);
      assert.equal(res2, config.testAddr[0]);
    });

    it('should send a deletePermission tx and get receipt', (done) => {
      const res = deletePermission(newPermissionAddrA);

      getTxReceipt(res)
        .then((receipt) => {
          logger.debug('\nSend ok and get receipt:\n', receipt);
          assert.equal(receipt.errorMessage, null, JSON.stringify(receipt.errorMessage));
          done();
        })
        .catch((err) => {
          logger.error('\n!!!!Get deletePermission receipt err:!!!!\n', err);
          this.skip();
        });
    });

    it("should cancel the account's permission", () => {
      const res = queryPermissions(config.testAddr[0]);
      logger.debug('\nPermissions of testAccount:\n', res);
      assert.equal(res.length, 1);
      assert.equal(res[0], newPermissionAddrB);
    });
  });
});
