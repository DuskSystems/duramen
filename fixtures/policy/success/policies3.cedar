forbid (principal, action, resource)           // Policy c2
when   { "private" in resource.tags }  // resource.tags is a set of strings
unless { resource in user.account };
