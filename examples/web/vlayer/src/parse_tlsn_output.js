import * as fs from 'fs'

const file = fs.readFileSync('tlsn.json')
const parsed = JSON.parse(file)
console.log(parsed)
console.log(parsed.substrings.inclusion_proof)
console.log(parsed.substrings.openings['38'])
console.log(new Buffer.from(parsed.substrings.openings['38'][1]["Blake3"].data).toString('utf8'))