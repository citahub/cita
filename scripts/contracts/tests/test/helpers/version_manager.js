const fs = require('fs');
const util = require('./util');
const config = require('../config');

const { genContract, genTxParams } = util;

const { superAdmin } = config;
const { versionManager } = config.contract;
const abi = JSON.parse(fs.readFileSync('../interaction/abi/VersionManager.abi'));
const contract = genContract(abi, versionManager);

const getVersion = () => contract.methods.getVersion().call('pending');

const setProtocolVersion = async (account, _sender = superAdmin) => {
  const param = await genTxParams(_sender);
  return contract.methods.setProtocolVersion(account).send(param);
};

const getProtocolVersion = () => contract.methods.getProtocolVersion().call('pending');

module.exports = {
  setProtocolVersion,
  getProtocolVersion,
  getVersion,
};
