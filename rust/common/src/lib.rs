use alloy_sol_types::sol;

sol! {
  interface Simple {
      function sum(uint256 lhs, uint256 rhs) public pure returns (uint256);
  }
}
