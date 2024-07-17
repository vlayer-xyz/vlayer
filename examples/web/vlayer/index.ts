import * as fs from 'fs'

const tlsProof = fs.readFileSync('tlsn.json', { encoding: "utf8" })
// taken from https://notary.pse.dev/v0.1.0-alpha.5/info
const notaryPubKey = fs.readFileSync('notaryKey.pub', { encoding: "utf8" })
const parsedTlsProof = JSON.parse(tlsProof)
console.log(Buffer.from(parsedTlsProof.substrings.openings['38'][1]["Blake3"].data).toString('utf8'))


const body = JSON.stringify({
    'jsonrpc': '2.0',
    'method': 'v_call',
    'id': '1',
    'params': [
        {
            "caller": "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266",
            "to": "0x5fbdb2315678afecb367f032d93f642f64180aa3",
            "data": "0xdff2dae50000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000000000e0000000000000000000000000000000000000000000000000000000000000001168747470733a2f2f6170692e782e636f6d000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000c746573745f636f6e74656e7400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000012746573745f6e6f746172795f7075626b65790000000000000000000000000000"
        },
        { "chain_id": 1, "block_no": 1 },
        {
            "web": {
                "tls_proof": tlsProof, 
                "notary_pub_key": notaryPubKey
            }
        }
    ]
})

const response = await fetch('http://localhost:3000', {
    body,
    headers: {
        Accept: "application/json",
        "Content-Type": "application/json"
    },
    method: "POST"
})

console.log(await response.text())