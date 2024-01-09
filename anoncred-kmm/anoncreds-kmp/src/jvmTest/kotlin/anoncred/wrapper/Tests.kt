package anoncred.wrapper

import anoncreds_wrapper.LinkSecret
import anoncreds_wrapper.PresentationRequest
import kotlin.test.Test
import kotlin.test.assertTrue

class Tests {

    @Test
    fun test() {
        val presentationJSON = """
            {
            "requested_attributes":{
            "attribute_1":{
            "name":"name",
            "restrictions":[

            ]
            }
            },
            "requested_predicates":{

            },
            "name":"presentation_request_1",
            "nonce":"1177620373658433495312997",
            "version":"0.1"
        }
        """
        val presentationRequest = PresentationRequest(presentationJSON)

        val requestedAttributes = presentationRequest.getRequestedAttributes()
        val key = requestedAttributes.keys.first()
        val requestedAttribute = requestedAttributes.get(key)!!
        val requestedAttributeJSON = requestedAttribute.getJson()

        println(requestedAttributeJSON)

        val secret = LinkSecret()
        assertTrue(secret != null)
    }
}
