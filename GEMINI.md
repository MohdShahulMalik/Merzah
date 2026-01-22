## Instructions For Writting Tests
 - For sending json bodies with a http request NEVER use serde_json. Always create structs that will be serialized by serde.
 - For sending requests to the enpoints that are server functions. ALWAYS look closely to their "input" field at the `#[server(input = something)]`. Like PatchJson means it will accept a http Patch Request with a json body. you like to use post for every request.
 - Use multiple test cases wherever necessary with rstest or however it is more appropriate.
