# Email
## Proof of Email
Thanks to the vlayer SDK, you can prove that you have access to a certain email without revealing its contents. Interestingly, you can then take such proofs and verify them in smart contracts.

Many online services, from social media platforms to e-commerce sites, require an email address for account creation. According to recent surveys, over 80% of businesses consider email to be their primary communication channel, both internally and with customers.  

All of this means that our inboxes are full of data that can be leveraged.

We can generate proof of ownership of certain subscriptions or actions made across many popular Web2 services. We just need an email from a trusted party. Then thanks to DKIM headers (DomainKeys Identified Mail), we can ensure that the email hasn't been tampered with in transit and is from an authorized sender.

## Example

Below you can see a smart contract written in Solidity. This contract takes `Mail` structure which represents email content along with headers and some metadata parameters attached by mail servers. By inspecting and parsing email payload elements, we can determine the purpose of a given email.   

```solidity
contract RecoveryEmail is Prover  {
    function main(Mail mail, address prevOwner, address newOwner) public returns (address) {  
      string memory recoveryEmailHash = MultigWallet(WALLET_ADDR).recoveryEmail(prevOwner)

      require(
        mail.subject == ("Wallet recovery - 0x04645AD5745fA7ad975A412631123195B7f41757"), 
        "wrong subject"
      )
      require(
        keccak256(abi.encodePacked(mail.sender) == recoveryEmailHash,
        "wrong sender"
      )

      require(
        mail.body.contains(newOwner)
        "wrong newOwner addr given"
      )
      return (newOwner); 
    }
}
```

The above contract takes the email payload and checks if the sender really wanted to reset access to the wallet represented by the smart contract (`MultisigWallet`). 
If the subject, sender and body match the predefined pattern, `Prover` returns proof. 

Such proof can be sent to `MultisigWallet`. If succeed, access would be restored and new owner wallet address is assigned:

```solidity 

import { IRecoveryEmail } from "./v/RecoveryEmail.v.sol";
address PROVER_ADDR = 0xd7141F4954c0B082b184542B8b3Bd00Dc58F5E05;
bytes4 PROVER_FUNC_SELECTOR = IRecoveryEmail(PROVER_ADDR).getSelector();

contract MultisigWallet is Prover  {

    address owner;

    mapping (address => bool) public owners;
    mapping (address => string) ownerToEmailHash;

    function recovery(Proof mailProof, string calldata emailHash, address prevOwner, address newOwner) 
      public 
      onlyVerified(PROVER_ADDR, PROVER_FUNC_SELECTOR) 
      returns (address) 
    {  
      require(ownerToEmailHash[prevOwner] == emailHash, "wrong email given")

      owners[prevOwner] = false
      owners[newOwner] = true

      return (newOwner); 
    }
}
```

MultisigWallet owner just need to send email in a right format. Our modifier checks `mailProof` content using `onlyVerified(address,bytes4)` function. It makes sure that recovery email actually exists and was sent from real and trusted domain. 