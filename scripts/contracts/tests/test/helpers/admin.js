const util = require('./util');
const config = require('../config');

const { genContract, genTxParams } = util;

const sender = config.superAdmin;
const { abi, addr } = config.contract.admin;

const contract = genContract(abi, addr);

// addAdmin
const update = async (account, _sender = sender) => {
  const param = await genTxParams(_sender);
  return contract.methods.update(account).send(param);
};

// isAdmin
const isAdmin = account => contract.methods.isAdmin(account).call();

module.exports = {
  update,
  isAdmin,
};
