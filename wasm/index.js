const {
    Prover,
    Issuer,
    CredentialSchema,
    CredentialKeyCorrectnessProof,
    CredentialOffer,
    CredentialDefinition,
    CredentialDefinitionPrivate,
    CredentialRequest,
    Presentation,
    PresentationRequest,
    CredentialRequestMetadata,
    Credential,
    LinkSecret,
    Verifier
} = require('./pkg');
const fs = require('fs');

try {
const linkSecret = Prover.createLinkSecret();
const issuerDID = "did:web:xyz";
const credentialSchema = Issuer.createSchema(
    "demo",
    "1.0",
    issuerDID,
    ["name", "age"]
)
function replacer(key, value) {
    if (value instanceof Map) {
        return Object.fromEntries(value.entries());
    } else {
        return value;
    }
}
if (!fs.existsSync("./k.test")) {
    const {
        credentialDefinitionPrivate,
        credentialDefinition,
        keyCorrectnessProof
    } = Issuer.createCredentialDefinition(
        `${issuerDID}/resource/schema`,
        CredentialSchema.from(credentialSchema),
        issuerDID,
        "any"
    )

    const dp = JSON.stringify(credentialDefinitionPrivate, replacer);
    const d = JSON.stringify(credentialDefinition, replacer);
    const k = JSON.stringify(keyCorrectnessProof, replacer);

    fs.writeFileSync("./k.test", k);
    fs.writeFileSync("./d.test", d);
    fs.writeFileSync("./dp.test", dp);
}

const credentialDefinitionPrivate = JSON.parse(fs.readFileSync("./dp.test").toString())
const credentialDefinition = JSON.parse(fs.readFileSync("./d.test").toString())
const keyCorrectnessProof = JSON.parse(fs.readFileSync("./k.test").toString())

    const offer = Issuer.createCredentialOffer(
        `${issuerDID}/resource/schema`,
        `${issuerDID}/resource/definition`,
        CredentialKeyCorrectnessProof.from(keyCorrectnessProof)
    )

    const entropy = "Some Entropy"
    const request = Prover.createCredentialRequest(
        entropy,
        CredentialDefinition.from(credentialDefinition),
        LinkSecret.fromString(linkSecret.toString()),
        "12345678654321",
        CredentialOffer.from(offer)
    )

    const issuedCredential = Issuer.createCredential(
        CredentialDefinition.from(credentialDefinition),
        CredentialDefinitionPrivate.from(credentialDefinitionPrivate),
        CredentialOffer.from(offer),
        CredentialRequest.from(request.request),
        {
            "age":20,
            "name":"test"
        }
    );

    const processedCredential = Prover.processCredential(
        Credential.from(JSON.parse(JSON.stringify(issuedCredential, replacer))),
        CredentialRequestMetadata.from(request.metadata),
        LinkSecret.fromString(linkSecret.toString()),
        CredentialDefinition.from(credentialDefinition),
    );

    const presentationRequest = Verifier.createPresentationRequest(
        "example",
        "0.1",
        {
            name1: {
                name: "name",
                restrictions: {
                    cred_def_id: `${issuerDID}/resource/definition`,
                },
            },
        },
        {
            age1: { name: "age", p_type: ">=", p_value: 18 },
        }
    )


    const schemas = new Map();
    const definitions = new Map();

    schemas.set(`${issuerDID}/resource/schema`, CredentialSchema.from(credentialSchema))
    definitions.set(`${issuerDID}/resource/definition`,CredentialDefinition.from(credentialDefinition))

    const presentation = Prover.createPresentation(
        PresentationRequest.from(JSON.parse(JSON.stringify(presentationRequest, replacer))),
        Credential.from(JSON.parse(JSON.stringify(processedCredential, replacer))),
        LinkSecret.fromString(linkSecret.toString()),
        schemas,
        definitions
    )
    const valid = Verifier.verifyPresentation(
        Presentation.from(JSON.parse(JSON.stringify(presentation, replacer))),
        PresentationRequest.from(JSON.parse(JSON.stringify(presentationRequest, replacer))),
        schemas,
        definitions
    );
    console.log("valid", valid);
    const schemass =  CredentialSchema.from({
        invalid: true
    });
} catch (err) {
    console.log(err)
    debugger;
}

