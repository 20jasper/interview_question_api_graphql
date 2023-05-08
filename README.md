# Interview Question API
Full stack interview question API 

<details>
<summary>
Table of contents
</summary>

- [Interview Question API](#interview-question-api)
  - [Documentation](#documentation)
    - [GraphQL Schema](#graphql-schema)
  - [Considerations](#considerations)
    - [Question Storage](#question-storage)
  - [Continuous Integration](#continuous-integration)
  - [Testing](#testing)
    - [Test Coverage](#test-coverage)
</details>

## Documentation
Documentation for the API can be found at https://20jasper.github.io/interview_question_api_graphql_docs/

### [GraphQL Schema](https://20jasper.github.io/interview_question_api_graphql_docs/docs/graphql)
A searchable reference for the API GraphQL Schema generated with GraphDoc

## Considerations
### Question Storage
The questions are stored in a JSON file since they will not need to change between releases, and I don't plan on adding any new questions at the moment

## Continuous Integration

<table with="100%">
<thead>
  <tr>
    <th>File</th>
    <th>Description</th>
    <th>Runs on</th>
  </tr>
</thead>
<tbody>
  <tr>
    <td><code>verify.yaml</code></td>
    <td>Check formatting, run tests and compile code</td>
    <td>Runs on pull request to main and called in <code>deploy</code></td>
  </tr>
  <tr>
    <td><code>deploy.yaml</code></td>
    <td>Calls <code>verify</code>, deploys production build to railway, then calls <code>publish-docs</code></td>
    <td>Runs on push to main</td>
  </tr>
  <tr>
    <td><code>publish-docs.yaml</code></td>
    <td>Generates <a href="#graphql-schema">GraphQL docs</a> and then pushes to <a href="https://github.com/20jasper/interview_question_api_graphql_docs">the docs repo</a></td>
    <td>Called in <code>deploy</code></td>
  </tr>
</tbody>
</table>

## Testing

Tests can be run with `cargo test`

### Test Coverage
To run test coverage, first install tarpaulin
```shell
cargo install cargo-tarpaulin
```
Then to generate HTML and XML coverage reports, run 
```shell
cargo tarpaulin
```
