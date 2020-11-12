---
type: slide
slideOptions:
  transition: slide
  theme: white
---

# CI/CD
## (Continuous Integration and Deployment)

##### Andrew Thorp

##### Linux @ App
###### Nov. 11, 2020

---

### Disclaimer

Any views or opinions represented in this blog are personal and belong solely to the blog owner and do not represent those of people, institutions or organizations that the owner may or may not be associated with in professional or personal capacity, unless explicitly stated.

---

### Overview

- What is CI/CD
- Review of SDLC
- hat is CI/CD
- Code testing
- Deployment
- Automation
- Examples and live coding

---

# What is CI/CD?

---

### Software Development Life-Cycle (SDLC)
![SDLC](https://github.com/aThorp96/cicd-talk-example/raw/main/images/SDLC-small.png)

---

### What is CI/CD
 - Continuous Integration
     - Testing
     - Quality control
 - Continuous Delivery
     - Building artifacts
     - Deployment

---

### Testing

- Help maintain code correctness 
- Informs code factoring
- Run manually or automatically 

---

### Testing

- Unit tests
- Integration tests
- Quality tests (linters, style checks, etc)

---

### Testing

Example unit test: 

``` python
def my_name():
    return ["Andrew", "Thorp"]


def test_name():
    n = my_name
    assert n[0].lower() == "andrew"
    assert n[1].lower() == "thorp"
```
```shell
pytest
```

---

### Testing

Example integration test:
``` python
def test_payload_accepted_by_api():
    n = my_name()
    # possibly requires secrets
    pl = create_some_payload(name=n)
    # requires access to other services
    api = new_api_client()
    response = api.send_payload(pl)

    assert validate_response(response.payload)
```
```shell
pytest & sleep 10 # this could take a while
```

---

### Testing
- Easy to setup simply
- You should be doing it
- Can require more complex development environment
- Can be time consuming

---

### Deployment

- Tag release
- Building binaries/packages
- SFTP new artifact to a server
- Creating docker image or restarting VMs to pull in new code

---

### Deployment

Example tagging a rust release:
```shell
# Tag release
git checkout main
git tag "1.2.3"
git push --tags

# Build artifact
cargo build --release --target-dir .
scp ./releases/my-app prod-server:/app/my-app

# Deploy artifact
ssh prod-server
# ---- On server ----
service my-app restart 
exit
```

---

### Deployment

Example creating a docker release:
```shell
# Tag release
git checkout main
git tag "1.2.3"
git push --tags

# Build image
docker build . -f Dockerfile --tag docker.hub/my-app:1.2.3
docker push docker.hub

# Deploy image
ssh prod-server
docker run docker.hub/my-app:1.2.3
exit
```

---

### Deployment

- Tagging a release version
- Creating some release artifact
- Moving that artifact where it needs to go
- Switching over from the old artifact to the new one

---

# Enter Automation

---

### Automation

- Automatically test code branches build
- Verify patches meet quality standards
- Hook deployments into SDLC events

---

### Automation

Automation can take many forms

- Scripts
- Declarative 
- Often they involve YAML

---

### Automation

Various flavors of automation tooling

| Tool      | Use-Case        | Managed by  |
| ----      | --------        | ----------  |
| builds    | general-purpose | Sourcehut   |
| workflows | general-purpose | Github      |
| Jenkins   | CI/CD           | Self-hosted |
| Argo      | Kubernetes GP   | Self-hosted |
| CodeBuild | CI/CD           | AWS         |
| TravisCI  | CI/CD           | TravisCI    |

---

### Live example 

1. Test driven development of a simple code problem
2. Github workflow that
    1. Runs code tests
    2. Runs linter
    3. Checks for formatting issues
3. Github workflow that `scp`s the binary to a server when main is tagged
4. Comparable sourcehut builds

---

### Live example 

For best results, aim the firearm towards the toes, to minimize damage to foot.

---

# Useful links
- Sourcehut builds documentation: https://man.sr.ht/builds.sr.ht/
- Github workflow documentation: https://docs.github.com/en/free-pro-team@latest/actions
- "Hypothesis" (testing methodology/libraries): https://hypothesis.works/


