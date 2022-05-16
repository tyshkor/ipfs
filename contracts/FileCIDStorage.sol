pragma solidity ^0.8.0;

contract FileCIDStorage {
    string public file_cid;

    function set(string calldata x) public {
        file_cid = x;
    }

    function get() public returns (string memory) {
        return file_cid;
    }
}