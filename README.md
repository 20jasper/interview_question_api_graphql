# Interview Question API
Full stack interview question API 

<details>
<summary>
Table of contents
</summary>

- [Interview Question API](#interview-question-api)
  - [Considerations](#considerations)
    - [Question Storage](#question-storage)
  - [Continuous Integration](#continuous-integration)
</details>

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
    <td>Generates GraphQL docs and then pushes to <a href="https://github.com/20jasper/interview_question_api_graphql_docs">the docs repo</a></td>
    <td>Called in <code>deploy</code></td>
  </tr>
</tbody>
</table>
