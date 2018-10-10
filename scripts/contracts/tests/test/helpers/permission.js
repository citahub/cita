const util = require('./util');
const config = require('../config');

const { genContract } = util;

const { abi, addr } = config.contract.permission;

const contract = genContract(abi, addr);

// queryInfo
const queryInfo = () => contract.methods.queryInfo().call('pending');

// inPermission
const inPermission = (cont, func) => contract.methods.inPermission(
  cont,
  func,
).call('pending');

module.exports = {
  queryInfo,
  inPermission,
};
