logs |
// filter for high severity logs for my app in staging or prod
where
    (
        severity_text == "WARN"
        or severity_text == "ERROR"
        or severity_number >= 13 // WARN
    )
    and contains(resource.attribute["k8s.deployment.name"], "albert-app")
    and matches(resource.attribute["k8s.cluster.name"], "(prod|staging)-.*")


