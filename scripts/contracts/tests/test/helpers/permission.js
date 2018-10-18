const fs = require('fs');
const util = require('./util');
const config = require('../config');

const { genContract } = util;

const { permission } = config.contract;
const abi = JSON.parse(fs.readFileSync('abi/Permission.abi'));

const contract = genContract(abi, permission);

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
