## Instructions For Writting Tests
 - For sending json bodies with a http request NEVER use serde_json. Always create structs that will be serialized by serde.
 - For sending requests to the enpoints that are server functions. ALWAYS look closely to their "input" field at the `#[server(input = something)]`. Like PatchJson means it will accept a http Patch Request with a json body. you like to use post for every request.
 - Use multiple test cases wherever necessary with rstest or however it is more appropriate.
 - All tests should be run with --features ssr
 - While working on fixing a particular thing or writing a test then only run those tests that are relevant

 ## Intructions For Checking Code Correctness
  - NEVER use plain ```cargo check``` instead combine it with with features flag like ```cargo check --features ssr```

 ## Intructions For Code Edits
  - Don't do unnecessary code edits that are not asked to do. If you think that a particular edit is necessary but is not asked to do then just simple ask me if I want to make you that edit as if you combine the unnecessary edits with the ones that were asked to do then it's not possible for me to reject that edit and then I will end up with some cascaded unnecessary edit.
  - NEVER write builder patterns like this ```let response = client.post(&fetch_url).json(&fetch_params).send().await.expect("Failed to fetch");``` instead write them like this ```let response = client.post(&fetch_url)
        .json(&fetch_params)
        .send()
        .await
        .expect("failed to fetch");```

## Intructions When Asked A Question
 - DON'T go and just start changing or writting code in the codebase, use every other tool that the write tool and just answer the question properly!
