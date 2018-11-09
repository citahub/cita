const authorization = require('../helpers/authorization');
const util = require('../helpers/util');
const config = require('../config');
const chai = require('chai');

const { expect } = chai;

const {
  permissions, superAdmin, rootGroup, rootGroupPermissions,
} = config;

const {
  queryPermissions, queryAccounts, checkPermission, queryAllAccounts,
} = authorization;

const { logger } = util;

describe('test authorization contract', () => {
  it('should be the build-in authorization: superAdmin has the permission', async () => {
    const res = await queryPermissions(superAdmin.address);
    logger.debug('\nPermissions of superAdmin:\n', res);
    expect(res).to.deep.equal(permissions);
  });

  it('should be the build-in authorization: rootGroup has the permission', async () => {
    const res = await queryPermissions(rootGroup);
    logger.debug('\nPermissions of rootGroup:\n', res);
    expect(res).to.deep.equal(rootGroupPermissions);
  });

  it('should be the build-in authorization: account of the permission', async () => {
    const results = permissions.map(p => queryAccounts(p));
    const res = await Promise.all(results);
    logger.debug('\nAccounts:\n', res);
    for (let i = 0; i < res.length; i += 1) {
      if (rootGroupPermissions.indexOf(permissions[i]) === -1) {
        expect(res[i]).to.deep.equal([superAdmin.address]);
      } else {
        expect(res[i]).to.deep.equal([superAdmin.address, rootGroup]);
      }
    }
  });

  it('should have all the accounts', async () => {
    const res = await queryAllAccounts();
    logger.debug('\nAll accounts:\n', res);
    expect(superAdmin.address).to.be.oneOf(res);
  });

  it('should check permission: admin', async () => {
    const results = permissions.map(p => checkPermission(superAdmin.address, p));
    const res = await Promise.all(results);
    for (let i = 0; i < res.length; i += 1) {
      expect(res[i]).to.be.true;
    }
  });

  it('should check permission: rootGroup', async () => {
    const results = permissions.map(p => checkPermission(rootGroup, p));
    const res = await Promise.all(results);
    for (let i = 0; i < res.length; i += 1) {
      expect(res[i]).to.be.true;
    }
  });
});
