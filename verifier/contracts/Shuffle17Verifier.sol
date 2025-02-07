// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/utils/introspection/ERC165.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

import "uzkge-contracts/contracts/shuffle/ShuffleVerifier.sol";
import "uzkge-contracts/contracts/shuffle/VerifierKey_52.sol";

import "./IVerifier.sol";

contract Shuffle17Verifier is OwnableUpgradeable, ERC165, IVerifier, ShuffleVerifier {
    uint constant DECK_ALL = 52;
    uint constant DECK_NUM = 17; // 17 in 52 cards

    constructor(address _vk1, address _vk2) ShuffleVerifier(_vk1, _vk2) {
        _verifyKey = VerifierKey_52.load;
    }

    function update(address _vk1, address _vk2) external onlyOwner {
        _extraVk1 = _vk1;
        _extraVk2 = _vk2;
    }

    function supportsInterface(bytes4 interfaceId) public view virtual override(ERC165) returns (bool) {
        return interfaceId == type(IVerifier).interfaceId || super.supportsInterface(interfaceId);
    }

    function name() external pure returns (string memory) {
        return "ace-shuffle-17";
    }

    function permission(address _sender) external view returns (bool) {
        return true;
    }

    /// show how to serialize/deseriaze the inputs params
    /// e.g. "uint256,bytes32,string,bytes32[],address[],ipfs"
    function inputs() external pure returns (string memory) {
        return "(uint256, uint256[])";
    }

    /// show how to serialize/deserialize the publics params
    /// e.g. "uint256,bytes32,string,bytes32[],address[],ipfs"
    function publics() external pure returns (string memory) {
        return "uint256[], uint256";
    }

    function types() external pure returns (string memory) {
        return "zk";
    }

    function verify(bytes calldata _publics, bytes calldata _proof) external view returns (bool) {
        (uint[] memory deck1, uint256 deck1Digest) = abi.decode(_publics, (uint[], uint256));
        (uint[] memory deck2, uint256 deck2Digest, uint[] memory pkc, bytes memory proof) = abi.decode(_proof, (uint[], uint256, uint[], bytes));

        uint256 deck1Length = DECK_NUM * 4;
        uint256 deck2Length = DECK_ALL * 4;
        uint256[] memory pi = new uint256[](deck1Length + deck2Length + 2);

        for (uint256 i = 0; i < deck1Length; i++) {
            pi[i] = deck1[i];
        }
        pi[deck1Length] = deck1Digest;
        for (uint256 i = 0; i < deck2Length; i++) {
            pi[deck1Length + 1 + i] = deck2[i];
        }
        pi[deck1Length + 1 + deck2Length] = deck2Digest;

        return this.verifyShuffle(proof, pi, pkc);
    }
}
