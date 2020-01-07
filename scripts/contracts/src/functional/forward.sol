pragma solidity ^0.4.24;

contract Forward {
    // Majority number of senators
    uint majority;
    // All totol number of senators
    uint totol;
    // All senators store here
	mapping (address => bool) senators;
	//How many user vote for address's foward call with same params. 
	mapping (address => mapping(bytes => address[])) call_agreed;

	//How many agreed on add senator
	mapping (address => address[]) senator_add_agreed;

	//How many agreed on delete senator
	mapping (address => address[]) senator_del_agreed;

	//How many agreed on modify majority number
	mapping (uint => address[]) senator_majority;
	
	event Called(address indexed addr);
	event Added(address indexed addr);
	event Deled(address indexed addr);
	event MajorityChanged(uint indexed majority);
	
	constructor() {
	    senators[msg.sender] = true;
	    majority = 1;
	    totol = 1;
	}

	modifier onlySenator {
		require(senators[msg.sender] == true,"only senator");
		_;
	}
	
	function is_senator(address _user) view external returns (bool) {
	    return senators[_user];
	}
	
	function get_majority() view external returns (uint) {
	    return majority;
	}

	function get_totol_num() view external returns (uint) {
	    return totol;
	}
	
	function add_senator(address _user) onlySenator {
		if (senators[_user]) {
			return;
		}
		uint alen = senator_add_agreed[_user].length;
		for(uint i = 0;i< alen; i++) {
			if (senator_add_agreed[_user][i] == msg.sender) {
				return;
			}
		}

		if (alen +1 >= majority) {
			senators[_user] = true;
			totol+=1;
			emit Added(_user);
			delete senator_add_agreed[_user];
		} else {
			senator_add_agreed[_user].push(msg.sender);
		}
		
	}

	function del_senator(address _user) onlySenator {
		require(majority<totol && totol > 1,"Majority should <= totol num");
		if (!senators[_user]) {
			return;
		}
		
		uint alen = senator_del_agreed[_user].length;
		for(uint i = 0;i< alen; i++) {
			if (senator_del_agreed[_user][i] == msg.sender) {
				return;
			}
		}

		if (alen +1 >= majority) {
			delete senators[_user];
			totol-=1;
			emit Deled(_user);
			delete senator_del_agreed[_user];
		} else {
			senator_del_agreed[_user].push(msg.sender);
		}
	}

	function set_majority(uint _num) onlySenator {
		require(_num<=totol,"majority greater than totol num");
		if (majority == _num) {
			return;
		}
		
		uint mlen = senator_majority[_num].length;
		for(uint i = 0;i< mlen; i++) {
			if (senator_majority[_num][i] == msg.sender) {
				return;
			}
		}
		
		if (mlen + 1 >= majority) {
			majority = _num;
			emit MajorityChanged(_num);
			delete senator_majority[_num];
		} else {
			senator_majority[_num].push(msg.sender);
		}
	}
	

	function fcall(address _target,bytes _args) onlySenator external {
		uint clen = call_agreed[_target][_args].length;
		for(uint i = 0;i< clen; i++) {
			if (call_agreed[_target][_args][i] == msg.sender) {
				return;
			}
		}

		if (clen +1 < majority) {
			call_agreed[_target][_args].push(msg.sender);
			return;
		}

		delete call_agreed[_target][_args];

        emit Called(_target);
		assembly {
            let ptr := mload(0x40)
            calldatacopy(ptr, 0x24, sub(calldatasize, 0x24))
            switch call(gas, _target, 0, ptr, sub(calldatasize, 0x24), ptr, 0)
            case 0 { revert(0, 0) }
        }
	}
}

