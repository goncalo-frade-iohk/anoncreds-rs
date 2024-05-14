const {
    LinkSecret,
    Prover,
    Issuer,
    CredentialSchema
} = require('./pkg');

const link1 = new LinkSecret();
const link2 = Prover.createLinkSecret();

const schema1 = new CredentialSchema(
    'demo',
    '1.0',
    "did:web:xyz",
    []
)
const schema2 = Issuer.createSchema(
    "demo",
    "1.0",
    "did:web:xyz",
    []
)

const definition = Issuer.createCredentialDefinition(
    schema1.name,
    schema1,
    "did:web:xyz",
    "any"
)

debugger;