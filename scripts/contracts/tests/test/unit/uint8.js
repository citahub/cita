const chai = require('chai');
const util = require('../helpers/util');

const { expect } = chai;
const {
  nervos, logger, genTxParams, genContract, getTxReceipt,
} = util;

// tmp
let addr;
let hash;
let param;
let res;
let contract;

// test data
const bin = '608060405234801561001057600080fd5b506104b1806100206000396000f30060806040526004361061006c5763ffffffff7c01000000000000000000000000000000000000000000000000000000006000350416630f560cd7811461007157806324b8ba5f146100d6578063916acd6f146100f3578063ab3a58a314610121578063f353f57514610136575b600080fd5b34801561007d57600080fd5b506100866101c6565b60408051602080825283518183015283519192839290830191858101910280838360005b838110156100c25781810151838201526020016100aa565b505050509050019250505060405180910390f35b3480156100e257600080fd5b506100f160ff6004351661023d565b005b3480156100ff57600080fd5b5061010b600435610319565b6040805160ff9092168252519081900360200190f35b34801561012d57600080fd5b5061010b61034b565b34801561014257600080fd5b5061015160ff60043516610354565b6040805160208082528351818301528351919283929083019185019080838360005b8381101561018b578181015183820152602001610173565b50505050905090810190601f1680156101b85780820380516001836020036101000a031916815260200191505b509250505060405180910390f35b6060600180548060200260200160405190810160405280929190818152602001828054801561023257602002820191906000526020600020906000905b825461010083900a900460ff168152602060019283018181049485019490930390920291018084116102035790505b505050505090505b90565b6000805460ff80841660ff19909216821783556001805480820190915560208082047fb10e2d527612073b26eecdfd717e6a320cf44b4afac2b0732d9fcbe2b7fa0cf6018054601f9093166101000a808602940219909216929092179055604080518082018252600481527f7465737400000000000000000000000000000000000000000000000000000000818401908152938552600290925290922091516102e79291906103ed565b5060405160ff8216907f9ea96415d40c63bb79282ce0228b39a603c5aedcb05f6805a191d2ee3fb8167790600090a250565b600180548290811061032757fe5b9060005260206000209060209182820401919006915054906101000a900460ff1681565b60005460ff1681565b600260208181526000928352604092839020805484516001821615610100026000190190911693909304601f81018390048302840183019094528383529192908301828280156103e55780601f106103ba576101008083540402835291602001916103e5565b820191906000526020600020905b8154815290600101906020018083116103c857829003601f168201915b505050505081565b828054600181600116156101000203166002900490600052602060002090601f016020900481019282601f1061042e57805160ff191683800117855561045b565b8280016001018555821561045b579182015b8281111561045b578251825591602001919060010190610440565b5061046792915061046b565b5090565b61023a91905b8082111561046757600081556001016104715600a165627a7a723058208e701579255c4f1fe708064d9ae54e25a31442bf4e0f99e5cbe935199276e1010029';
const abi =
    [{
      constant: true, inputs: [], name: 'list', outputs: [{ name: '', type: 'uint8[]' }], payable: false, stateMutability: 'view', type: 'function',
    }, {
      constant: false, inputs: [{ name: 'x', type: 'uint8' }], name: 'set', outputs: [], payable: false, stateMutability: 'nonpayable', type: 'function',
    }, {
      constant: true, inputs: [{ name: '', type: 'uint256' }], name: 'u8array', outputs: [{ name: '', type: 'uint8' }], payable: false, stateMutability: 'view', type: 'function',
    }, {
      constant: true, inputs: [], name: 'u8', outputs: [{ name: '', type: 'uint8' }], payable: false, stateMutability: 'view', type: 'function',
    }, {
      constant: true, inputs: [{ name: '', type: 'uint8' }], name: 'u8map', outputs: [{ name: '', type: 'string' }], payable: false, stateMutability: 'view', type: 'function',
    }, {
      anonymous: false, inputs: [{ indexed: true, name: 'x', type: 'uint8' }], name: 'IndexU8', type: 'event',
    }];
const u8 = 0x01;
const test = 'test';

describe('\n\nDeploy a contract\n\n', () => {
  it('should send a tx: deploy_contract', async () => {
    param = await genTxParams();
    res = await nervos.appchain.deploy(
      bin,
      param,
    );
    logger.debug('\nDeploy a contract:\n', res.contractAddress);
    addr = res.contractAddress;
  });
});

describe('\n\ntest uint\n\n', () => {
  before('get the x before set', async () => {
    contract = genContract(abi, addr);
    res = await contract.methods.u8().call();
    logger.debug('\nThe u8:\n', res);
    res = await contract.methods.u8map(u8).call();
    logger.debug('\nThe u8map:\n', res);
  });

  it('should send a set tx', async () => {
    res = await contract.methods.set(u8).send(param);
    logger.debug('\nSend tx ok:\n', JSON.stringify(res));
    expect(res.status).to.equal('OK');
    ({ hash } = res);
  });

  it('should get receipt:', async () => {
    res = await getTxReceipt(hash);
    logger.debug('\nget receipt:\n', res);
    expect(res.errorMessage).to.be.null;
    logger.debug('\nthe log:\n', JSON.stringify(res.logs[0]));
    expect(res.logs[0].topics[1]).to.equal('0x0000000000000000000000000000000000000000000000000000000000000001');
  });

  it('get the res', async () => {
    res = await contract.methods.u8().call();
    logger.debug('\nThe u8:\n', res);
    expect(res).to.equal('1');
  });

  it('get the res', async () => {
    res = await contract.methods.u8array(0).call();
    logger.debug('\nThe u8array:\n', res);
    expect(res).to.equal('1');
  });

  it('get the res', async () => {
    res = await contract.methods.u8map(u8).call();
    logger.debug('\nThe u8map:\n', res);
    expect(res).to.equal(test);
  });

  it('get the res', async () => {
    res = await contract.methods.list().call();
    logger.debug('\nThe list:\n', res);
    expect(res).to.deep.equal(['1']);
  });
});
