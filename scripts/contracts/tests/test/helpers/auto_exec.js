const fs = require('fs');
const util = require('./util');
const config = require('../config');

const { genContract, genTxParams } = util;

const { superAdmin } = config;
const { autoExecAddr } = config.contract;
const abi = JSON.parse(fs.readFileSync('abi/AutoExec.abi'));
const contract = genContract(abi, autoExecAddr);

// register
const register = async (contAddr, _sender = superAdmin) => {
  const param = await genTxParams(_sender);
  return contract.methods.register(contAddr).send(param);
};

// autoExec
const autoExec = async (_sender = superAdmin) => {
  const param = await genTxParams(_sender);
  return contract.methods.autoExec().send(param);
};

module.exports = {
  register,
  autoExec,
};
