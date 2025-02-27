// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0

pragma solidity ^0.8.20;

import {IRiscZeroVerifier} from "risc0/IRiscZeroVerifier.sol";
import {ImageID} from "./ImageID.sol"; // auto-generated contract after running `cargo build`.

contract MerkleRoot {
    IRiscZeroVerifier public immutable verifier;
    bytes32 public constant imageId = ImageID.MERKLE_ID;

    bytes32 public root;

    constructor(IRiscZeroVerifier _verifier) {
        verifier = _verifier;
        root = 0;
    }

    function set(bytes32 r, bytes calldata seal) public {
        // Construct the expected journal data. Verify will fail if journal does not match.
        bytes memory journal = abi.encode(r);
        verifier.verify(seal, imageId, sha256(journal));
        root = r;
    }

    /// @notice Returns the number stored.
    function get() public view returns (bytes32) {
        return root;
    }
}
