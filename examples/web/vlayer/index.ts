import fetch from 'node-fetch'

const body = JSON.stringify({
    'jsonrpc': '2.0',
    'method': 'v_call',
    'id': '1',
    'params': [
        {   "caller": "<from address>", 
            "to": "<contract address>",
            "data": "0x<abi encoded calldata>"
        },
        {"chain_id": 1, "block_no": 1},
        // {
        //     "email": "<base64? encoded raw email>",
        //     "web": "<encoded web artifacts>",
        // }
    ]
});

const response = await fetch('http://localhost:3000', {
    body,
    headers: {
      Accept: "application/json",
      "Content-Type": "application/json"
    },
    method: "POST"
  });

console.log(await response.text());