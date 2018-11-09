const fs = require('fs');
const util = require('./util');
const config = require('../config');

const { genContract, genTxParams } = util;

const { superAdmin } = config;
const { admin } = config.contract;
const abi = JSON.parse(fs.readFileSync('abi/Admin.abi'));
const contract = genContract(abi, admin);

// addAdmin
const update = async (account, _sender = superAdmin) => {
  const param = await genTxParams(_sender);
  return contract.methods.update(account).send(param);
};

// isAdmin
const isAdmin = account => contract.methods.isAdmin(account).call('pending');

module.exports = {
  update,
  isAdmin,
};
